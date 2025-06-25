use crate::merge::{Merge, MergeCandidate};
use crate::releases::impls;
use crate::Beta;
use rust_release::RustRelease;

#[derive(Debug, Default)]
pub struct BetaReleases<C = ()>(impls::ReleasesImpl<Beta, C>);

impl<C> BetaReleases<C> {
    /// Merge with another set of stable releases
    pub fn merge_with<C2, F, C3>(self, other: BetaReleases<C2>, resolver: F) -> BetaReleases<C3>
    where
        F: Fn(&Beta, MergeCandidate<C>, MergeCandidate<C2>) -> Merge<C3>,
    {
        BetaReleases(self.0.merge_with(other.0, resolver))
    }
}

impl<C> BetaReleases<C> {
    /// Add a stable release
    pub fn add(&mut self, release: RustRelease<Beta, C>) {
        self.0.add(release);
    }

    /// Get the number of releases
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if there are no releases, and false otherwise.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Iterate over the releases
    pub fn iter(&self) -> impl Iterator<Item = &RustRelease<Beta, C>> {
        self.0.iter()
    }
}
