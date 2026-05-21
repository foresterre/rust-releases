use rust_release::RustRelease;
use std::collections::BTreeSet;

mod beta;
mod nightly;
mod stable;

pub use beta::BetaReleases;
pub use nightly::NightlyReleases;
pub use stable::StableReleases;

// shared implementation for StableReleases, BetaReleases and NightlyReleases (implementation detail)
pub(in crate::releases) mod impls {
    use super::*;
    use std::fmt::Debug;

    #[derive(Debug)]
    pub struct ReleasesImpl<V: Debug, C> {
        releases: BTreeSet<RustRelease<V, C>>,
    }

    impl<V: Debug, C> Default for ReleasesImpl<V, C> {
        fn default() -> Self {
            Self {
                releases: BTreeSet::default(),
            }
        }
    }

    impl<V: Debug, C> ReleasesImpl<V, C>
    where
        V: Ord,
    {
        /// Add a release to the collection
        pub fn add(&mut self, release: RustRelease<V, C>) {
            self.releases.insert(release);
        }

        /// Returns the amount of releases
        pub fn len(&self) -> usize {
            self.releases.len()
        }

        /// Returns true if there are no releases, and false otherwise.
        pub fn is_empty(&self) -> bool {
            self.releases.is_empty()
        }

        /// Iterate over the releases.
        pub fn iter(&self) -> impl Iterator<Item = &RustRelease<V, C>> {
            self.releases.iter()
        }

        /// Merge two sets of releases.
        ///
        /// If a release exists in both, apply the `merge_fn` to resolve the conflict. Releases that
        /// exist in only one set are included unchanged.
        pub fn merge_with<F>(self, right: Self, merge_fn: F) -> Self
        where
            V: Ord,
            F: Fn(RustRelease<V, C>, RustRelease<V, C>) -> RustRelease<V, C>,
        {
            let mut result = BTreeSet::new();
            let mut others = right.releases;

            for release in self.releases {
                if let Some(other_release) = others.take(&release) {
                    result.insert(merge_fn(release, other_release));
                } else {
                    result.insert(release);
                }
            }

            result.extend(others);
            Self { releases: result }
        }
    }

    impl<V: Debug, C> IntoIterator for ReleasesImpl<V, C>
    where
        V: Ord,
    {
        type Item = RustRelease<V, C>;
        type IntoIter = std::collections::btree_set::IntoIter<RustRelease<V, C>>;

        fn into_iter(self) -> Self::IntoIter {
            self.releases.into_iter()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::Stable;

        #[test]
        fn merge_with_is_additive() {
            let mut left = ReleasesImpl::default();
            left.add(RustRelease::new(Stable::new(1, 0, 0), None, []));

            let mut right = ReleasesImpl::default();
            right.add(RustRelease::new(Stable::new(2, 0, 0), None, []));

            let merged = left.merge_with(right, |_l, _r| panic!("must not be called"));

            assert_eq!(merged.len(), 2);
        }
    }
}
