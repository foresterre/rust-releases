use crate::merge::{Merge, MergeCandidate};
use crate::releases::impls;
use crate::Stable;
use rust_release::RustRelease;

#[derive(Debug, Default)]
pub struct StableReleases<C = ()>(impls::ReleasesImpl<Stable, C>);

impl<C> StableReleases<C> {
    /// Merge with another set of stable releases
    pub fn merge_with<C2, F, C3>(self, other: StableReleases<C2>, resolver: F) -> StableReleases<C3>
    where
        F: Fn(&Stable, MergeCandidate<C>, MergeCandidate<C2>) -> Merge<C3>,
    {
        StableReleases(self.0.merge_with(other.0, resolver))
    }
}

impl<C> StableReleases<C> {
    /// Add a stable release
    pub fn add(&mut self, release: RustRelease<Stable, C>) {
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
    pub fn iter(&self) -> impl Iterator<Item = &RustRelease<Stable, C>> {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolver::{combine, dedup_toolchains};
    use rust_release::rust_toolchain::{Channel, RustVersion, Toolchain};
    use rust_release::rust_toolchain::{Date, Target};
    use rust_release::toolchain::{ReleaseToolchain, TargetTier};
    use std::collections::HashSet;

    fn make_release(v: impl Into<RustVersion>, d: Option<Date>) -> RustRelease<Stable> {
        let v = v.into();

        RustRelease {
            version: Stable::new(v.major(), v.minor(), v.patch()),
            release_date: d.clone(),
            toolchains: vec![make_toolchain(v, d)],
            context: (),
        }
    }

    fn make_toolchain(v: impl Into<RustVersion>, d: Option<Date>) -> ReleaseToolchain {
        ReleaseToolchain::new(
            Toolchain::new(
                Channel::stable(v.into()),
                d,
                Target::host(),
                HashSet::new(),
                HashSet::new(),
            ),
            TargetTier::T1,
        )
    }

    #[test]
    fn two_unique_versions() {
        let mut releases1 = StableReleases::default();
        let mut releases2 = StableReleases::default();

        releases1.add(make_release((1, 2, 3), None));
        releases2.add(make_release((4, 5, 6), None));

        let merged = releases1.merge_with(releases2, combine);
        assert_eq!(merged.len(), 2);

        let versions: Vec<_> = merged.iter().map(|r| &r.version).collect();
        assert!(versions.contains(&&Stable::new(1, 2, 3)));
        assert!(versions.contains(&&Stable::new(4, 5, 6)));

        let toolchains: Vec<_> = merged.iter().map(|r| r.toolchains.len()).collect();
        assert_eq!(toolchains, vec![1, 1]);
    }

    #[test]
    fn two_matching_versions() {
        let mut releases1 = StableReleases::default();
        let mut releases2 = StableReleases::default();

        releases1.add(make_release((1, 2, 3), None));
        releases2.add(make_release((1, 2, 3), None));

        let merged = releases1.merge_with(releases2, combine);
        assert_eq!(merged.len(), 1);

        let versions: Vec<_> = merged.iter().map(|r| &r.version).collect();
        assert_eq!(&versions[0], &&Stable::new(1, 2, 3));

        // The combine resolver doesn't filter toolchains based on uniqueness properties
        assert_eq!(merged.iter().next().unwrap().toolchains.len(), 2);
    }

    #[test]
    fn two_matching_versions_with_dedup_toolchains() {
        let mut releases1 = StableReleases::default();
        let mut releases2 = StableReleases::default();

        releases1.add(make_release((1, 2, 3), None));
        releases2.add(make_release((1, 2, 3), None));

        let merged = releases1.merge_with(releases2, dedup_toolchains);
        assert_eq!(merged.len(), 1);

        let versions: Vec<_> = merged.iter().map(|r| &r.version).collect();
        assert_eq!(&versions[0], &&Stable::new(1, 2, 3));

        // The dedup_toolchains resolver does some filtering of toolchains based on
        // uniqueness properties (via hashing)
        assert_eq!(merged.iter().next().unwrap().toolchains.len(), 1);
    }
}
