use crate::{Channel, Platform, ReleaseDate, RustVersion};

mod rustup_toolchain;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Toolchain {
    pub channel: Channel,
    pub date: ReleaseDate,
    pub platform: Platform,
    pub version: Option<RustVersion>,
}

impl Toolchain {
    /// Construct a new toolchain.
    ///
    /// A toolchain consists of a `channel`, a `release date`, a platform
    pub fn new(
        channel: Channel,
        date: ReleaseDate,
        platform: Platform,
        version: Option<RustVersion>,
    ) -> Self {
        Self {
            channel,
            date,
            platform,
            version,
        }
    }

    /// The version of the toolchain, if any.
    pub fn version(&self) -> Option<&RustVersion> {
        self.version.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_toolchain() {
        let toolchain = Toolchain::new(
            Channel::Stable,
            ReleaseDate::new(20, 1, 1),
            Platform::host(),
            None,
        );

        assert_eq!(&toolchain.channel, &Channel::Stable);
        assert_eq!(&toolchain.date, &ReleaseDate::new(20, 1, 1));
        assert_eq!(&toolchain.platform, &Platform::host());
        assert!(toolchain.version.is_none());
    }
}
