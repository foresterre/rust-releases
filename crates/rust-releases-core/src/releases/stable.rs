use crate::releases::impls;
use crate::Stable;
use rust_release::RustRelease;

#[derive(Debug, Default)]
pub struct StableReleases<C = ()>(impls::ReleasesImpl<Stable, C>);

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
    use crate::merge::builder::MergeBuilder;
    use rust_release::{
        date::Date,
        toolchain::{Channel, RustVersion, Target, Toolchain},
    };
    use std::collections::HashSet;

    fn make_release(v: impl Into<RustVersion>, d: Option<Date>) -> RustRelease<Stable> {
        let v = v.into();

        RustRelease {
            version: Stable::new(v.major(), v.minor(), v.patch()),
            release_date: d.clone(),
            toolchains: vec![make_toolchain(v, d, Target::host())],
            context: (),
        }
    }

    fn make_toolchain(v: impl Into<RustVersion>, d: Option<Date>, target: Target) -> Toolchain {
        Toolchain::new(
            Channel::stable(v.into()),
            d,
            target,
            HashSet::new(),
            HashSet::new(),
        )
    }

    #[test]
    fn two_unique_versions() {
        let mut releases = StableReleases::default();

        releases.add(make_release((1, 2, 3), None));
        releases.add(make_release((4, 5, 6), None));

        assert_eq!(releases.len(), 2);

        let versions: Vec<_> = releases.iter().map(|r| &r.version).collect();
        assert!(versions.contains(&&Stable::new(1, 2, 3)));
        assert!(versions.contains(&&Stable::new(4, 5, 6)));

        let toolchains: Vec<_> = releases.iter().map(|r| r.toolchains.len()).collect();
        assert_eq!(toolchains, vec![1, 1]);
    }

    #[test]
    fn two_matching_versions() {
        let r1 = make_release((1, 2, 3), None);
        let r2 = RustRelease {
            version: Stable::new(1, 2, 3),
            release_date: None,
            toolchains: vec![make_toolchain(
                (1u64, 2u64, 3u64),
                None,
                Target::from_target_triple_or_unknown("wasm32-unknown-unknown"),
            )],
            context: (),
        };

        let merged = MergeBuilder::new(r1, r2).finish();

        let mut releases = StableReleases::default();
        releases.add(merged);

        assert_eq!(releases.len(), 1);

        let versions: Vec<_> = releases.iter().map(|r| &r.version).collect();
        assert_eq!(versions[0], &Stable::new(1, 2, 3));

        // UnionToolchains keeps both since the toolchains have different targets
        assert_eq!(releases.iter().next().unwrap().toolchains.len(), 2);
    }

    #[test]
    fn two_matching_versions_with_dedup_toolchains() {
        let r1 = make_release((1, 2, 3), None);
        let r2 = make_release((1, 2, 3), None);

        let merged = MergeBuilder::new(r1, r2).finish();

        let mut releases = StableReleases::default();
        releases.add(merged);

        assert_eq!(releases.len(), 1);

        let versions: Vec<_> = releases.iter().map(|r| &r.version).collect();
        assert_eq!(versions[0], &Stable::new(1, 2, 3));

        // UnionToolchains deduplicates identical toolchains
        assert_eq!(releases.iter().next().unwrap().toolchains.len(), 1);
    }
}
