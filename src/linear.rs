use rust_releases_core::Release;
use std::iter;

/// An iterator over the latest stable releases, with only the latest patch version included.
/// For example, if the ordered set of releases given consists of
/// `{"1.40.2", "1.40.1", "1.40.0", "1.39.0", "1.38.1", "1.38.0"}`, the iterator will return in
/// order `{"1.40.2", "1.39.0", "1.38.1"}`.
///
/// NB: Assumes releases are ordered from most to least recent on iterator initialization.
pub struct LatestStableReleasesIterator<I: Iterator<Item = Release>> {
    pub(crate) iter: iter::Peekable<I>,
}

impl<I: Iterator<Item = Release>> Iterator for LatestStableReleasesIterator<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.iter.next();

        #[allow(clippy::manual_inspect)]
        current.map(|it| {
            let minor = it.version().minor;

            while let Some(release) = self.iter.peek() {
                if release.version().minor == minor {
                    self.iter.next();
                } else {
                    break;
                }
            }

            it
        })
    }
}

/// Trait to transform any iterator over [`Release`] into a [`LatestStableReleasesIterator`]
///
/// [`Release`]: crate::Release
/// [`LatestStableReleasesIterator`]: core::linear::LatestStableReleasesIterator
pub trait LatestStableReleases: Iterator<Item = Release> + Sized {
    /// Consume the given iterator over [`Release`] items, into a [`LatestStableReleasesIterator`].
    fn latest_stable_releases(self) -> LatestStableReleasesIterator<Self>;
}

impl<I: Iterator<Item = Release>> LatestStableReleases for I {
    fn latest_stable_releases(self) -> LatestStableReleasesIterator<I> {
        LatestStableReleasesIterator {
            iter: self.peekable(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::linear::LatestStableReleases;
    use crate::Release;
    use rust_releases_core::semver;

    struct MyTestStruct {
        vec: Vec<Release>,
    }

    impl MyTestStruct {
        fn releases(&self) -> &[Release] {
            &self.vec
        }

        fn into_latest_patch(self) -> Self {
            Self {
                vec: self.vec.into_iter().latest_stable_releases().collect(),
            }
        }
    }

    #[test]
    fn use_case_test() {
        let releases = vec![
            Release::new_stable(semver::Version::new(1, 40, 2)),
            Release::new_stable(semver::Version::new(1, 40, 1)),
            Release::new_stable(semver::Version::new(1, 40, 0)),
            Release::new_stable(semver::Version::new(1, 39, 0)),
            Release::new_stable(semver::Version::new(1, 38, 1)),
            Release::new_stable(semver::Version::new(1, 38, 0)),
        ];

        let system_under_test = MyTestStruct { vec: releases };

        // pre check
        assert_eq!(system_under_test.releases().len(), 6);
        assert_eq!(
            system_under_test.releases()[0],
            Release::new_stable(semver::Version::new(1, 40, 2))
        );
        assert_eq!(
            system_under_test.releases()[5],
            Release::new_stable(semver::Version::new(1, 38, 0))
        );

        // perform action (moves bind, and returns Self)
        let system_under_test = system_under_test.into_latest_patch();

        assert_eq!(system_under_test.releases().len(), 3);
        assert_eq!(
            system_under_test.releases()[0],
            Release::new_stable(semver::Version::new(1, 40, 2))
        );
        assert_eq!(
            system_under_test.releases()[1],
            Release::new_stable(semver::Version::new(1, 39, 0))
        );
        assert_eq!(
            system_under_test.releases()[2],
            Release::new_stable(semver::Version::new(1, 38, 1))
        );
    }
}
