use crate::Release;

#[derive(Clone, Debug)]
pub struct Bisect<'slice> {
    view: &'slice [Release],
}

impl<'slice> Bisect<'slice> {
    /// Create a new binary searcher from a slice of releases
    pub fn from_slice(view: &'slice [Release]) -> Self {
        Self { view }
    }

    /// Perform a binary search on a slice of releases.
    /// The binary search is instrumented by a narrowing function `f`, which is used to determine
    /// to which side the slice should be narrowed.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_releases::Release;
    /// use rust_releases::index::{Bisect, Narrow};
    ///
    /// let items = vec![
    ///     Release::new(semver::Version::new(1, 47, 0)),
    ///     Release::new(semver::Version::new(1, 46, 0)),
    ///     Release::new(semver::Version::new(1, 45, 2)),
    ///     Release::new(semver::Version::new(1, 45, 1)),
    ///     Release::new(semver::Version::new(1, 45, 0)),
    ///     Release::new(semver::Version::new(1, 44, 0)),
    ///     Release::new(semver::Version::new(1, 43, 0)),
    ///     Release::new(semver::Version::new(1, 42, 0)),
    ///     Release::new(semver::Version::new(1, 41, 0)),
    /// ];
    ///
    /// let mut binary_search = Bisect::from_slice(items.as_slice());
    ///
    /// let output = binary_search.search(|f| if f.version().minor >= 43 { Narrow::ToRight } else { Narrow::ToLeft });
    ///
    /// assert_eq!(items[output.unwrap()].version().minor, 43)
    ///
    /// ```
    pub fn search(&mut self, f: impl Fn(&Release) -> Narrow) -> Option<usize> {
        // FIXME: replace with never type, see issue #35121 <https://github.com/rust-lang/rust/issues/35121>
        #[derive(Debug)]
        enum Never {}

        self.search_with_result_and_remainder(|release, _| Result::Ok::<Narrow, Never>(f(release)))
            .unwrap()
    }

    /// Perform a binary search on a slice of releases.
    ///
    /// The binary search is instrumented by a narrowing function `f`, which is used to determine
    /// to which side the slice should be narrowed.
    ///
    /// In `search_with_result`, the narrowing function returns a `Result<Narrow, E>` instead of
    /// simply a `Narrow`. This can be useful if your narrowing function can fail externally.
    /// For example, in [`cargo-msrv`], when bisecting Rust version for the Minumum Supported Rust Version,
    /// we run the Cargo and the Rust compiler as an external process. The external process can fail in multiple
    /// ways which are unrelated to whether the Cargo project is compatible with a certain Rust version (e.g.
    /// `cargo` is not on the PATH). In such cases, we want to notify the user of these external errors,
    /// and return an error `E`.
    ///
    /// # Error type
    ///
    /// The narrowing function `f`, requires a `Fn` closure which returns an error with type `E`,
    /// which is bound to the method. In some cases, the Rust compiler may not be able to figure out
    /// what the type of `E` must be. This can be resolved by manually annotating the result type.
    /// For example, you can return an explicit `Result::Ok::<Narrow, MyErrorType>(Narrow::ToLeft)`
    /// from within the closure where you would otherwise return a regular `Ok(NarrowToLeft)`.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_releases::Release;
    /// use rust_releases::index::{Bisect, Narrow};
    ///
    /// #[derive(Debug)]
    /// struct RequiresRust2018;
    ///
    /// let items = vec![
    ///     Release::new(semver::Version::new(1, 47, 0)),
    ///     Release::new(semver::Version::new(1, 46, 0)),
    ///     Release::new(semver::Version::new(1, 45, 2)),
    ///     Release::new(semver::Version::new(1, 45, 1)),
    ///     Release::new(semver::Version::new(1, 45, 0)),
    ///     Release::new(semver::Version::new(1, 44, 0)),
    ///     Release::new(semver::Version::new(1, 43, 0)),
    ///     Release::new(semver::Version::new(1, 42, 0)),
    ///     Release::new(semver::Version::new(1, 41, 0)),
    /// ];
    ///
    /// let mut binary_search = Bisect::from_slice(items.as_slice());
    ///
    /// let output: Result<Option<usize>, RequiresRust2018> = binary_search.search_with_result(|release| {
    ///     if release.version().minor < 31 {
    ///         return Err(RequiresRust2018);
    ///     }   
    ///
    ///     Ok(if release.version().minor >= 43 { Narrow::ToRight } else { Narrow::ToLeft })
    /// });
    ///
    /// assert_eq!(items[output.unwrap().unwrap()].version().minor, 43)
    ///
    /// ```
    ///
    /// [`cargo-msrv`]: https://github.com/foresterre/cargo-msrv
    pub fn search_with_result<E>(
        &mut self,
        f: impl Fn(&Release) -> Result<Narrow, E>,
    ) -> Result<Option<usize>, E> {
        self.search_with_result_and_remainder(|release, _remainder| f(release))
    }

