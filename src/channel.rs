use crate::RustReleasesError;
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Channel {
    Beta,
    Nightly,
    Stable,
}

impl<'a> TryFrom<&'a str> for Channel {
    type Error = RustReleasesError;

    fn try_from(item: &'a str) -> Result<Self, Self::Error> {
        Ok(match item {
            "beta" => Self::Beta,
            "nightly" => Self::Nightly,
            "stable" => Self::Stable,
            unsupported => return Err(RustReleasesError::NoSuchChannel(unsupported.to_string())),
        })
    }
}

impl<'a> From<Channel> for &'a str {
    fn from(channel: Channel) -> Self {
        match channel {
            Channel::Beta => "beta",
            Channel::Nightly => "nightly",
            Channel::Stable => "stable",
        }
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
