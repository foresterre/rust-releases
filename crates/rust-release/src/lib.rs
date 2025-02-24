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
#![deny(missing_docs)]

/// Type to model a Rust release.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RustRelease {
    version: ReleaseVersion,
    release_date: Option<rust_toolchain::Date>,
    toolchains: Vec<ExtendedToolchain>,
}

impl RustRelease {
    /// Create a new RustRelease instance using a version, optionally
    /// a release date, and an iterator of toolchains.
    pub fn new(
        version: ReleaseVersion,
        release_date: Option<rust_toolchain::Date>,
        toolchains: impl IntoIterator<Item = ExtendedToolchain>,
    ) -> Self {
        Self {
            version,
            release_date,
            toolchains: toolchains.into_iter().collect(),
        }
    }

    /// The 3 component MAJOR.MINOR.PATCH version number of the release
    pub fn version(&self) -> &ReleaseVersion {
        &self.version
    }

    /// Release date of the Rust release, if known
    pub fn release_date(&self) -> Option<&rust_toolchain::Date> {
        self.release_date.as_ref()
    }

    /// Toolchains associated with the release
    pub fn toolchains(&self) -> impl Iterator<Item = &ExtendedToolchain> {
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
    Stable(rust_toolchain::channel::Stable),
    /// A beta channel release version
    Beta(rust_toolchain::channel::Beta),
    /// A nightly channel release version
    Nightly(rust_toolchain::channel::Nightly),
}

/// Type to model a Rust toolchain, with additional metadata relevant to a
/// release.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtendedToolchain {
    toolchain: rust_toolchain::Toolchain,
    tier: TargetTier,
}

impl ExtendedToolchain {
    /// Create an ExtendedToolchain from a rust_toolchain::Toolchain
    pub fn new(toolchain: rust_toolchain::Toolchain, tier: TargetTier) -> Self {
        Self { toolchain, tier }
    }

    /// Get the toolchain
    pub fn toolchain(&self) -> &rust_toolchain::Toolchain {
        &self.toolchain
    }

    /// Get the toolchain tier
    pub fn tier(&self) -> TargetTier {
        self.tier
    }
}

/// Support tier for a target
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum TargetTier {
    /// Tier 1 target
    T1,
    /// Tier 2 target
    T2,
    /// Tier 2.5 target
    T2_5,
    /// Tier 3 target
    T3,
    /// Tier is unknown
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn can_instantiate() {
        let stable_version = rust_toolchain::channel::Stable {
            version: rust_toolchain::RustVersion::new(1, 82, 0),
        };
        let version = ReleaseVersion::Stable(stable_version.clone());

        let release = RustRelease::new(
            version,
            None,
            vec![ExtendedToolchain::new(
                rust_toolchain::Toolchain::new(
                    rust_toolchain::Channel::Stable(stable_version.clone()),
                    None,
                    rust_toolchain::Target::host(),
                    HashSet::new(),
                    HashSet::new(),
                ),
                TargetTier::Unknown,
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
            vec![ExtendedToolchain::new(
                rust_toolchain::Toolchain::new(
                    rust_toolchain::Channel::Stable(stable_version.clone()),
                    date,
                    rust_toolchain::Target::host(),
                    HashSet::new(),
                    HashSet::new(),
                ),
                TargetTier::Unknown,
            )],
        );

        let release_date = release.release_date();
        let target_date = release.toolchains().next().unwrap().toolchain().date();

        assert_eq!(release_date, target_date);
    }
}