    /// Perform a binary search on a slice of releases.
    ///
    /// The binary search is instrumented by a narrowing function `f`, which is used to determine
    /// to which side the slice should be narrowed. This variant of the search method also provides the
    /// amount of remaining (i.e. searchable) items as the second argument of the narowing function.
    ///
    /// See [`search_with_result`] for more.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_releases::Release;
    /// use rust_releases::index::{Bisect, Narrow};
    ///
    /// #[derive(Debug)]
    /// struct RequiresRust2018;
    ///
    /// let items = vec![
    ///     Release::new(semver::Version::new(1, 34, 0)),
    ///     Release::new(semver::Version::new(1, 33, 0)),
    ///     Release::new(semver::Version::new(1, 32, 0)),
    ///     Release::new(semver::Version::new(1, 31, 0)),
    ///     Release::new(semver::Version::new(1, 30, 0)),
    /// ];
    ///
    /// let mut binary_search = Bisect::from_slice(items.as_slice());
    ///
    /// let output: Result<Option<usize>, RequiresRust2018> = binary_search.search_with_result_and_remainder(|release, remainder| {
    ///     println!("items remaining: {}", remainder);
    ///
    ///     if release.version().minor < 31 {
    ///         return Err(RequiresRust2018);
    ///     }   
    ///
    ///     Ok(if release.version().minor >= 33 { Narrow::ToRight } else { Narrow::ToLeft })
    /// });
    ///
    /// assert_eq!(items[output.unwrap().unwrap()].version().minor, 43)
    ///
    /// ```
    ///
    /// [`search_with_result`]: crate::index::bisect::Bisect::search_with_result
    pub fn search_with_result_and_remainder<E>(
        &mut self,
        f: impl Fn(&Release, usize) -> Result<Narrow, E>,
    ) -> Result<Option<usize>, E> {
        let mut left = 0;
        let mut right = self.view.len() - 1;
        let mut result = None;

        'search: while left <= right {
            // Re-compute the mid point, where we'll divide the remaining values
            let mid_point: usize = ((left as f32 + right as f32) / 2f32).floor() as usize;
            // Add 1 since `left` and `right` are indices
            let remainder = 1 + right - left;

            // Let the narrowing function `f` compute which half of the remainder should be used
            // to continue the search
            match f(&self.view[mid_point], remainder)? {
                Narrow::ToLeft => {
                    if mid_point >= 1 {
                        right = mid_point - 1;
                    } else {
                        break 'search;
                    }
                }
                Narrow::ToRight => {
                    left = mid_point + 1;
                    result = Some(mid_point);
                }
            }
        }

        Ok(result)
    }
}

/// The `SearchResult` is used by narrowing function `f` in [`BinarySearch::search`] as the
/// determining value which side of the slice of releases, the binary search should be narrowed to.
///
/// For example, if we have a slice of releases, sorted from most recent (high) to least recent (low),
/// then, if we want to continue finding a more recent release version, we should narrow the slice
/// to the left, while if we want to find a less recent version, we should narrow the slice to the
/// right. Note that ordering matters here, as well as what the narrowing function `f` exactly computes.
///
/// [`BinarySearch::search`]: crate::index::bisect::BinarySearch::search;
#[derive(Clone, Copy, Debug)]
pub enum Narrow {
    ToLeft,
    ToRight,
}

#[cfg(test)]
mod tests {
    use super::*;

    yare::ide!();

    fn narrow_by_minor(release: &Release, at_least: u64) -> Narrow {
        if release.version().minor >= at_least {
            Narrow::ToRight
        } else {
            Narrow::ToLeft
        }
    }

