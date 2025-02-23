use crate::{Channel, Component, Date, Target};
use std::collections::HashSet;

/// A Rust toolchain
///
/// # Reading materials
///
/// - [`rustup concepts: toolchains`]
///
/// [`rustup concepts: toolchains`]: https://rust-lang.github.io/rustup/concepts/toolchains.html
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Toolchain {
    channel: Channel,
    date: Option<Date>,
    host: Target,

    components: HashSet<Component>,
    targets: HashSet<Target>,
}

impl Toolchain {
    /// Create a new toolchain instance
    pub fn new(
        channel: Channel,
        date: Option<Date>,
        host: Target,
        components: HashSet<Component>,
        targets: HashSet<Target>,
    ) -> Self {
        Self {
            channel,
            date,
            host,
            components,
            targets,
        }
    }

    /// The release associated with the toolchain
    pub fn channel(&self) -> &Channel {
        &self.channel
    }

    /// The date on which the toolchain was released
    pub fn date(&self) -> Option<&Date> {
        self.date.as_ref()
    }

    /// The host target associated with the toolchain
    pub fn host(&self) -> &Target {
        &self.host
    }

    /// The components associated with the toolchain
    pub fn components(&self) -> &HashSet<Component> {
        &self.components
    }

    /// The targets associated with the toolchain
    pub fn targets(&self) -> &HashSet<Target> {
        &self.targets
    }

    /// Update the associated channel
    pub fn set_channel(&mut self, channel: Channel) {
        self.channel = channel;
    }

    /// Update the associated toolchain release date
    pub fn set_date(&mut self, date: Option<Date>) {
        self.date = date;
    }

    /// Updated the associated host platform
    pub fn set_host(&mut self, host: Target) {
        self.host = host;
    }

    /// Update the associated toolchain components
    pub fn set_components(&mut self, components: HashSet<Component>) {
        self.components = components;
    }

    /// Update the associated toolchain targets
    pub fn set_targets(&mut self, targets: HashSet<Target>) {
        self.targets = targets;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RustVersion;

    #[test]
    fn create_toolchain() {
        let channel = Channel::stable(RustVersion::new(1, 2, 3));

        let toolchain = Toolchain::new(
            channel,
            None,
            Target::host(),
            HashSet::new(),
            HashSet::new(),
        );

        assert!(toolchain.channel().is_stable());
        assert_eq!(toolchain.host(), &Target::host());
    }

    #[test]
    fn channel() {
        let channel = Channel::stable(RustVersion::new(1, 2, 3));
        let mut toolchain = Toolchain::new(
            channel,
            None,
            Target::host(),
            HashSet::new(),
            HashSet::new(),
        );

        assert!(toolchain.channel().is_stable());

        toolchain.set_channel(Channel::beta(RustVersion::new(1, 2, 4)));

        assert!(toolchain.channel().is_beta());
    }

    #[test]
    fn date() {
        let channel = Channel::stable(RustVersion::new(1, 2, 3));
        let mut toolchain = Toolchain::new(
            channel,
            None,
            Target::host(),
            HashSet::new(),
            HashSet::new(),
        );

        assert!(toolchain.date().is_none());

        toolchain.set_date(Some(Date::new(2025, 1, 2)));

        assert_eq!(toolchain.date().unwrap().year(), 2025);
        assert_eq!(toolchain.date().unwrap().month(), 1);
        assert_eq!(toolchain.date().unwrap().day(), 2);
    }

    #[test]
    fn host() {
        let channel = Channel::stable(RustVersion::new(1, 2, 3));
        let mut toolchain = Toolchain::new(
            channel,
            None,
            Target::host(),
            HashSet::new(),
            HashSet::new(),
        );

        assert_eq!(toolchain.host(), &Target::host());

        toolchain.set_host(Target::from_target_triple_or_unknown("make it unknown"));

        assert_eq!(
            toolchain.host(),
            &Target::from_target_triple_or_unknown("make it unknown")
        );
    }

    #[test]
    fn components() {
        let channel = Channel::stable(RustVersion::new(1, 2, 3));
        let mut toolchain = Toolchain::new(
            channel,
            None,
            Target::host(),
            HashSet::new(),
            HashSet::new(),
        );

        assert!(toolchain.components().is_empty());

        let mut set = HashSet::new();
        set.insert(Component::new("hello"));
        set.insert(Component::new("world"));

        let expected = set.clone();

        toolchain.set_components(set);

        assert_eq!(toolchain.components(), &expected);
    }

    #[test]
    fn targets() {
        let channel = Channel::stable(RustVersion::new(1, 2, 3));
        let mut toolchain = Toolchain::new(
            channel,
            None,
            Target::host(),
            HashSet::new(),
            HashSet::new(),
        );

        assert!(toolchain.targets().is_empty());

        let mut set = HashSet::new();
        set.insert(Target::from_target_triple_or_unknown("hello"));

        let expected = set.clone();

        toolchain.set_targets(set);

        assert_eq!(toolchain.targets(), &expected);
    }
}
