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
        let mut left = 0;
        let mut right = self.view.len() - 1;
        let mut result = None;

        'search: while left <= right {
            let mid_point: usize = ((left as f32 + right as f32) / 2f32).floor() as usize;

            match f(&self.view[mid_point]) {
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

        result
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
    ToRight,
    ToLeft,
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
}
