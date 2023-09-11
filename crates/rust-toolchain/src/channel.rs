use crate::{ReleaseDate, RustVersion};

/// A Rust release channel
///
/// Alpha releases, which are no longer used, are unsupported.
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
    /// Create a new stable channel instance.
    pub fn stable(version: RustVersion) -> Self {
        Channel::Stable(Stable { version })
    }

    /// Create a new beta channel instance.
    pub fn beta(version: RustVersion) -> Self {
        Channel::Beta(Beta { version })
    }

    /// Create a new nightly channel instance.
    pub fn nightly(date: ReleaseDate) -> Self {
        Channel::Nightly(Nightly { date })
    }
}

impl Channel {
    pub fn is_stable(&self) -> bool {
        matches!(self, Self::Stable(_))
    }

    pub fn is_beta(&self) -> bool {
        matches!(self, Self::Beta(_))
    }

    pub fn is_nightly(&self) -> bool {
        matches!(self, Self::Nightly(_))
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Stable {
    pub version: RustVersion,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Beta {
    pub version: RustVersion,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Nightly {
    pub date: ReleaseDate,
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
