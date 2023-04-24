use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Release {
    toolchain: rust_toolchain::Toolchain,
    components: Vec<rust_toolchain::Component>,
}

impl Release {
    pub fn new(
        toolchain: rust_toolchain::Toolchain,
        components: impl IntoIterator<Item = rust_toolchain::Component>,
    ) -> Self {
        Self {
            toolchain,
            components: components.into_iter().collect(),
        }
    }

    pub fn toolchain(&self) -> &rust_toolchain::Toolchain {
        &self.toolchain
    }

    pub fn release_date(&self) -> rust_toolchain::ReleaseDate {
        todo!("release date of the toolchain")
    }

    /// Find a component by its name.
    ///
    /// If the component does not exist for this `Release`, returns `Option::None`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_releases_core2::Release;
    ///
    /// let channel = rust_toolchain::Channel::Nightly;
    /// let release_date = rust_toolchain::ReleaseDate::new(2023, 1, 1);
    /// let platform = rust_toolchain::Platform::host();
    /// let version = None;
    ///
    /// let toolchain = rust_toolchain::Toolchain::new(channel, release_date, platform, version);
    ///
    /// let release = Release::new(toolchain, vec![]);
    /// let component = release.find_component("hello");
    ///
    /// assert!(component.is_none());
    /// ```
    pub fn find_component(&self, name: &str) -> Option<&rust_toolchain::Component> {
        self.components.iter().find(|f| f.name == name)
    }

    pub fn default_components(&self) -> impl Iterator<Item = &rust_toolchain::Component> {
        self.components.iter().filter(|f| !f.optional)
    }

    /// Returns an iterator over the components which are optional, and not installed by default.
    pub fn extension_components(&self) -> impl Iterator<Item = &rust_toolchain::Component> {
        self.components.iter().filter(|f| f.optional)
    }
}

impl PartialOrd for Release {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Release {
    fn cmp(&self, other: &Self) -> Ordering {
        let c1 = CompareRustToolchain::new(self.toolchain());
        let c2 = CompareRustToolchain::new(other.toolchain());

        c1.cmp(&c2)
    }
}

/// Comparator which prioritizes Rust versions over release dates, but will fall back
/// to release dates if no version is known.
///
/// While this comparator accepts a mix of versions (stable has both) and release dates
/// (as used by nightly), commonly, you should only use either versions or release dates.
/// TODO: maybe we should use just release date and only prio versions to those which do not
///     have a version; why?: because stuff like PlatformRegister::most_recent, can now
///     be incorrect, because we return an old version over a recent nighly.
///     To compare for just stable, we can still use `.iter(|r| r.channel == Stable)`.
#[derive(Clone, Debug, PartialEq, Eq)]
struct CompareRustToolchain<'toolchain> {
    toolchain: &'toolchain rust_toolchain::Toolchain,
}

impl<'toolchain> CompareRustToolchain<'toolchain> {
    pub fn new(toolchain: &'toolchain rust_toolchain::Toolchain) -> Self {
        Self { toolchain }
    }
}

impl<'toolchain> Ord for CompareRustToolchain<'toolchain> {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_toolchain = self.toolchain;
        let other_toolchain = other.toolchain;

        let this = self_toolchain.version();
        let that = other_toolchain.version();

        match (this, that) {
            // If both have a version, newer versions win
            (Some(l), Some(r)) => l.cmp(&r),
            // If either has a version, but the other hasn't, the version wins,
            (Some(l), None) => Ordering::Greater,
            // Same as above
            (None, Some(r)) => Ordering::Less,
            // If neither has a version, we only do a date compare,
            (None, None) => self_toolchain
                .release_date()
                .cmp(&other_toolchain.release_date()),
        }
    }
}

impl<'toolchain> PartialOrd for CompareRustToolchain<'toolchain> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests_compare_rust_toolchain {
    use super::*;

