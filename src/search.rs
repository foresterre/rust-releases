/// With `LinearSearch`, one can iterate over a slice of `Releases` in a linear ordering.
#[derive(Clone, Debug)]
pub struct LinearSearch<'r, T> {
    // All known releases, sorted by their version from high to low
    releases: &'r [T],
    // The current release
    current: usize,
}

impl<'r, T> LinearSearch<'r, T> {
    pub fn new(releases: &'r [T]) -> Self {
        Self {
            releases,
            current: 0,
        }
    }

    pub fn next_release(&mut self) -> Option<&'r T> {
        let ret = self.releases.get(self.current);
        self.current += 1;
        ret
    }
}

/// With `Bisect`, one can perform a binary search over a slice of `Releases`.
/// This can for example be used to more quickly find a `MSRV` or the latest working `nightly` build.
#[derive(Clone, Debug)]
pub struct BisectionSearch<'r, T> {
    // All known releases, sorted by their version from high to low
    releases: &'r [T],

    // The lowest index still in search range.
    lower_bound: usize,

    // The highest index still in search range.
    upper_bound: usize,

    // The halfway point of our current search range.
    pivot: usize,

    // Whether the search is complete, and can not go further
    done: bool,
}

impl<'r, T> BisectionSearch<'r, T> {
    /// Iterate over the element by performing a binary search.
    ///
    /// # Panics
    ///
    /// Panics if the given `Releases` slice has length `0`.
    pub fn new(releases: &'r [T]) -> Self {
        let half = releases.len() / 2 - 1;

        Self {
            releases,
            lower_bound: 0,
            upper_bound: releases.len() - 1,
            pivot: half,
            done: false,
        }
    }

    pub fn search_left(&mut self) -> Option<&'r T> {
        if self.done {
            return None;
        }

        let ret = &self.releases[self.pivot];

        #[cfg(debug_assertions)]
        let min = self.lower_bound;

        if self.pivot - self.lower_bound == 1 {
            self.upper_bound = self.lower_bound;
            self.pivot = self.lower_bound;
        } else if self.upper_bound == self.pivot && self.lower_bound == self.pivot {
            self.done = true;
        } else {
            self.upper_bound = self.pivot;
            self.pivot = self.lower_bound + (self.upper_bound - self.lower_bound) / 2;
        }

        #[cfg(debug_assertions)]
        assert!(self.lower_bound >= min);

        Some(ret)
    }

    pub fn search_right(&mut self) -> Option<&'r T> {
        if self.done {
            return None;
        }

        #[cfg(debug_assertions)]
        let max = self.upper_bound;

        let ret = &self.releases[self.pivot];

        if self.upper_bound - self.pivot == 1 {
            self.lower_bound = self.upper_bound;
            self.pivot = self.upper_bound;
        } else if self.upper_bound == self.pivot && self.lower_bound == self.pivot {
            self.done = true;
        } else {
            self.lower_bound = self.pivot;
            self.pivot = (self.lower_bound + self.upper_bound) / 2;
        }

        #[cfg(debug_assertions)]
        assert!(self.upper_bound <= max);

        Some(ret)
    }

    pub fn is_done(&self) -> bool {
        self.done
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::{Document, RustChangelog};
    use crate::{Release, ReleaseIndex};

    #[test]
    fn linear_search() {
        let path = [
            env!("CARGO_MANIFEST_DIR"),
            "/resources/rust_changelog/RELEASES.md",
        ]
        .join("");
        let strategy = RustChangelog::from_document(Document::LocalPath(path.into()));
        let index = ReleaseIndex::from_source(strategy).unwrap();
        let releases = index.releases();

        let mut search = LinearSearch::new(&releases);
        let first = search.next_release().unwrap();
        let second = search.next_release().unwrap();
        let third = search.next_release().unwrap();

        assert_eq!(first, &&Release::new(semver::Version::from((1, 50, 0))));
        assert_eq!(second, &&Release::new(semver::Version::from((1, 49, 0))));
        assert_eq!(third, &&Release::new(semver::Version::from((1, 48, 0))));
    }

    #[test]
    fn bisection_search_only_left() {
        let releases = (0usize..10).into_iter().collect::<Vec<_>>();

        let mut search = BisectionSearch::new(&releases);

        let check = search.search_left();
        assert_eq!(*check.unwrap(), 4);

        let check = search.search_left();
        assert_eq!(*check.unwrap(), 2);

        let check = search.search_left();
        assert_eq!(*check.unwrap(), 1);

        let check = search.search_left();
        assert_eq!(*check.unwrap(), 0);

        assert!(search.search_left().is_none());
    }

    #[test]
    fn bisection_search_only_right() {
        let releases = (0usize..10).into_iter().collect::<Vec<_>>();

        let mut search = BisectionSearch::new(&releases);

        let check = search.search_right();
        assert_eq!(*check.unwrap(), 4);

        let check = search.search_right();
        assert_eq!(*check.unwrap(), 6);

        let check = search.search_right();
        assert_eq!(*check.unwrap(), 7);

        let check = search.search_right();
        assert_eq!(*check.unwrap(), 8);

        let check = search.search_right();
        assert_eq!(*check.unwrap(), 9);

        assert!(search.search_right().is_none());
    }

    #[test]
    fn bisection_search_zig_zag() {
        println!();

        let releases = (0usize..100).into_iter().collect::<Vec<_>>();

        let mut search = BisectionSearch::new(&releases);

        let check = search.search_right();
        assert_eq!(*check.unwrap(), 49);

        let check = search.search_left();
        assert_eq!(*check.unwrap(), 74);

        let check = search.search_right();
        assert_eq!(*check.unwrap(), 61);

        let check = search.search_left();
        assert_eq!(*check.unwrap(), 67);

        let check = search.search_right();
        assert_eq!(*check.unwrap(), 64);

        let check = search.search_left();
        assert_eq!(*check.unwrap(), 65);

        let check = search.search_right();
        assert_eq!(*check.unwrap(), 64);

        assert!(search.clone().search_right().is_none());
        assert!(search.search_left().is_none());
    }
}
