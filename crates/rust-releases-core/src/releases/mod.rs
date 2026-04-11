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
    }
}
