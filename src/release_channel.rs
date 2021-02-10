use crate::ManifestaError;
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Channel {
    Beta,
    Nightly,
    Stable,
}

impl<'a> TryFrom<&'a str> for Channel {
    type Error = ManifestaError;

    fn try_from(item: &'a str) -> Result<Self, Self::Error> {
        Ok(match item {
            "beta" => Self::Beta,
            "nightly" => Self::Nightly,
            "stable" => Self::Stable,
            unsupported => return Err(ManifestaError::NoSuchChannel(unsupported.to_string())),
        })
    }
}

impl<'a> From<Channel> for &'a str {
    fn from(channel: Channel) -> Self {
        match channel {
            Channel::Beta => "beta",
            Channel::Stable => "stable",
            Channel::Nightly => "nightly",
        }
    }
}
