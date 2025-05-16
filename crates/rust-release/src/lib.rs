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
// #![deny(missing_docs)]
#![warn(clippy::all)]
#![deny(unsafe_code)]
// #![deny(missing_docs)]

use crate::toolchain::ReleaseToolchain;
use std::cmp::Ordering;

// exports
pub use rust_toolchain;
pub use rust_toolchain::channel::{Beta, Nightly, Stable};

/// Describes toolchains in so far they're relevant to a release
pub mod toolchain;

/// Describes the version of a release
pub mod version;

/// Type to model a Rust release.
///
/// # PartialEq, Eq, Ord, PartialOrd
///
/// With respect to the PartialEq, Eq, PartialOrd and Ord traits, a [`RustRelease`]
/// is only compared and ordered based on its `version`.
#[derive(Clone, Debug)]
pub struct RustRelease<V, C = ()> {
    pub version: V,
    pub release_date: Option<rust_toolchain::Date>,
    pub toolchains: Vec<ReleaseToolchain>,
    pub context: C,
}

impl<V: PartialEq, C> PartialEq for RustRelease<V, C> {
    fn eq(&self, other: &Self) -> bool {
        self.version.eq(&other.version)
    }
}

impl<V: Eq, C> Eq for RustRelease<V, C> {}

impl<V: PartialOrd, C> PartialOrd for RustRelease<V, C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.version.partial_cmp(&other.version)
    }
}

impl<V: Ord, C> Ord for RustRelease<V, C> {
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
        toolchains: impl IntoIterator<Item = ReleaseToolchain>,
    ) -> Self {
        Self {
            version,
            release_date,
            toolchains: toolchains.into_iter().collect(),
            context: (),
        }
    }

    /// The 3 component MAJOR.MINOR.PATCH version number of the release
    pub fn version(&self) -> &V {
        &self.version
    }

    /// Release date of the Rust release, if known
    pub fn release_date(&self) -> Option<&rust_toolchain::Date> {
        self.release_date.as_ref()
    }

    /// Toolchains associated with the release
    pub fn toolchains(&self) -> impl Iterator<Item = &ReleaseToolchain> {
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
    use std::collections::HashSet;

    #[test]
    fn can_instantiate() {
        let stable_version = Stable {
            version: rust_toolchain::RustVersion::new(1, 82, 0),
        };
        let version = ReleaseVersion::Stable(stable_version.clone());

        let release = RustRelease::new(
            version,
            None,
            vec![ReleaseToolchain::new(
                rust_toolchain::Toolchain::new(
                    rust_toolchain::Channel::Stable(stable_version.clone()),
                    None,
                    rust_toolchain::Target::host(),
                    HashSet::new(),
                    HashSet::new(),
                ),
                toolchain::TargetTier::Unknown,
            )],
        );

        let release_version = release.version();
        assert_eq!(release_version, &ReleaseVersion::Stable(stable_version));
    }

    #[yare::parameterized(
        some = { Some(rust_toolchain::Date::new(2024, 1, 1)) },
        none = { None },
    )]
    fn can_instantiate_deux(date: Option<rust_toolchain::Date>) {
        let stable_version = rust_toolchain::channel::Stable {
            version: rust_toolchain::RustVersion::new(1, 82, 0),
        };
        let version = ReleaseVersion::Stable(stable_version.clone());

        let release = RustRelease::new(
            version,
            date.clone(),
            vec![ReleaseToolchain::new(
                rust_toolchain::Toolchain::new(
                    rust_toolchain::Channel::Stable(stable_version.clone()),
                    date,
                    rust_toolchain::Target::host(),
                    HashSet::new(),
                    HashSet::new(),
                ),
                toolchain::TargetTier::Unknown,
            )],
        );

        let release_date = release.release_date();
        let target_date = release.toolchains().next().unwrap().toolchain().date();

        assert_eq!(release_date, target_date);
    }
}
