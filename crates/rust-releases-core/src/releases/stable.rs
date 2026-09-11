use crate::Stable;
use crate::releases::impls;
use rust_release::{RustRelease, date, toolchain};
use std::fmt::Debug;
use std::iter::FromIterator;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StableReleases<C = ()>(impls::ReleasesImpl<Stable, C>);

impl<C> StableReleases<C> {
    pub fn new<I>(releases: I) -> Self
    where
        I: IntoIterator<Item = RustRelease<Stable, C>>,
    {
        Self(releases.into_iter().collect())
    }

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

    /// Map the `release` and re-collect the result.
    ///
    /// # Warning: may result in unintended consequences
    ///
    /// Internally, versions are stored by `version` in a `BTreeSet`, so if you
    /// map the version specifically, you might lose releases unexpectedly.
    pub fn map<F>(self, f: F) -> StableReleases<C>
    where
        F: FnMut(RustRelease<Stable, C>) -> RustRelease<Stable, C>,
    {
        let releases = self.into_iter().map(f).collect();

        StableReleases(impls::ReleasesImpl::new(releases))
    }

    /// Map the `version` property of all releases in the set.
    ///
    /// # Warning: may result in unintended consequences
    ///
    /// Since the `version` field of a [`RustRelease`] is the only relevant property for
    /// equivalence wrt `PartialEq`, `Eq`, `PartialOrd` and `Ord`, in set of `RustRelease` elements,
    /// changing the `version` of multiple elements to the same version may have unintended
    /// consequences.
    ///
    /// For example:
    ///
    /// ```
    ///  # use rust_release::RustRelease;
    ///  # use rust_releases_core::{Stable, StableReleases};
    ///
    ///  let item0 = RustRelease::new(Stable::new(1, 2, 3), None, []);
    ///  let item1 = RustRelease::new(Stable::new(2, 3, 4), None, []);
    ///
    ///  let mut original = StableReleases::empty();
    ///  original.add(item0.clone());
    ///  original.add(item1.clone());
    ///  assert_eq!(original.len(), 2);
    ///
    ///  // Eq. is determined by version
    ///  let modified = original.map_version(|_| Stable::new(9, 9, 9));
    ///  assert_eq!(modified.len(), 1);
    /// ```
    pub fn map_version<F>(self, mut f: F) -> StableReleases<C>
    where
        F: FnMut(&RustRelease<Stable, C>) -> Stable,
    {
        let releases = self
            .into_iter()
            .map(|r| {
                let version = f(&r);
                RustRelease {
                    version,
                    release_date: r.release_date,
                    toolchains: r.toolchains,
                    context: r.context,
                }
            })
            .collect();

        StableReleases(impls::ReleasesImpl::new(releases))
    }

    /// Map the `release_date` property of all releases in the set.
    pub fn map_release_date<F>(self, mut f: F) -> StableReleases<C>
    where
        F: FnMut(&RustRelease<Stable, C>) -> Option<date::Date>,
    {
        let releases = self
            .into_iter()
            .map(|r| {
                let release_date = f(&r);
                RustRelease {
                    version: r.version,
                    release_date,
                    toolchains: r.toolchains,
                    context: r.context,
                }
            })
            .collect();

        StableReleases(impls::ReleasesImpl::new(releases))
    }

    /// Map the `toolchains` property of all releases in the set.
    pub fn map_toolchains<F>(self, mut f: F) -> StableReleases<C>
    where
        F: FnMut(&RustRelease<Stable, C>) -> Vec<toolchain::Toolchain>,
    {
        let releases = self
            .into_iter()
            .map(|r| {
                let toolchains = f(&r);
                RustRelease {
                    version: r.version,
                    release_date: r.release_date,
                    toolchains,
                    context: r.context,
                }
            })
            .collect();

        StableReleases(impls::ReleasesImpl::new(releases))
    }

    /// Map the `context` property of all releases in the set, changing the context type from `C` to `C2`.
    pub fn map_context<C2, F>(self, mut f: F) -> StableReleases<C2>
    where
        F: FnMut(&RustRelease<Stable, C>) -> C2,
    {
        let releases = self
            .into_iter()
            .map(|r| {
                let context = f(&r);
                RustRelease {
                    version: r.version,
                    release_date: r.release_date,
                    toolchains: r.toolchains,
                    context,
                }
            })
            .collect();

        StableReleases(impls::ReleasesImpl::new(releases))
    }

    /// Merge two collections, applying `merge_fn` to releases that exist in both.
    ///
    /// Releases that exist in only one collection are included unchanged.
    pub fn merge_with<F>(self, right: StableReleases<C>, merge_fn: F) -> StableReleases<C>
    where
        F: Fn(RustRelease<Stable, C>, RustRelease<Stable, C>) -> RustRelease<Stable, C>,
    {
        StableReleases(self.0.merge_with(right.0, merge_fn))
    }
}

