/// A Rust release channel
///
/// Alpha releases, which are no longer used, are unsupported.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Channel {
    /// The stable release channel
    Stable,
    /// The beta release channel
    Beta,
    /// The nightly release channel
    Nightly,
}

impl Channel {
    pub fn is_stable(&self) -> bool {
        matches!(self, Self::Stable)
    }

    pub fn is_beta(&self) -> bool {
        matches!(self, Self::Beta)
    }

    pub fn is_nightly(&self) -> bool {
        matches!(self, Self::Nightly)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_channel_stable() {
        let stable = Channel::Stable;

        assert!(stable.is_stable());
        assert!(!stable.is_beta());
        assert!(!stable.is_nightly());
    }

    #[test]
    fn create_channel_beta() {
        let stable = Channel::Beta;

        assert!(!stable.is_stable());
        assert!(stable.is_beta());
        assert!(!stable.is_nightly());
    }

    #[test]
    fn create_channel_nightly() {
        let stable = Channel::Nightly;

        assert!(!stable.is_stable());
        assert!(!stable.is_beta());
        assert!(stable.is_nightly());
    }
}
