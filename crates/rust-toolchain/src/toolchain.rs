use crate::{Channel, ComponentSet, Date, Target, TargetSet};

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

    components: ComponentSet,
    targets: TargetSet,
}

impl Toolchain {
    /// Create a new toolchain instance
    pub fn new(
        channel: Channel,
        date: Option<Date>,
        host: Target,
        components: ComponentSet,
        targets: TargetSet,
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
    pub fn components(&self) -> &ComponentSet {
        &self.components
    }

    /// The targets associated with the toolchain
    pub fn targets(&self) -> &TargetSet {
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
    pub fn set_components(&mut self, components: ComponentSet) {
        self.components = components;
    }

    /// Update the associated toolchain targets
    pub fn set_targets(&mut self, targets: TargetSet) {
        self.targets = targets;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Component, RustVersion};
    use std::ptr;

    #[test]
    fn create_toolchain() {
        let channel = Channel::stable(RustVersion::new(1, 2, 3));

        let toolchain = Toolchain::new(
            channel,
            None,
            Target::host(),
            ComponentSet::default(),
            TargetSet::default(),
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
            ComponentSet::default(),
            TargetSet::default(),
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
            ComponentSet::default(),
            TargetSet::default(),
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
            ComponentSet::default(),
            TargetSet::default(),
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
            ComponentSet::default(),
            TargetSet::default(),
        );

        assert!(toolchain.components().is_empty());

        let set = [Component::new("hello"), Component::new("world")]
            .into_iter()
            .collect::<ComponentSet>();

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
            ComponentSet::default(),
            TargetSet::default(),
        );

        assert!(toolchain.targets().is_empty());

        let set = [Target::from_target_triple_or_unknown("hello")]
            .into_iter()
            .collect::<TargetSet>();

        let expected = set.clone();

        toolchain.set_targets(set);

        assert_eq!(toolchain.targets(), &expected);
    }

    #[test]
    fn a_clone_shares_the_components_and_targets_of_the_toolchain_it_was_cloned_from() {
        let toolchain = Toolchain::new(
            Channel::stable(RustVersion::new(1, 2, 3)),
            None,
            Target::host(),
            [Component::new("cargo")].into_iter().collect(),
            [Target::host()].into_iter().collect(),
        );

        let clone = toolchain.clone();

        assert!(ptr::eq(
            toolchain.components().as_slice(),
            clone.components().as_slice()
        ));
        assert!(ptr::eq(
            toolchain.targets().as_slice(),
            clone.targets().as_slice()
        ));
    }
}
