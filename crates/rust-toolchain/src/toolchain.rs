use crate::channel::{Beta, Stable};
use crate::{Channel, Platform, RustVersion};

mod rustup_toolchain;

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Toolchain {
    pub channel: Channel,
    pub platform: Platform,
}

impl Toolchain {
    /// Construct a new toolchain.
    ///
    /// A toolchain consists of a `channel`, a `release date`, a platform
    pub fn new(channel: Channel, platform: Platform) -> Self {
        Self { channel, platform }
    }

    pub fn channel(&self) -> &Channel {
        &self.channel
    }

    pub fn platform(&self) -> &Platform {
        &self.platform
    }

    pub fn version(&self) -> Option<&RustVersion> {
        match &self.channel {
            Channel::Stable(Stable { version }) | Channel::Beta(Beta { version }) => Some(version),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_toolchain() {
        let channel = Channel::stable(RustVersion::new(1, 2, 3));

        let toolchain = Toolchain::new(channel, Platform::host());

        assert!(&toolchain.channel().is_stable());
        assert_eq!(&toolchain.platform, &Platform::host());
    }
}
