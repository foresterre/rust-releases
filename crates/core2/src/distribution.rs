//! See [`Distribution`].
#![forbid(missing_docs)]

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
    ///
    /// # Example
    ///
    /// ```
    /// # use std::iter;
    /// # use rust_releases_core2::Distribution;
    /// # use rust_toolchain::{Channel, Component, Platform, ReleaseDate, RustVersion, Toolchain};
    /// #
    /// let date = ReleaseDate::new(2018, 12, 6);
    /// let toolchain = Toolchain::new(Channel::stable(RustVersion::new(1, 31, 0)), Platform::host());
    /// let component = Component::new_component("some");
    ///
    /// // Creating a Rust distribution for Rust 1.31 which was released on 2018-12-06
    /// let distribution = Distribution::new(date, toolchain, iter::once(component));
    /// ```
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
    ///
    /// # Example
    ///
    /// ```
    /// # use std::iter;
    /// # use rust_releases_core2::Distribution;
    /// # use rust_toolchain::{Channel, Component, Platform, ReleaseDate, RustVersion, Toolchain};
    /// #
    /// let date = ReleaseDate::new(2018, 12, 6);
    /// let toolchain = Toolchain::new(Channel::stable(RustVersion::new(1, 31, 0)), Platform::host());
    ///
    /// // Creating a Rust distribution for Rust 1.31 which was released on 2018-12-06
    /// let distribution = Distribution::new_without_components(date, toolchain);
    /// ```
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
    ///
    /// # Example
    ///
    /// ```
    /// # use std::iter;
    /// # use rust_releases_core2::Distribution;
    /// # use rust_toolchain::{Channel, Component, Platform, ReleaseDate, RustVersion, Toolchain};
    /// #
    /// # let date = ReleaseDate::new(2018, 12, 6);
    /// # let toolchain = Toolchain::new(Channel::stable(RustVersion::new(1, 31, 0)), Platform::host());
    /// #
    /// // Creating a Rust distribution for Rust 1.31 which was released on 2018-12-06
    /// let distribution = Distribution::new_without_components(date, toolchain);
    ///
    /// assert_eq!(distribution.date(), &ReleaseDate::new(2018, 12, 6));
    /// ```
    pub fn date(&self) -> &rust_toolchain::ReleaseDate {
        &self.date
    }

    /// Returns an exclusive reference to the release date.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::iter;
    /// # use rust_releases_core2::Distribution;
    /// # use rust_toolchain::{Channel, Component, Platform, ReleaseDate, RustVersion, Toolchain};
    /// #
    /// let date = ReleaseDate::new(2018, 12, 6);
    /// let toolchain = Toolchain::new(Channel::stable(RustVersion::new(1, 31, 0)), Platform::host());
    /// #
    /// // Creating a Rust distribution for Rust 1.31 which was released on 2018-12-06
    /// let mut distribution = Distribution::new_without_components(date, toolchain);
    ///
    /// // Modifying the release date!
    /// *distribution.date_mut() = ReleaseDate::new(2018, 1, 1);
    ///
    /// assert_eq!(distribution.date(), &ReleaseDate::new(2018, 1, 1));
    /// ```
    ///
    /// # A note on modifying nightly releases
    ///
    /// Nightly releases are versioned by their release date. As such, it
    /// is usually the case that this release date, and its release version are
    /// in sync.
    ///
    /// If you use this method to modify the release date, make sure you also
    /// (if desired, of course), update the release date of the nightly toolchain:
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
    ///
    /// # Example
    ///
    /// ```
    /// # use std::iter;
    /// # use rust_releases_core2::Distribution;
    /// # use rust_toolchain::{Channel, Component, Platform, ReleaseDate, RustVersion, Toolchain};
    /// #
    /// # let date = ReleaseDate::new(2018, 12, 6);
    /// # let creation_toolchain = Toolchain::new(Channel::stable(RustVersion::new(1, 31, 0)), Platform::host());
    /// #
    /// // Creating a Rust distribution for Rust 1.31 which was released on 2018-12-06
    /// let distribution = Distribution::new_without_components(date, creation_toolchain);
    ///
    /// // Get the toolchain
    /// let toolchain = distribution.toolchain();
    ///
    /// assert_eq!(toolchain.version().unwrap(), &RustVersion::new(1, 31, 0));
    /// assert!(toolchain.channel().is_stable());
    /// ```
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
    /// ```
    /// # use std::iter;
    /// # use rust_releases_core2::Distribution;
    /// # use rust_toolchain::{Channel, Component, Platform, ReleaseDate, RustVersion, Toolchain};
    /// #
    /// # let date = ReleaseDate::new(2018, 12, 6);
    /// # let toolchain = Toolchain::new(Channel::stable(RustVersion::new(1, 31, 0)), Platform::host());
    /// #
    /// let default_component = Component::new_component("a-default-component");
    /// let optional_component = Component::new_extension("an-optional-component");
    ///
    /// // Creating a Rust distribution with a default and an optional component.
    /// let distribution = Distribution::new(date, toolchain, [default_component, optional_component]);
    ///
    /// let component = distribution.find_component("an-optional-component").unwrap();
    ///
    /// assert_eq!(component.name, "an-optional-component");
    /// assert!(component.optional);
    ///
    /// let not_included = distribution.find_component("not-this-one");
    ///
    /// assert!(not_included.is_none());
    /// ```
    pub fn find_component(&self, name: &str) -> Option<&rust_toolchain::Component> {
        self.components.iter().find(|f| f.name == name)
    }

    /// Returns an iterator over the components which are enabled by default.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::iter;
    /// # use rust_releases_core2::Distribution;
    /// # use rust_toolchain::{Channel, Component, Platform, ReleaseDate, RustVersion, Toolchain};
    /// #
    /// # let date = ReleaseDate::new(2018, 12, 6);
    /// # let toolchain = Toolchain::new(Channel::stable(RustVersion::new(1, 31, 0)), Platform::host());
    /// #
    /// let default_component = Component::new_component("a-default-component");
    /// let optional_component = Component::new_extension("an-optional-component");
    ///
    /// // Creating a Rust distribution with a default and an optional component.
    /// let distribution = Distribution::new(date, toolchain, [default_component, optional_component]);
    ///
    /// let subset_of_components = distribution.default_components().collect::<Vec<_>>();
    ///
    /// assert_eq!(subset_of_components.len(), 1);
    /// assert_eq!(&subset_of_components[0].name, &"a-default-component");
    /// assert!(!&subset_of_components[0].optional);
    /// ```
    pub fn default_components(&self) -> impl Iterator<Item = &rust_toolchain::Component> {
        self.components.iter().filter(|f| !f.optional)
    }

    /// Returns an iterator over the components which are optional, and not installed by default.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::iter;
    /// # use rust_releases_core2::Distribution;
    /// # use rust_toolchain::{Channel, Component, Platform, ReleaseDate, RustVersion, Toolchain};
    /// #
    /// # let date = ReleaseDate::new(2018, 12, 6);
    /// # let toolchain = Toolchain::new(Channel::stable(RustVersion::new(1, 31, 0)), Platform::host());
    /// #
    /// let default_component = Component::new_component("a-default-component");
    /// let optional_component = Component::new_extension("an-optional-component");
    ///
    /// // Creating a Rust distribution with a default and an optional component.
    /// let distribution = Distribution::new(date, toolchain, [default_component, optional_component]);
    ///
    /// let subset_of_components = distribution.extension_components().collect::<Vec<_>>();
    ///
    /// assert_eq!(subset_of_components.len(), 1);
    /// assert_eq!(&subset_of_components[0].name, &"an-optional-component");
    /// assert!(&subset_of_components[0].optional)
    /// ```
    pub fn extension_components(&self) -> impl Iterator<Item = &rust_toolchain::Component> {
        self.components.iter().filter(|f| f.optional)
    }

    /// Whether this is a _stable_ release.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::iter;
    /// # use rust_releases_core2::Distribution;
    /// # use rust_toolchain::{Channel, Component, Platform, ReleaseDate, RustVersion, Toolchain};
    /// #
    /// # let date = ReleaseDate::new(2018, 12, 6);
    /// # let toolchain = Toolchain::new(Channel::stable(RustVersion::new(1, 31, 0)), Platform::host());
    /// #
    /// // Creating a Rust distribution for Rust 1.31 which was released on 2018-12-06
    /// let distribution = Distribution::new_without_components(date, toolchain);
    ///
    /// assert!(distribution.is_stable());
    ///
    /// assert!(!distribution.is_beta());
    /// assert!(!distribution.is_nightly());
    /// ```
    pub fn is_stable(&self) -> bool {
        self.toolchain.channel.is_stable()
    }

    /// Whether this is a _beta_ release.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::iter;
    /// # use rust_releases_core2::Distribution;
    /// # use rust_toolchain::{Channel, Component, Platform, ReleaseDate, RustVersion, Toolchain};
    /// #
    /// # let date = ReleaseDate::new(2018, 12, 6);
    /// # let toolchain = Toolchain::new(Channel::beta(RustVersion::new(1, 32, 0)), Platform::host());
    /// #
    /// // Creating a Rust distribution for Rust 1.32-beta which was released on 2018-12-06
    /// let distribution = Distribution::new_without_components(date, toolchain);
    ///
    /// assert!(distribution.is_beta());
    ///
    /// assert!(!distribution.is_stable());
    /// assert!(!distribution.is_nightly());
    /// ```
    pub fn is_beta(&self) -> bool {
        self.toolchain.channel.is_beta()
    }

    /// Whether this is a _nightly_ release.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::iter;
    /// # use rust_releases_core2::Distribution;
    /// # use rust_toolchain::{Channel, Component, Platform, ReleaseDate, RustVersion, Toolchain};
    /// #
    /// # let date = ReleaseDate::new(2018, 12, 6);
    /// # let toolchain = Toolchain::new(Channel::nightly(date.clone()), Platform::host());
    /// #
    /// // Creating a Rust distribution for the Rust nightly of 2018-12-06
    /// let distribution = Distribution::new_without_components(date, toolchain);
    ///
    /// assert!(distribution.is_nightly());
    ///
    /// assert!(!distribution.is_stable());
    /// assert!(!distribution.is_beta())
    /// ```
    pub fn is_nightly(&self) -> bool {
        self.toolchain.channel.is_nightly()
    }
}