    #[yare::parameterized(v50 = { 50 }, v30 = { 30 }, v10 = { 10 })]
    fn in_the_middle_tests(exp: u64) {
        let items = vec![
            Release::new(semver::Version::new(2, 100, 0)),
            Release::new(semver::Version::new(1, 50, 0)),
            Release::new(semver::Version::new(1, exp, 0)),
            Release::new(semver::Version::new(1, 10, 0)),
            Release::new(semver::Version::new(1, 9, 0)),
            Release::new(semver::Version::new(1, 8, 0)),
            Release::new(semver::Version::new(0, 0, 0)),
        ];

        let mut searcher = Bisect {
            view: items.as_ref(),
        };

        let output = searcher
            .search(|release| narrow_by_minor(release, exp))
            .unwrap();

        assert_eq!(items[output].version().minor, exp)
    }

    #[test]
    fn most_recent_release() {
        let items = vec![
            Release::new(semver::Version::new(1, 10, 0)),
            Release::new(semver::Version::new(1, 9, 0)),
            Release::new(semver::Version::new(1, 8, 0)),
            Release::new(semver::Version::new(1, 7, 0)),
            Release::new(semver::Version::new(1, 6, 0)),
            Release::new(semver::Version::new(1, 5, 0)),
        ];

        let mut searcher = Bisect {
            view: items.as_ref(),
        };

        let output = searcher
            .search(|release| narrow_by_minor(release, 10))
            .unwrap();

        assert_eq!(items[output].version().minor, 10)
    }

    #[yare::parameterized(
        on_bound = { 5 },
        outside_bound = { 4 }
    )]
    fn least_recent_release(at_least: u64) {
        let items = vec![
            Release::new(semver::Version::new(1, 10, 0)),
            Release::new(semver::Version::new(1, 9, 0)),
            Release::new(semver::Version::new(1, 8, 0)),
            Release::new(semver::Version::new(1, 7, 0)),
            Release::new(semver::Version::new(1, 6, 0)),
            Release::new(semver::Version::new(1, 5, 0)),
        ];

        let mut searcher = Bisect {
            view: items.as_ref(),
        };

        let output = searcher
            .search(|release| narrow_by_minor(release, at_least))
            .unwrap();

        assert_eq!(items[output].version().minor, 5)
    }

    #[test]
    fn not_found() {
        let items = vec![
            Release::new(semver::Version::new(1, 10, 0)),
            Release::new(semver::Version::new(1, 9, 0)),
            Release::new(semver::Version::new(1, 8, 0)),
            Release::new(semver::Version::new(1, 7, 0)),
            Release::new(semver::Version::new(1, 6, 0)),
            Release::new(semver::Version::new(1, 5, 0)),
        ];

        let mut searcher = Bisect {
            view: items.as_ref(),
        };

        let output = searcher.search(|release| narrow_by_minor(release, 11)); // not in range;

        assert!(output.is_none());
    }

    #[test]
    fn with_result_to_error() {
        let items = vec![Release::new(semver::Version::new(1, 10, 0))];

        let mut searcher = Bisect {
            view: items.as_ref(),
        };

        #[derive(Debug)]
        struct MyError;

        let output = searcher.search_with_result(|_release| Err(MyError)); // not in range;

        assert!(output.is_err());
    }

    #[test]
    fn with_result_and_remainder_to_error() {
        let items = vec![Release::new(semver::Version::new(1, 10, 0))];

        let mut searcher = Bisect {
            view: items.as_ref(),
        };

        #[derive(Debug)]
        struct MyError;

        let output = searcher.search_with_result_and_remainder(|_, _| Err(MyError));

        assert!(output.is_err());
    }
    #[test]
    fn with_result_and_remainder() {
        let items = vec![
            Release::new(semver::Version::new(1, 10, 0)),
            Release::new(semver::Version::new(1, 9, 0)),
            Release::new(semver::Version::new(1, 8, 0)),
            Release::new(semver::Version::new(1, 7, 0)),
            Release::new(semver::Version::new(1, 6, 0)),
            Release::new(semver::Version::new(1, 5, 0)),
            Release::new(semver::Version::new(1, 4, 0)),
            Release::new(semver::Version::new(1, 3, 0)),
            Release::new(semver::Version::new(1, 2, 0)),
            Release::new(semver::Version::new(1, 1, 0)),
        ];

        let mut searcher = Bisect {
            view: items.as_ref(),
        };

        #[derive(Debug)]
        struct MyError;

        use std::cell::Cell;

        let cell = Cell::new(vec![]);

        let _ = searcher.search_with_result_and_remainder(|_, rem| {
            let mut prev = cell.take();
            prev.push(rem);
            cell.set(prev);

            Result::Ok::<Narrow, ()>(Narrow::ToLeft)
        });

        let vec = cell.take();

        assert_eq!(vec, vec![10, 4, 1]);
    }
}
