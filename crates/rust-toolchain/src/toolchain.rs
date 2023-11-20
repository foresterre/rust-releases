use crate::channel::{Beta, Stable};
use crate::{Channel, Nightly, ReleaseDate, RustVersion, Target};

mod rustup_toolchain;

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Toolchain {
    pub channel: Channel,
    pub platform: Target,
}

impl Toolchain {
    /// Construct a new toolchain.
    ///
    /// A toolchain consists of a [`channel`] and a [`platform`].
    ///
    /// [`channel`]: Channel
    /// [`platform`]: Target
    pub fn new(channel: Channel, platform: Target) -> Self {
        Self { channel, platform }
    }

    pub fn channel(&self) -> &Channel {
        &self.channel
    }

    pub fn platform(&self) -> &Target {
        &self.platform
    }

    /// The version of the toolchain, if any.
    ///
    /// A nightly release is versioned by date instead of by version.
    /// To obtain the nightly release date, see [`Toolchain::nightly_date`].
    pub fn version(&self) -> Option<&RustVersion> {
        match &self.channel {
            Channel::Stable(Stable { version }) | Channel::Beta(Beta { version }) => Some(version),
            _ => None,
        }
    }

    /// The 'version' of a nightly release.
    /// Returns `None` if this is not a nightly release.
    pub fn nightly_date(&self) -> Option<&ReleaseDate> {
        match &self.channel {
            Channel::Nightly(Nightly { date }) => Some(date),
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

        let toolchain = Toolchain::new(channel, Target::host());

        assert!(&toolchain.channel().is_stable());
        assert_eq!(&toolchain.platform, &Target::host());
    }
}
