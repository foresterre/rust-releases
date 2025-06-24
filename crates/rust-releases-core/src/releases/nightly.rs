use crate::merge::{Merge, MergeCandidate};
use crate::releases::impls;
use crate::Nightly;
use rust_release::RustRelease;

#[derive(Default)]
pub struct NightlyReleases<C = ()>(impls::ReleasesImpl<Nightly, C>);

impl<C> NightlyReleases<C> {
    /// Merge with another set of stable releases
    pub fn merge_with<C2, F, C3>(
        self,
        other: NightlyReleases<C2>,
        resolver: F,
    ) -> NightlyReleases<C3>
    where
        F: Fn(&Nightly, MergeCandidate<C>, MergeCandidate<C2>) -> Merge<C3>,
    {
        NightlyReleases(self.0.merge_with(other.0, resolver))
    }
}

impl<C> NightlyReleases<C> {
    /// Add a stable release
    pub fn add(&mut self, release: RustRelease<Nightly, C>) {
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
    pub fn iter(&self) -> impl Iterator<Item = &RustRelease<Nightly, C>> {
        self.0.iter()
    }
}
