#[cfg(test)]
mod tests;

#[cfg(test)]
mod tests_eq;

/// A _Rust distribution_, is a combined package which includes a _Rust toolchain_,
/// consisting of the Rust compiler (`rustc`), and usually several common tools
/// and libraries, like the Rust package manager (`cargo`) and the Rust standard library.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Distribution {
    date: rust_toolchain::ReleaseDate,
    toolchain: rust_toolchain::Toolchain,
    components: Vec<rust_toolchain::Component>,
}

impl Distribution {
    /// Create a new [`Distribution`] for a given toolchain with the given components.
    pub fn new(
        date: rust_toolchain::ReleaseDate,
        toolchain: rust_toolchain::Toolchain,
        components: impl IntoIterator<Item = rust_toolchain::Component>,
    ) -> Self {
        Self {
            date,
            toolchain,
            components: components.into_iter().collect(),
        }
    }

    /// Create a new [`Distribution`] for a given toolchain, without any components.
    pub fn new_without_components(
        date: rust_toolchain::ReleaseDate,
        toolchain: rust_toolchain::Toolchain,
    ) -> Self {
        Self {
            date,
            toolchain,
            components: Vec::with_capacity(0),
        }
    }

    /// Returns a shared reference to the release date.
    pub fn date(&self) -> &rust_toolchain::ReleaseDate {
        &self.date
    }

    /// Returns an exclusive reference to the release date.
    ///
    /// # A note on modifying nightly releases
    ///
    /// Nightly releases are versioned by their release date. As such, it
    /// is expected that this release date, and its release version are always
    /// equal and in sync.
    ///
    /// If you use this method to modify the release date, make sure you also
    /// update the release date of the nightly toolchain:
    ///
    /// ```
    /// # use rust_releases_core2::Distribution;
    /// use rust_toolchain::{Channel, Platform, ReleaseDate, Toolchain};
    ///
    /// let date = ReleaseDate::new(2023, 1, 1);
    /// let platform = Platform::host();
    /// let channel = Channel::nightly(date.clone()); // <- Note that a nightly version is a date
    /// let toolchain = Toolchain::new(channel, platform);
    ///
    /// let mut release = Distribution::new_without_components(date, toolchain); // <- Note that a release also has a date
    ///
    /// // Modify the date
    /// *release.date_mut() = ReleaseDate::new(2024, 1, 1);
    ///
    /// // Now the dates are out of sync. This may have unintended consequences.
    /// assert_ne!(release.date(), release.toolchain().nightly_date().unwrap());
    ///
    /// // Also update the nightly toolchain version by updating the channel
    /// release.toolchain_mut().channel = Channel::nightly(ReleaseDate::new(2024, 1, 1));
    ///
    /// // Now the dates are back in sync.
    /// assert_eq!(release.date(), release.toolchain().nightly_date().unwrap());
    /// ```
    pub fn date_mut(&mut self) -> &mut rust_toolchain::ReleaseDate {
        &mut self.date
    }

    /// Returns a shared reference to the [`rust_toolchain::Toolchain`].
    pub fn toolchain(&self) -> &rust_toolchain::Toolchain {
        &self.toolchain
    }

    /// Returns an exclusive reference to the [`rust_toolchain::Toolchain`].
    pub fn toolchain_mut(&mut self) -> &mut rust_toolchain::Toolchain {
        &mut self.toolchain
    }

    /// Find a component by its name.
    ///
    /// If the component does not exist for this `Release`, returns `Option::None`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_releases_core2::Distribution;
    ///
    /// let release_date = rust_toolchain::ReleaseDate::new(2023, 1, 1);
    /// let version = rust_toolchain::RustVersion::new(1, 31, 1);
    ///
    /// let channel = rust_toolchain::Channel::beta(version);
    /// let platform = rust_toolchain::Platform::host();
    ///
    /// let toolchain = rust_toolchain::Toolchain::new(channel, platform);
    ///
    /// let release = Distribution::new(release_date, toolchain, []);
    /// let component = release.find_component("hello");
    ///
    /// assert!(component.is_none());
    /// ```
    pub fn find_component(&self, name: &str) -> Option<&rust_toolchain::Component> {
        self.components.iter().find(|f| f.name == name)
    }

    /// Returns an iterator over the components which are enabled by default.
    pub fn default_components(&self) -> impl Iterator<Item = &rust_toolchain::Component> {
        self.components.iter().filter(|f| !f.optional)
    }

    /// Returns an iterator over the components which are optional, and not installed by default.
    pub fn extension_components(&self) -> impl Iterator<Item = &rust_toolchain::Component> {
        self.components.iter().filter(|f| f.optional)
    }

    /// Whether this is a _stable_ release.
    pub fn is_stable(&self) -> bool {
        self.toolchain.channel.is_stable()
    }

    /// Whether this is a _beta_ release.
    pub fn is_beta(&self) -> bool {
        self.toolchain.channel.is_beta()
    }

    /// Whether this is a _nightly_ release.
    pub fn is_nightly(&self) -> bool {
        self.toolchain.channel.is_nightly()
    }
}
