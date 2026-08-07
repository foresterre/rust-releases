use crate::Nightly;
use crate::releases::impls;
use rust_release::{RustRelease, date, toolchain};
use std::iter::FromIterator;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct NightlyReleases<C = ()>(impls::ReleasesImpl<Nightly, C>);

impl<C> NightlyReleases<C> {
    pub fn new<I>(releases: I) -> Self
    where
        I: IntoIterator<Item = RustRelease<Nightly, C>>,
    {
        Self(releases.into_iter().collect())
    }

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

    /// Map the `release` and re-collect the result.
    ///
    /// # Warning: may result in unintended consequences
    ///
    /// Internally, versions are stored by `version` in a `BTreeSet`, so if you
    /// map the version specifically, you might lose releases unexpectedly.
    pub fn map<F>(self, f: F) -> NightlyReleases<C>
    where
        F: FnMut(RustRelease<Nightly, C>) -> RustRelease<Nightly, C>,
    {
        let releases = self.into_iter().map(f).collect();

        NightlyReleases(impls::ReleasesImpl::new(releases))
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
    ///  # use rust_releases_core::{Nightly, NightlyReleases};
    ///
    ///  let item0 = RustRelease::new(Nightly::new(2024, 1, 2), None, []);
    ///  let item1 = RustRelease::new(Nightly::new(2024, 2, 3), None, []);
    ///
    ///  let mut original = NightlyReleases::empty();
    ///  original.add(item0.clone());
    ///  original.add(item1.clone());
    ///  assert_eq!(original.len(), 2);
    ///
    ///  // Eq. is determined by version
    ///  let modified = original.map_version(|_| Nightly::new(2024, 9, 9));
    ///  assert_eq!(modified.len(), 1);
    /// ```
    pub fn map_version<F>(self, mut f: F) -> NightlyReleases<C>
    where
        F: FnMut(&RustRelease<Nightly, C>) -> Nightly,
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

        NightlyReleases(impls::ReleasesImpl::new(releases))
    }

    /// Map the `release_date` property of all releases in the set.
    pub fn map_release_date<F>(self, mut f: F) -> NightlyReleases<C>
    where
        F: FnMut(&RustRelease<Nightly, C>) -> Option<date::Date>,
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

        NightlyReleases(impls::ReleasesImpl::new(releases))
    }

    /// Map the `toolchains` property of all releases in the set.
    pub fn map_toolchains<F>(self, mut f: F) -> NightlyReleases<C>
    where
        F: FnMut(&RustRelease<Nightly, C>) -> Vec<toolchain::Toolchain>,
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

        NightlyReleases(impls::ReleasesImpl::new(releases))
    }

    /// Map the `context` property of all releases in the set, changing the context type from `C` to `C2`.
    pub fn map_context<C2, F>(self, mut f: F) -> NightlyReleases<C2>
    where
        F: FnMut(&RustRelease<Nightly, C>) -> C2,
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

        NightlyReleases(impls::ReleasesImpl::new(releases))
    }

    /// Merge two collections, applying `merge_fn` to releases that exist in both.
    ///
    /// Releases that exist in only one collection are included unchanged.
    pub fn merge_with<F>(self, right: NightlyReleases<C>, merge_fn: F) -> NightlyReleases<C>
    where
        F: Fn(RustRelease<Nightly, C>, RustRelease<Nightly, C>) -> RustRelease<Nightly, C>,
    {
        NightlyReleases(self.0.merge_with(right.0, merge_fn))
    }
}

impl<C> IntoIterator for NightlyReleases<C> {
    type Item = RustRelease<Nightly, C>;
    type IntoIter = std::collections::btree_set::IntoIter<RustRelease<Nightly, C>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<C> FromIterator<RustRelease<Nightly, C>> for NightlyReleases<C> {
    fn from_iter<T: IntoIterator<Item = RustRelease<Nightly, C>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl NightlyReleases<()> {
    /// Create a new, but empty, instance.
    ///
    /// NB: This function is only available for `C = ()`. Instances which use a different type `C`
    ///     can be created using `NightlyReleases::<C>::default()`.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_releases_core::NightlyReleases;
    ///
    /// let releases = NightlyReleases::empty();
    ///
    /// assert!(releases.is_empty());
    /// ```
    ///
    /// # See also
    ///
    /// [`NightlyReleases::default`]: create an empty collection, with any context type `C`.
    /// [`NightlyReleases::add`]: add releases to the collection.
    pub fn empty() -> Self {
        Self(impls::ReleasesImpl::default())
    }

    /// Merge two collections using default strategies (prefer left date, union toolchains).
    ///
    /// Releases that exist in only one collection are included unchanged.
    pub fn merge(self, right: NightlyReleases<()>) -> NightlyReleases<()> {
        self.merge_with(right, crate::merge::merge_default)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_release::date::Date;

    fn make_release(year: u16, month: u8, day: u8) -> RustRelease<Nightly> {
        RustRelease::new(Nightly::new(year, month, day), None, [])
    }

    #[test]
    fn merge_overlapping_sets() {
        let mut left = NightlyReleases::default();
        left.add(make_release(2024, 1, 1));
        left.add(make_release(2024, 1, 2));

        let mut right = NightlyReleases::default();
        right.add(make_release(2024, 1, 2));
        right.add(make_release(2024, 1, 3));

        let merged = left.merge(right);

        assert_eq!(merged.len(), 3);

        let versions: Vec<_> = merged.iter().map(|r| &r.version).collect();
        assert!(versions.contains(&&Nightly::new(2024, 1, 1)));
        assert!(versions.contains(&&Nightly::new(2024, 1, 2)));
        assert!(versions.contains(&&Nightly::new(2024, 1, 3)));
    }

    #[test]
    fn new_from_iterator() {
        let item0 = make_release(2024, 1, 2);
        let item1 = make_release(2024, 2, 3);

        let releases = NightlyReleases::new([item0.clone(), item1.clone()]);

        assert_eq!(releases.iter().collect::<Vec<_>>(), vec![&item0, &item1]);
    }

    #[test]
    fn map_version() {
        let mut original = NightlyReleases::empty();
        original.add(make_release(2024, 1, 2));
        original.add(make_release(2024, 2, 3));
        assert_eq!(original.len(), 2);

        // Eq. is determined by version
        let modified = original.map_version(|_n| Nightly::new(2024, 9, 9));
        assert_eq!(modified.len(), 1);

        let out = modified.iter().next().unwrap();
        assert_eq!(out.version().date, Date::new(2024, 9, 9));
    }
}
