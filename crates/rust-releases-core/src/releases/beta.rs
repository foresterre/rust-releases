use crate::releases::impls;
use crate::Beta;
use rust_release::RustRelease;

#[derive(Debug, Default)]
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
        RustRelease::new(
            Beta {
                version: RustVersion::new(major, minor, patch),
                prerelease: None,
            },
            None,
            [],
        )
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
        assert!(versions.contains(&&Beta {
            version: RustVersion::new(1, 0, 0),
            prerelease: None
        }));
        assert!(versions.contains(&&Beta {
            version: RustVersion::new(2, 0, 0),
            prerelease: None
        }));
        assert!(versions.contains(&&Beta {
            version: RustVersion::new(3, 0, 0),
            prerelease: None
        }));
    }
}
