//! # rust-release
//!
//! The [`rust-release`] crate defines a set of types which model a Rust release.
//!
//! This project is part of the [`rust-releases`] and [`cargo-msrv`] projects.
//!
//! In case you have a feature request, question, bug, or have another reason to
//! contact the developers, please create a new issue at the `rust-releases`
//! [`repository`].
//!
//! [`rust-releases`]: https://github.com/foresterre/rust-releases
//! [`cargo-msrv`]: https://github.com/foresterre/cargo-msrv
//! [`repository`]: https://github.com/foresterre/rust-releases/issues
#![warn(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

use std::cmp::Ordering;

// exports
pub use rust_toolchain::channel::{Beta, Nightly, Stable};

/// A module for an unrefined Date type, solely used as a version number.
///
/// Do not use as your date type!
pub mod date {
    pub use rust_toolchain::Date;
}
/// Describes toolchains in so far they're relevant to a release
pub mod toolchain {
    pub use rust_toolchain::{Channel, Component, RustVersion, Target, Toolchain};
}

/// Describes the version of a release
pub mod version;

/// Type to model a Rust release.
///
/// # PartialEq, Eq, Ord, PartialOrd
///
/// With respect to the PartialEq, Eq, PartialOrd and Ord traits, a [`RustRelease`]
/// `a` is equal, less, or greater than a [`RustRelease`] `b` iff respectively the
/// `a.version` field is equal, less, or greater than `b.version`.
#[derive(Clone, Debug)]
pub struct RustRelease<V> {
    /// The version of a [`RustRelease`].
    ///
    /// The versioning scheme depends on the channel, which is why the version
    /// type is a generic. In this library, the `V` is always substituted by one
    /// of the following types: [`Stable`], [`Beta`] or [`Nightly`].
    ///
    /// [`Stable`] and [`Beta`] carry a semver version number, while [`Nightly`]
    /// is versioned by a date.
    pub version: V,
    /// The release date of the release.
    ///
    /// The field is optional, because the value may be absent from a data source.
    pub release_date: Option<date::Date>,
    /// The toolchains associated with the release.
    ///
    /// The field may be empty if toolchains were absent from a data source.
    pub toolchains: Vec<toolchain::Toolchain>,
    // pub context: C, // Eventually, I want to add this again which can be used to tag the release with arbitrary data
}

impl<V: PartialEq> PartialEq for RustRelease<V> {
    fn eq(&self, other: &Self) -> bool {
        self.version.eq(&other.version)
    }
}

impl<V: Eq> Eq for RustRelease<V> {}

impl<V: PartialOrd> PartialOrd for RustRelease<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.version.partial_cmp(&other.version)
    }
}

impl<V: Ord> Ord for RustRelease<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.version.cmp(&other.version)
    }
}

impl<V> RustRelease<V> {
    /// Create a new RustRelease instance using a version, optionally
    /// a release date, and an iterator of toolchains.
    pub fn new(
        version: V,
        release_date: Option<rust_toolchain::Date>,
        toolchains: impl IntoIterator<Item = toolchain::Toolchain>,
    ) -> Self {
        Self {
            version,
            release_date,
            toolchains: toolchains.into_iter().collect(),
        }
    }

    /// The version of a release.
    ///
    /// The 3 component MAJOR.MINOR.PATCH version number of the release
    pub fn version(&self) -> &V {
        &self.version
    }

    /// Release date of the Rust release, if known
    pub fn release_date(&self) -> Option<&date::Date> {
        self.release_date.as_ref()
    }

    /// Toolchains associated with the release
    pub fn toolchains(&self) -> impl Iterator<Item = &toolchain::Toolchain> {
        self.toolchains.iter()
    }
}

/// A combination of a channel and the version number.
///
/// For stable and beta releases, we have a three component MAJOR.MINOR.PATCH
/// version number. For nightly releases, we have a release date.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReleaseVersion {
    /// A stable channel release version
    Stable(Stable),
    /// A beta channel release version
    Beta(Beta),
    /// A nightly channel release version
    Nightly(Nightly),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolchain::Toolchain;
    use rust_toolchain::RustVersion;
    use std::collections::HashSet;

    fn fake(stable: Stable, date: Option<rust_toolchain::Date>) -> Toolchain {
        Toolchain::new(
            rust_toolchain::Channel::Stable(stable),
            date,
            rust_toolchain::Target::host(),
            HashSet::new(),
            HashSet::new(),
        )
    }

    #[test]
    fn can_instantiate() {
        let stable = Stable {
            version: RustVersion::new(1, 82, 0),
        };
        let version = ReleaseVersion::Stable(stable.clone());
        let release = RustRelease::new(version, None, vec![fake(stable.clone(), None)]);

        assert_eq!(release.version(), &ReleaseVersion::Stable(stable));
    }

    #[yare::parameterized(
        some = { Some(rust_toolchain::Date::new(2024, 1, 1)) },
        none = { None },
    )]
    fn can_instantiate_deux(date: Option<rust_toolchain::Date>) {
        let stable = Stable {
            version: RustVersion::new(1, 82, 0),
        };
        let version = ReleaseVersion::Stable(stable.clone());
        let release = RustRelease::new(version, date.clone(), vec![fake(stable, date)]);

        let target_date = release.toolchains().next().unwrap().date();

        assert_eq!(release.release_date(), target_date);
    }
}
