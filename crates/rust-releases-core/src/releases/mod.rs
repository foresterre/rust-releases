use crate::PartialRustRelease;
use rust_release::RustRelease;
use std::collections::{BTreeMap, BTreeSet};

mod beta;
mod nightly;
mod stable;

pub use beta::BetaReleases;
pub use nightly::NightlyReleases;
pub use stable::StableReleases;

// shared implementation for StableReleases, BetaReleases and NightlyReleases (implementation detail)
pub(in crate::releases) mod impls {
    use super::*;

    #[derive(Debug)]
    pub struct ReleasesImpl<V> {
        releases: BTreeSet<RustRelease<V>>,
    }

    impl<V> Default for ReleasesImpl<V> {
        fn default() -> Self {
            Self {
                releases: BTreeSet::default(),
            }
        }
    }

    impl<V> ReleasesImpl<V>
    where
        V: Clone + Ord,
    {
        /// Merges this releases collection with another using a custom merge function.
        ///
        /// When releases with the same version exist in both collections, the merge_fn
        /// is called to combine them. Releases that exist in only one collection are
        /// kept as-is.
        pub fn merge_with<F>(self, rhs: Self, merge_fn: F) -> Self
        where
            V: Ord + Clone,
            F: Fn(RustRelease<V>, RustRelease<V>) -> RustRelease<V>,
        {
            let mut result = BTreeSet::new();
            let mut others = rhs.releases;

            for release in self.releases {
                // Check if a matching release exists in other
                if let Some(other_release) = others.take(&release) {
                    // Both have this version - merge them
                    result.insert(merge_fn(release, other_release));
                } else {
                    // Only in self
                    result.insert(release);
                }
            }

            // Add remaining releases from other that weren't matched
            result.extend(others);

            ReleasesImpl { releases: result }
        }
    }

    impl<V> ReleasesImpl<V>
    where
        V: Ord,
    {
        /// Add a release to the collection
        pub fn add(&mut self, release: RustRelease<V>) {
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
        pub fn iter(&self) -> impl Iterator<Item = &RustRelease<V>> {
            self.releases.iter()
        }
    }
}