    #[test]
    fn ordering_by_version() {
        let channel = rust_toolchain::Channel::Stable;
        let release_date = rust_toolchain::ReleaseDate::new(2022, 1, 1);
        let platform = rust_toolchain::Platform::host();

        let toolchain1 = rust_toolchain::Toolchain::new(
            channel,
            release_date.clone(),
            platform.clone(),
            Some(rust_toolchain::RustVersion::new(1, 0, 0)),
        );

        let toolchain2 = rust_toolchain::Toolchain::new(
            channel,
            release_date,
            platform,
            Some(rust_toolchain::RustVersion::new(0, 0, 0)),
        );

        let c1 = CompareRustToolchain::new(&toolchain1);
        let c2 = CompareRustToolchain::new(&toolchain2);

        assert_eq!(c1.cmp(&c2), Ordering::Greater);
    }

    #[test]
    fn ordering_by_date() {
        let channel = rust_toolchain::Channel::Stable;
        let release_date1 = rust_toolchain::ReleaseDate::new(2023, 1, 1);
        let release_date2 = rust_toolchain::ReleaseDate::new(2022, 1, 1);

        let platform = rust_toolchain::Platform::host();

        let toolchain1 =
            rust_toolchain::Toolchain::new(channel, release_date1, platform.clone(), None);

        let toolchain2 = rust_toolchain::Toolchain::new(channel, release_date2, platform, None);

        let c1 = CompareRustToolchain::new(&toolchain1);
        let c2 = CompareRustToolchain::new(&toolchain2);

        assert_eq!(c1.cmp(&c2), Ordering::Greater);
    }

    #[test]
    fn ordering_with_version_and_date_left() {
        let channel = rust_toolchain::Channel::Stable;
        let release_date1 = rust_toolchain::ReleaseDate::new(2022, 1, 1);
        let release_date2 = rust_toolchain::ReleaseDate::new(2023, 1, 1);

        let platform = rust_toolchain::Platform::host();

        let toolchain1 = rust_toolchain::Toolchain::new(
            channel,
            release_date1,
            platform.clone(),
            Some(rust_toolchain::RustVersion::new(1, 0, 0)),
        );

        let toolchain2 = rust_toolchain::Toolchain::new(channel, release_date2, platform, None);

        let c1 = CompareRustToolchain::new(&toolchain1);
        let c2 = CompareRustToolchain::new(&toolchain2);

        // Regardless of the newer release date of toolchain2, toolchain1 will be greater because it has a version
        assert_eq!(c1.cmp(&c2), Ordering::Greater);
    }

    #[test]
    fn ordering_with_version_and_date_right() {
        let channel = rust_toolchain::Channel::Stable;
        let release_date1 = rust_toolchain::ReleaseDate::new(2023, 1, 1);
        let release_date2 = rust_toolchain::ReleaseDate::new(2022, 1, 1);

        let platform = rust_toolchain::Platform::host();

        let toolchain1 =
            rust_toolchain::Toolchain::new(channel, release_date1, platform.clone(), None);

        let toolchain2 = rust_toolchain::Toolchain::new(
            channel,
            release_date2,
            platform,
            Some(rust_toolchain::RustVersion::new(1, 0, 0)),
        );

        let c1 = CompareRustToolchain::new(&toolchain1);
        let c2 = CompareRustToolchain::new(&toolchain2);

        // Regardless of the newer release date of toolchain1, toolchain2 will be greater because it has a version
        assert_eq!(c1.cmp(&c2), Ordering::Less);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_component() {
        let release: Release = todo!();

        release.find_component("hello-world");
    }

    #[test]
    fn extensions() {
        let channel = rust_toolchain::Channel::Nightly;
        let release_date = rust_toolchain::ReleaseDate::new(2023, 1, 1);
        let platform = rust_toolchain::Platform::host();
        let version = None;

        let toolchain = rust_toolchain::Toolchain::new(channel, release_date, platform, version);

        let release = Release::new(toolchain, vec![]);

        let ext = release.extension_components();
        let ext = release.default_components();
    }

    #[test]
    fn find_component_returns_none_if_release_has_no_components() {
        let channel = rust_toolchain::Channel::Nightly;
        let release_date = rust_toolchain::ReleaseDate::new(2023, 1, 1);
        let platform = rust_toolchain::Platform::host();
        let version = None;

        let toolchain = rust_toolchain::Toolchain::new(channel, release_date, platform, version);

        let release = Release::new(toolchain, vec![]);
        let component = release.find_component("hello");

        assert!(component.is_none());
    }
}
