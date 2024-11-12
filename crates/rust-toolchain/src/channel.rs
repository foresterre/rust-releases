mod beta;
mod nightly;
mod stable;

use crate::{ReleaseDate, RustVersion};

pub use beta::Beta;
pub use nightly::Nightly;
pub use stable::Stable;

/// A Rust [`release channel`].
///
/// Does not include the once used `Alpha` release channel, which has not been used post `1.0.0`.
///
/// # Variants
///
/// See also: [`Stable`], [`Beta`] and [`Nightly`], and [`ChannelKind`] which can be used
/// if the channel needs only be described by its identifier.
///
/// [`release channel`]: https://forge.rust-lang.org/#current-release-versions
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Channel {
    /// The stable release channel
    Stable(Stable),
    /// The beta release channel
    Beta(Beta),
    /// The nightly release channel
    Nightly(Nightly),
}

impl Channel {
    /// Create a new [`Stable`] channel instance.
    pub fn stable(version: RustVersion) -> Self {
        Channel::Stable(Stable { version })
    }

    /// Create a new [`Beta`] channel instance.
    pub fn beta(version: RustVersion) -> Self {
        Channel::Beta(Beta { version })
    }

    /// Create a new [`Nightly`] channel instance.
    pub fn nightly(date: ReleaseDate) -> Self {
        Channel::Nightly(Nightly { date })
    }

    /// Whether the given [`Channel`] is of the [`Stable`] variant.
    pub fn is_stable(&self) -> bool {
        matches!(self, Self::Stable(_))
    }

    /// Whether the given [`Channel`] is of the [`Beta`] variant.
    pub fn is_beta(&self) -> bool {
        matches!(self, Self::Beta(_))
    }

    /// Whether the given [`Channel`] is of the [`Nightly`] variant.
    pub fn is_nightly(&self) -> bool {
        matches!(self, Self::Nightly(_))
    }
}

/// A description of a channel, only describing a channel by its name.
///
/// See also: [`Channel`] which includes version information.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum ChannelKind {
    Stable,
    Beta,
    Nightly,
}

impl ChannelKind {
    /// Create a new instance from a given str.
    ///
    /// Must be lowercase 'stable', 'beta' or 'nightly'.
    pub fn try_from_str(channel: &str) -> Result<Self, ()> {
        match channel {
            "stable" => Ok(Self::Stable),
            "beta" => Ok(Self::Beta),
            "nightly" => Ok(Self::Nightly),
            _ => Err(()),
        }
    }

    /// Whether the given [`ChannelKind`] is of the `Stable` variant.
    pub fn is_stable(&self) -> bool {
        matches!(self, Self::Stable)
    }

    /// Whether the given [`ChannelKind`] is of the `Beta` variant.
    pub fn is_beta(&self) -> bool {
        matches!(self, Self::Beta)
    }

    /// Whether the given [`ChannelKind`] is of the `Nightly` variant.
    pub fn is_nightly(&self) -> bool {
        matches!(self, Self::Nightly)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_channel_stable() {
        let stable = Channel::stable(RustVersion::new(1, 2, 3));

        assert!(stable.is_stable());
        assert!(!stable.is_beta());
        assert!(!stable.is_nightly());
    }

    #[test]
    fn create_channel_beta() {
        let stable = Channel::beta(RustVersion::new(1, 2, 3));

        assert!(!stable.is_stable());
        assert!(stable.is_beta());
        assert!(!stable.is_nightly());
    }

    #[test]
    fn create_channel_nightly() {
        let stable = Channel::nightly(ReleaseDate::new(1, 1, 1));

        assert!(!stable.is_stable());
        assert!(!stable.is_beta());
        assert!(stable.is_nightly());
    }
}
