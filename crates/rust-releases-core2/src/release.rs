#[cfg(test)]
mod tests;

#[cfg(test)]
mod tests_eq;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Release {
    date: rust_toolchain::ReleaseDate,
    toolchain: rust_toolchain::Toolchain,
    components: Vec<rust_toolchain::Component>,
}

impl Release {
    /// Create a new [`Release`] instance for a given toolchain with the given components.
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

    /// Create a new [`Release`] instance for a given toolchain, without any components.
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
    /// use rust_releases_core2::Release;
    ///
    /// let channel = rust_toolchain::Channel::Nightly;
    /// let release_date = rust_toolchain::ReleaseDate::new(2023, 1, 1);
    /// let platform = rust_toolchain::Platform::host();
    /// let version = None;
    ///
    /// let toolchain = rust_toolchain::Toolchain::new(channel, platform, version);
    ///
    /// let release = Release::new(release_date, toolchain, vec![]);
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
