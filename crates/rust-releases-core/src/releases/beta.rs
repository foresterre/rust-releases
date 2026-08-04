use crate::releases::impls;
use crate::Beta;
use rust_release::{date, toolchain, RustRelease};
use std::iter::FromIterator;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BetaReleases<C = ()>(impls::ReleasesImpl<Beta, C>);

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

    /// Map the `release` and re-collect the result.
    ///
    /// # Warning: may result in unintended consequences
    ///
    /// Internally, versions are stored by `version` in a `BTreeSet`, so if you
    /// map the version specifically, you might lose releases unexpectedly.
    pub fn map<F>(self, f: F) -> BetaReleases<C>
    where
        F: FnMut(RustRelease<Beta, C>) -> RustRelease<Beta, C>,
    {
        let releases = self.into_iter().map(f).collect();

        BetaReleases(impls::ReleasesImpl::new(releases))
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
    ///  # use rust_releases_core::{Beta, BetaReleases};
    ///
    ///  let item0 = RustRelease::new(Beta::new(1, 2, 3, None), None, []);
    ///  let item1 = RustRelease::new(Beta::new(2, 3, 4, None), None, []);
    ///
    ///  let mut original = BetaReleases::empty();
    ///  original.add(item0.clone());
    ///  original.add(item1.clone());
    ///  assert_eq!(original.len(), 2);
    ///
    ///  // Eq. is determined by version
    ///  let modified = original.map_version(|_| Beta::new(9, 9, 9, None));
    ///  assert_eq!(modified.len(), 1);
    /// ```
    pub fn map_version<F>(self, mut f: F) -> BetaReleases<C>
    where
        F: FnMut(&RustRelease<Beta, C>) -> Beta,
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

        BetaReleases(impls::ReleasesImpl::new(releases))
    }

    /// Map the `release_date` property of all releases in the set.
    pub fn map_release_date<F>(self, mut f: F) -> BetaReleases<C>
    where
        F: FnMut(&RustRelease<Beta, C>) -> Option<date::Date>,
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

        BetaReleases(impls::ReleasesImpl::new(releases))
    }

    /// Map the `toolchains` property of all releases in the set.
    pub fn map_toolchains<F>(self, mut f: F) -> BetaReleases<C>
    where
        F: FnMut(&RustRelease<Beta, C>) -> Vec<toolchain::Toolchain>,
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

        BetaReleases(impls::ReleasesImpl::new(releases))
    }

    /// Map the `context` property of all releases in the set, changing the context type from `C` to `C2`.
    pub fn map_context<C2, F>(self, mut f: F) -> BetaReleases<C2>
    where
        F: FnMut(&RustRelease<Beta, C>) -> C2,
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

        BetaReleases(impls::ReleasesImpl::new(releases))
    }

    /// Merge two collections, applying `merge_fn` to releases that exist in both.
    ///
    /// Releases that exist in only one collection are included unchanged.
    pub fn merge_with<F>(self, right: BetaReleases<C>, merge_fn: F) -> BetaReleases<C>
    where
        F: Fn(RustRelease<Beta, C>, RustRelease<Beta, C>) -> RustRelease<Beta, C>,
    {
        BetaReleases(self.0.merge_with(right.0, merge_fn))
    }
}

impl<C> IntoIterator for BetaReleases<C> {
    type Item = RustRelease<Beta, C>;
    type IntoIter = std::collections::btree_set::IntoIter<RustRelease<Beta, C>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<C> FromIterator<RustRelease<Beta, C>> for BetaReleases<C> {
    fn from_iter<T: IntoIterator<Item = RustRelease<Beta, C>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl BetaReleases<()> {
    /// Create a new, but empty, instance.
    ///
    /// NB: This function is only available for `C = ()`. Instances which use a different type `C`
    ///     can be created using `BetaReleases::<C>::default()`.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_releases_core::BetaReleases;
    ///
    /// let releases = BetaReleases::empty();
    ///
    /// assert!(releases.is_empty());
    /// ```
    ///
    /// # See also
    ///
    /// [`BetaReleases::default`]: create an empty collection, with any context type `C`.
    /// [`BetaReleases::add`]: add releases to the collection.
    pub fn empty() -> Self {
        Self(impls::ReleasesImpl::default())
    }

    /// Merge two collections using default strategies (prefer left date, union toolchains).
    ///
    /// Releases that exist in only one collection are included unchanged.
    pub fn merge(self, right: BetaReleases<()>) -> BetaReleases<()> {
        self.merge_with(right, crate::merge::merge_default)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_release::toolchain::RustVersion;

    fn make_release(major: u64, minor: u64, patch: u64) -> RustRelease<Beta> {
        RustRelease::new(Beta::new(major, minor, patch, None), None, [])
    }

    #[test]
    fn merge_overlapping_sets() {
        let mut left = BetaReleases::default();
        left.add(make_release(1, 0, 0));
        left.add(make_release(2, 0, 0));

        let mut right = BetaReleases::default();
        right.add(make_release(2, 0, 0));
        right.add(make_release(3, 0, 0));

        let merged = left.merge(right);

        assert_eq!(merged.len(), 3);

        let versions: Vec<_> = merged.iter().map(|r| &r.version).collect();
        assert!(versions.contains(&&Beta::new(1, 0, 0, None)));
        assert!(versions.contains(&&Beta::new(2, 0, 0, None)));
        assert!(versions.contains(&&Beta::new(3, 0, 0, None)));
    }

    #[test]
    fn map_version() {
        let mut original = BetaReleases::empty();
        original.add(make_release(1, 2, 3));
        original.add(make_release(2, 3, 4));
        assert_eq!(original.len(), 2);

        // Eq. is determined by version
        let modified = original.map_version(|_b| Beta::new(9, 9, 9, None));
        assert_eq!(modified.len(), 1);

        let out = modified.iter().next().unwrap();
        assert_eq!(out.version().version, RustVersion::new(9, 9, 9));
    }
}