impl<C> IntoIterator for StableReleases<C> {
    type Item = RustRelease<Stable, C>;
    type IntoIter = std::collections::btree_set::IntoIter<RustRelease<Stable, C>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<C> FromIterator<RustRelease<Stable, C>> for StableReleases<C> {
    fn from_iter<T: IntoIterator<Item = RustRelease<Stable, C>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl StableReleases<()> {
    /// Create a new, but empty, instance.
    ///
    /// NB: This function is only available for `C = ()`. Instances which use a different type `C`
    ///     can be created using `StableReleases::<C>::default()`.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_releases_core::StableReleases;
    ///
    /// let releases = StableReleases::empty();
    ///
    /// assert!(releases.is_empty());
    /// ```
    ///
    /// # See also
    ///
    /// [`StableReleases::default`]: create an empty collection, with any context type `C`.
    /// [`StableReleases::add`]: add releases to the collection.
    pub fn empty() -> Self {
        Self(impls::ReleasesImpl::default())
    }

    /// Merge two collections using default strategies (prefer left date, union toolchains).
    ///
    /// Releases that exist in only one collection are included unchanged.
    pub fn merge(self, right: StableReleases<()>) -> StableReleases<()> {
        self.merge_with(right, crate::merge::merge_default)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::merge::builder::MergeBuilder;
    use rust_release::{
        date::Date,
        toolchain::{Channel, ComponentSet, RustVersion, Target, TargetSet, Toolchain},
    };

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
            ComponentSet::default(),
            TargetSet::default(),
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

    #[test]
    fn merge_overlapping_sets() {
        let mut left = StableReleases::default();
        left.add(make_release((1, 0, 0), None));
        left.add(make_release((2, 0, 0), None));

        let mut right = StableReleases::default();
        right.add(make_release((2, 0, 0), None));
        right.add(make_release((3, 0, 0), None));

        let merged = left.merge(right);

        assert_eq!(merged.len(), 3);

        let versions: Vec<_> = merged.iter().map(|r| &r.version).collect();
        assert!(versions.contains(&&Stable::new(1, 0, 0)));
        assert!(versions.contains(&&Stable::new(2, 0, 0)));
        assert!(versions.contains(&&Stable::new(3, 0, 0)));
    }

    #[test]
    fn empty() {
        let releases = StableReleases::empty();
        assert!(releases.is_empty());
    }

    #[test]
    fn new_from_iterator() {
        let item0 = RustRelease::new(Stable::new(1, 2, 3), None, []);
        let item1 = RustRelease::new(Stable::new(2, 3, 4), None, []);

        let releases = StableReleases::new([item0.clone(), item1.clone()]);

        assert_eq!(releases.iter().collect::<Vec<_>>(), vec![&item0, &item1]);
    }

    #[test]
    fn new_from_iterator_deduplicates_by_version() {
        let item0 = RustRelease::new(Stable::new(1, 2, 3), None, []);
        let item1 = RustRelease::new(Stable::new(1, 2, 3), None, []);

        let releases = StableReleases::new([item0, item1]);

        assert_eq!(releases.len(), 1);
    }

    #[test]
    fn new_from_empty_iterator() {
        let releases = StableReleases::new::<[RustRelease<Stable>; 0]>([]);

        assert!(releases.is_empty());
    }

    #[test]
    fn into_iter() {
        let item0 = RustRelease::new(Stable::new(1, 2, 3), None, []);
        let item1 = RustRelease::new(Stable::new(2, 3, 4), None, []);

        let mut releases = StableReleases::empty();
        releases.add(item0.clone());
        releases.add(item1.clone());

        // only test that what goes in, also goes out again, without changes
        let out = releases.into_iter().collect::<Vec<_>>();

        assert_eq!(out[0], item0);
        assert_eq!(out[1], item1);
    }

    #[test]
    fn collect() {
        let item0 = RustRelease::new(Stable::new(1, 2, 3), None, []);
        let item1 = RustRelease::new(Stable::new(2, 3, 4), None, []);

        let mut original = StableReleases::empty();
        original.add(item0.clone());
        original.add(item1.clone());

        // only test that what goes in, also goes out again, without changes
        let out = original.clone().into_iter().collect::<StableReleases<()>>();

        assert_eq!(original, out);
    }

    #[test]
    fn map_version() {
        let item0 = RustRelease::new(Stable::new(1, 2, 3), None, []);
        let item1 = RustRelease::new(Stable::new(2, 3, 4), None, []);

        let mut original = StableReleases::empty();
        original.add(item0.clone());
        original.add(item1.clone());
        assert_eq!(original.len(), 2);

        // Eq. is determined by version
        let modified = original.map_version(|_s| Stable::new(9, 9, 9));
        assert_eq!(modified.len(), 1);

        let out = modified.iter().next().unwrap();
        assert_eq!(out.version().version, RustVersion::new(9, 9, 9));
    }
}
