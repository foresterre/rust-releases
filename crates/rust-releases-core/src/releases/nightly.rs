use crate::merge::{Merge, PartialRustRelease};
use crate::releases::impls;
use crate::Nightly;
use rust_release::RustRelease;

#[derive(Debug, Default)]
pub struct NightlyReleases(impls::ReleasesImpl<Nightly>);

impl Merge for NightlyReleases {
    type Channel = Nightly;

    /// Merge with another set of stable releases
    fn merge_with<F>(self, other: NightlyReleases, resolver: F) -> NightlyReleases
    where
        F: Fn(Nightly, PartialRustRelease, PartialRustRelease) -> RustRelease<Nightly>,
    {
        NightlyReleases(self.0.merge_with(other.0, resolver))
    }
}

impl NightlyReleases {
    /// Add a stable release
    pub fn add(&mut self, release: RustRelease<Nightly>) {
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
    pub fn iter(&self) -> impl Iterator<Item = &RustRelease<Nightly>> {
        self.0.iter()
    }
}
