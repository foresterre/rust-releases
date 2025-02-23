mod beta;
mod nightly;
mod stable;

use crate::{Date, RustVersion};

pub use beta::Beta;
pub use nightly::Nightly;
pub use stable::Stable;

/// A Rust release [`channel`].
///
/// Does not include the once used `Alpha` release channel, which has not been used post `1.0.0`.
///
/// # Variants
///
/// See also: [`Stable`], [`Beta`] and [`Nightly`].
///
/// # Reading materials
///
/// - [`rustup concepts: channels`]
/// - [`rust book: how rust is made`]
/// - [`rust forge: rust release channel layout`]
/// - [`rust forge: current rust release versions`]
///
///
/// [`channel`]: https://forge.rust-lang.org/#current-release-versions
/// [`rustup concepts: channels`]: https://rust-lang.github.io/rustup/concepts/channels.html
/// [`rust book: how rust is made`]: https://doc.rust-lang.org/book/appendix-07-nightly-rust.html
/// [`rust forge: rust release channel layout`]: https://forge.rust-lang.org/infra/channel-layout.html#the-rust-release-channel-layout
/// [`rust forge: current rust release versions`]: https://forge.rust-lang.org/#current-release-versions
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
        Channel::Beta(Beta {
            version,
            prerelease: None,
        })
    }

    /// Create a new [`Nightly`] channel instance.
    pub fn nightly(date: Date) -> Self {
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

    /// Returns the release version, or None, if it's a nightly release.
    pub fn version(&self) -> Option<RustVersion> {
        match self {
            Channel::Stable(v) => Some(v.version),
            Channel::Beta(v) => Some(v.version),
            Channel::Nightly(_) => None,
        }
    }

    /// Returns the release date, or
    pub fn date(&self) -> Option<Date> {
        match self {
            Channel::Stable(_) => None,
            Channel::Beta(_) => None,
            Channel::Nightly(v) => Some(v.date.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Date, RustVersion};
    use yare::parameterized;

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
        let stable = Channel::nightly(Date::new(1, 1, 1));

        assert!(!stable.is_stable());
        assert!(!stable.is_beta());
        assert!(stable.is_nightly());
    }

    #[parameterized(
        stable = { Channel::stable("1.2.3".parse().unwrap()), Some(RustVersion::new(1,2,3)) },
        beta = { Channel::beta("1.2.3".parse().unwrap()), Some(RustVersion::new(1,2,3)) },
        nightly = { Channel::nightly(Date::new(2024, 1, 1)), None }
    )]
    fn version(c: Channel, expected: Option<RustVersion>) {
        assert_eq!(c.version(), expected)
    }
}
