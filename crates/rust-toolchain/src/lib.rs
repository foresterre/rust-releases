//! # rust-toolchain
//!
//! The `rust-toolchain` crate defines a set of types which describe a Rust toolchain.
//! While there is no definitive spec which defines what a "Rust toolchain" is,
//! we try to follow the official Rust release process as closely as possible.
//! The [`rustup`] project has written down a rough specification for [`toolchains`] used
//! by the Rust project. In the initial version, we will follow this spec, but disregard custom
//! toolchains altogether, in the name of simplicity.
//!
//! This project is part of the [`rust-releases`] and [`cargo-msrv`] projects.
//! In case you have a feature request, question, bug, or have another reason to contact the developers,
//! please, create a new issue at the `rust-releases` [`repository`].
//!
//! [`rustup`]: https://github.com/rust-lang/rustup
//! [`toolchains`]: https://rust-lang.github.io/rustup/concepts/toolchains.html
//! [`rust-releases`]: https://github.com/foresterre/rust-releases
//! [`cargo-msrv`]: https://github.com/foresterre/cargo-msrv
//! [`repository`]: https://github.com/foresterre/rust-releases/issues
// #![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(unsafe_code)]

use std::str::FromStr;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Toolchain {
    pub channel: Channel,
    pub date: ReleaseDate,
    pub host: TargetPlatform,
    // Different from Channel::Versioned, since stable and beta releases
    // also have a version. Maybe enumify?
    version: Option<RustVersion>,
}

impl Toolchain {
    pub fn new(
        channel: Channel,
        date: ReleaseDate,
        host: TargetPlatform,
        version: RustVersion,
    ) -> Self {
        Self {
            channel,
            date,
            host,
            version,
        }
    }
}

pub struct RustupToolchain {
    toolchain: Toolchain,
}

impl RustupToolchain {
    pub fn active() -> Option<Self> {
        todo!()
    }

    pub fn installed() -> Vec<Self> {
        todo!()
    }
}

impl From<RustupToolchain> for Toolchain {
    fn from(value: RustupToolchain) -> Self {
        value.toolchain
    }
}

/// A Rust release channel
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Channel {
    /// The stable release channel
    Stable,
    /// The beta release channel
    Beta,
    /// The nightly release channel
    Nightly,
    /// A versioned release
    ///
    /// Various tools accepts both a two component (`major.minor`) and three component
    /// (`major.minor.patch`) version. Toolchains have however always a three component version,
    /// so versioned only accepts the full three component version. If you want to access
    /// toolchains by their two component version, consider using a higher level interface,
    /// like [TODO](todo!).
    Versioned(RustVersion),
}

/// A three component, `major.minor.patch` version number.
///
/// This version number is a subset of [semver](https://semver.org/spec/v2.0.0.html), except that
/// it only accepts the numeric MAJOR, MINOR and PATCH components, while pre-release and build
/// metadata, and other labels, are rejected.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RustVersion {
    version: version_number::MajorMinorPatch,
}

/// A release date of the form `YYYY-MM-DD`.
///
/// It is up to the implementer to ensure that a constructed date is valid.
/// E.g. this date may accept `2023-02-30`, while February only has 28 or 29 days in
/// the Gregorian calendar.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseDate {
    date: DateImpl,
}

impl ReleaseDate {
    /// Create a new `ReleaseDate` instance.
    ///
    /// It is up to the caller to make sure that the given date is valid.
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Self {
            date: DateImpl { year, month, day },
        }
    }
}

/// A compact date consisting of a four number year, and a two number month and day.
/// Up to the caller to ensure it matches with their reality of a 'valid date'.
///
/// Not intended to be compatible with common date standards
#[derive(Clone, Debug, Eq, PartialEq)]
struct DateImpl {
    year: u16,
    month: u8,
    day: u8,
}

/// The platform of a toolchain.
///
/// Commonly represented as a [`target triple`]. A target triple consists of three (or four) components: the
/// architecture component, the vendor component, the operating system component and optionally
/// a fourth component representing the environment (e.g. gnu or msvc).
///
/// [`target triple`]: https://github.com/rust-lang/rfcs/blob/master/text/0131-target-specification.md#detailed-design
// Extra information may be found here:
// - https://doc.rust-lang.org/rustc/platform-support.html
// - https://rust-lang.github.io/rustup/concepts/toolchains.html
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TargetPlatform {
    platform: target_lexicon::Triple,
}

impl TargetPlatform {
    pub const fn host() -> Self {
        Self {
            // todo: don't unwrap =)
            platform: target_lexicon::HOST,
        }
    }

    pub fn from_target_triple(triple: &str) -> Self {
        Self {
            // todo: don't unwrap =)
            platform: target_lexicon::Triple::from_str(triple).expect("invalid target triple"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Channel, ReleaseDate, TargetPlatform, Toolchain};

    #[test]
    fn test() {
        let toolchain = Toolchain {
            channel: Channel::Stable,
            date: ReleaseDate::new(20, 01, 01),
            host: TargetPlatform::host(),
        };

        assert_eq!(&toolchain.channel, &Channel::Stable);
    }
}
