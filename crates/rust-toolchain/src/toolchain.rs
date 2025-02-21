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

    pub fn channel(&self) -> &Channel {
        &self.channel
    }

    pub fn date(&self) -> &Option<Date> {
        &self.date
    }

    pub fn host(&self) -> &Target {
        &self.host
    }

    pub fn components(&self) -> &HashSet<Component> {
        &self.components
    }

    pub fn targets(&self) -> &HashSet<Target> {
        &self.targets
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

        assert!(&toolchain.channel().is_stable());
        assert_eq!(&toolchain.host, &Target::host());
    }
}
