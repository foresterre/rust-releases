use crate::releases::impls;
use crate::{Beta, PartialRustRelease};
use rust_release::RustRelease;

#[derive(Debug, Default)]
pub struct BetaReleases(impls::ReleasesImpl<Beta>);

impl BetaReleases {
    /// Merge with another set of stable releases
    pub fn merge_with<F>(self, other: BetaReleases, resolver: F) -> BetaReleases
    where
        F: Fn(Beta, PartialRustRelease, PartialRustRelease) -> RustRelease<Beta>,
    {
        BetaReleases(self.0.merge_with(other.0, resolver))
    }
}

impl BetaReleases {
    /// Add a stable release
    pub fn add(&mut self, release: RustRelease<Beta>) {
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
    pub fn iter(&self) -> impl Iterator<Item = &RustRelease<Beta>> {
        self.0.iter()
    }
}
