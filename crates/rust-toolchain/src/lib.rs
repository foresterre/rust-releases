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
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(unsafe_code)]
#![allow(mixed_script_confusables)]

#[cfg(test)]
mod tests;

/// The release channel for a given toolchain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Channel {
    /// The `stable` release channel.
    Stable,
    /// The `beta` release channel.
    Beta,
    /// The `nightly` release channel.
    Nightly,
}

impl<'s> std::convert::TryFrom<&'s str> for Channel {
    type Error = crate::Error;

    fn try_from(value: &'s str) -> Result<Self, Self::Error> {
        Ok(match value {
            "stable" => Self::Stable,
            "beta" => Self::Beta,
            "nightly" => Self::Nightly,
            elsy => return Err(crate::Error::UnknownChannel(elsy.to_string())),
        })
    }
}

/// Resolve the missing parts of a `ToolchainSpec`, and turn it into a `ResolvedToolchainSpec`
pub trait ResolveToolchain: ResolveHost {
    /// Consume the `ToolchainSpec`, resolve the missing parts, and turn it into a `ResolvedToolchainSpec`.
    fn resolve_toolchain(
        &self,
        toolchain: ToolchainSpec,
    ) -> Result<ResolvedToolchainSpec, crate::Error>;
}

/// Find the target triple of the current platform.
pub trait ResolveHost {
    /// Find the current platform host
    fn resolve_host(&self) -> Result<Host, crate::Error>;
}

/// A resolved toolchain consists of a set of tools and components which make up a Rust compiler distribution,
/// but requires the toolchain to be resolved, i.e. targeted to a specific host and platform.
pub struct ResolvedToolchain {
    spec: ResolvedToolchainSpec,
    components: Option<Vec<Component>>,
}

/// A toolchain consists of a set of tools and components which make up a Rust compiler distribution.
#[derive(Debug)]
pub struct Toolchain {
    spec: ToolchainSpec,
    components: Option<Vec<Component>>,
}

/// A completed toolchain specification, i.e. without unresolved parts.
#[derive(Debug)]
pub enum ResolvedToolchainSpec {
    /// A `stable` toolchain has a version and host.
    Stable {
        /// A complete semver compatible version.
        version: semver::Version,
        /// The target triple of a stable release.
        host: Host,
    },
    /// A `beta` toolchain has a version and host.
    Beta {
        /// A complete semver compatible version.
        version: semver::Version,
        /// The target triple of a beta release.
        host: Host,
    },
    /// A `nightly` toolchain has a date and host.
    Nightly {
        /// The date component of a nightly release.
        date: Date,
        /// The target triple of a nightly release.
        host: Host,
    },
}

/// The toolchain specifier can be used to identify a toolchain, excluding it's components.
#[derive(Debug)]
pub struct ToolchainSpec {
    channel: ReleaseChannel,
    date: Option<Date>,
    host: Option<Host>,
}

impl<'s> std::convert::TryFrom<&'s str> for ToolchainSpec {
    type Error = crate::Error;

    fn try_from(_value: &'s str) -> Result<Self, Self::Error> {
        todo!()
    }
}

/// The release channel can either be a channel, or, a specific version.
#[derive(Debug)]
pub enum ReleaseChannel {
    /// An unversioned release channel, such as `stable` or `nightly`.
    Channel(Channel),
    /// A version release, such as `1.56.0`.
    Version(semver::Version), // we infer a major.minor version and store it as a ReleaseChannel::Version
}

/// The release date of a toolchain.
#[derive(Debug)]
pub struct Date {
    time: time::Date,
}

/// The host target of a toolchain.
#[derive(Debug)]
pub struct Host {
    triple: String,
}

/// A component which is available to a toolchain.
#[derive(Debug)]
pub struct Component {
    name: String,
}

/// Errors returned for various bad weather paths.
#[derive(Debug, thiserror::Error, Eq, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// Returned in case a ToolchainSpec could not be parsed from a given string.
    #[error("Unable to determine a valid toolchain spec from '{0}'")]
    CantDetermineToolchainSpec(String),

    /// Returned in case a channel is given, which is not known to be a valid Rust release channel.
    #[error("Unknown channel '{0}'")]
    UnknownChannel(String),
}
