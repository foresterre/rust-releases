use crate::CoreError;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

/// Enumerates the Rust release channels
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Channel {
    /// An identifier for the `beta` release channel
    Beta,
    /// An identifier for the `nightly` release channel
    Nightly,
    /// An identifier for the `stable` release channel
    Stable,
}

impl TryFrom<&str> for Channel {
    type Error = CoreError;

    fn try_from(item: &str) -> Result<Self, Self::Error> {
        Ok(match item {
            "beta" => Self::Beta,
            "nightly" => Self::Nightly,
            "stable" => Self::Stable,
            unsupported => return Err(CoreError::NoSuchChannel(unsupported.to_string())),
        })
    }
}

impl From<Channel> for &str {
    fn from(channel: Channel) -> Self {
        match channel {
            Channel::Beta => "beta",
            Channel::Nightly => "nightly",
            Channel::Stable => "stable",
        }
    }
}

impl Display for Channel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let channel: Channel = *self;
        f.write_str(channel.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yare::parameterized;

    #[parameterized(
        beta = { "beta", Channel::Beta },
        nightly = { "nightly", Channel::Nightly },
        stable = { "stable", Channel::Stable },
    )]
    fn channel_from_str(input: &str, expected: Channel) {
        assert_eq!(Channel::try_from(input).unwrap(), expected);
    }

    #[parameterized(
        beta = { Channel::Beta, "beta" },
        nightly = { Channel::Nightly, "nightly" },
        stable = { Channel::Stable, "stable" },
    )]
    fn channel_into_str(input: Channel, expected: &str) {
        assert_eq!(Into::<&str>::into(input), expected);
    }
}
