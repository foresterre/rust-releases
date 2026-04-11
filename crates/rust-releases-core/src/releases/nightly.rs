use crate::releases::impls;
use crate::Nightly;
use rust_release::RustRelease;

#[derive(Debug, Default)]
pub struct NightlyReleases<C = ()>(impls::ReleasesImpl<Nightly, C>);

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

impl NightlyReleases<()> {
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
        RustRelease::new(Nightly { date: Date::new(year, month, day) }, None, [])
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
        assert!(versions.contains(&&Nightly { date: Date::new(2024, 1, 1) }));
        assert!(versions.contains(&&Nightly { date: Date::new(2024, 1, 2) }));
        assert!(versions.contains(&&Nightly { date: Date::new(2024, 1, 3) }));
    }
}
