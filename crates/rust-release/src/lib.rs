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

// exports
pub use rust_toolchain::channel::{Beta, Nightly, Stable};
use std::cmp;
use std::fmt::Debug;

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
// The fields are have a `pub` privacy so they can be pattern patched on
#[derive(Clone, Debug)]
pub struct RustRelease<V: Debug, C = ()> {
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
    /// Arbitrary extra data
    pub context: C,
}

impl<V: PartialEq + Debug, C> PartialEq for RustRelease<V, C> {
    fn eq(&self, other: &Self) -> bool {
        self.version.eq(&other.version)
    }
}

impl<V: Eq + Debug, C> Eq for RustRelease<V, C> {}

impl<V: PartialOrd + Debug, C> PartialOrd for RustRelease<V, C> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.version.partial_cmp(&other.version)
    }
}

impl<V: Ord + Debug, C> Ord for RustRelease<V, C> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.version.cmp(&other.version)
    }
}

impl<V: Debug> RustRelease<V, ()> {
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
            context: (),
        }
    }
}

impl<V: Debug, C> RustRelease<V, C> {
    /// Create a new RustRelease instance using a version, optionally
    /// a release date, an iterator of toolchains and an "arbitrary" context.
    ///
    /// The context is can contain any additional data you want to store with the previously mentioned
    /// data fields, i.e. the version, release date and toolchains. For example, you could add a struct
    /// which contains metadata about when and where the data was fetched from, or which contains
    /// checksums or signatures.
    pub fn new_with_context(
        version: V,
        release_date: Option<rust_toolchain::Date>,
        toolchains: impl IntoIterator<Item = toolchain::Toolchain>,
        context: C,
    ) -> Self {
        Self {
            version,
            release_date,
            toolchains: toolchains.into_iter().collect(),
            context,
        }
    }
    /// A shared reference to version of a release.
    ///
    /// # See also
    ///
    /// Commonly `V` is parameterized by one of these:
    ///
    /// * [`Stable`]
    /// * [`Beta`]
    /// * [`Nightly`]
    pub fn version(&self) -> &V {
        &self.version
    }

    /// An exclusive reference to version of a release.
    ///
    /// # See also
    ///
    /// Commonly `V` is parameterized by one of these:
    ///
    /// * [`Stable`]
    /// * [`Beta`]
    /// * [`Nightly`]
    pub fn version_mut(&mut self) -> &mut V {
        &mut self.version
    }

    /// A shared reference to the release date of a release, if set.
    pub fn release_date(&self) -> Option<&date::Date> {
        self.release_date.as_ref()
    }

    /// An exclusive reference to the release date of a release, if set.
    pub fn release_date_mut(&mut self) -> Option<&mut date::Date> {
        self.release_date.as_mut()
    }

    /// A shared reference to the toolchains associated with the release.
    pub fn toolchains(&self) -> &Vec<toolchain::Toolchain> {
        &self.toolchains
    }

    /// An exclusive reference to the toolchains associated with the release.
    pub fn toolchains_mut(&mut self) -> &mut Vec<toolchain::Toolchain> {
        &mut self.toolchains
    }

    /// Iterator over the toolchains associated with the release.
    pub fn toolchains_iter(&self) -> impl Iterator<Item = &toolchain::Toolchain> {
        self.toolchains.iter()
    }

    /// A shared reference to the (added) context of this release.
    pub fn context(&self) -> &C {
        &self.context
    }

    /// An exclusive reference to the (added) context of this release.
    pub fn context_mut(&mut self) -> &mut C {
        &mut self.context
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

    // Create a fake toolchain model
    fn fake_tc(stable: Stable, date: Option<rust_toolchain::Date>) -> Toolchain {
        Toolchain::new(
            rust_toolchain::Channel::Stable(stable),
            date,
            rust_toolchain::Target::host(),
            HashSet::new(),
            HashSet::new(),
        )
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
        let release = RustRelease::new(version, date.clone(), vec![fake_tc(stable, date)]);

        let target_date = release.toolchains_iter().next().unwrap().date();

        assert_eq!(release.release_date(), target_date);
    }

    #[test]
    fn version() {
        let stable = Stable::new(1, 82, 0);
        let release = RustRelease::new(stable.clone(), None, vec![fake_tc(stable.clone(), None)]);

        assert_eq!(release.version(), &stable);
    }

    #[test]
    fn version_mut() {
        let stable = Stable::new(1, 82, 0);
        let mut release =
            RustRelease::new(stable.clone(), None, vec![fake_tc(stable.clone(), None)]);

        assert_eq!(release.version(), &stable);
        let replacement = Stable::new(9, 9, 9);
        *release.version_mut() = replacement.clone();

        assert_eq!(release.version(), &replacement);
    }

    #[test]
    fn release_date() {
        let stable = Stable::new(1, 82, 0);
        let date = rust_toolchain::Date::new(2026, 12, 12);
        let release = RustRelease::new(
            stable.clone(),
            Some(date.clone()),
            vec![fake_tc(stable.clone(), Some(date.clone()))],
        );

        assert_eq!(release.release_date().unwrap(), &date);
    }

    #[test]
    fn release_date_mut() {
        let stable = Stable::new(1, 82, 0);
        let date = rust_toolchain::Date::new(2026, 12, 12);
        let mut release = RustRelease::new(
            stable.clone(),
            Some(date.clone()),
            vec![fake_tc(stable.clone(), Some(date.clone()))],
        );

        assert_eq!(release.release_date().unwrap(), &date);
        let replacement = rust_toolchain::Date::new(2026, 05, 22);
        release.release_date_mut().replace(&mut replacement.clone());

        assert_eq!(release.release_date().unwrap(), &date);
    }

    #[test]
    fn toolchains() {
        let stable1 = Stable::new(1, 82, 0);
        let stable2 = Stable::new(1, 83, 0);
        let date = rust_toolchain::Date::new(2026, 12, 12);
        // doesn't really make sense to put different versions in the toolchains vec, but for this test
        // it is enough, and theoretically it would be possible.
        let toolchains = vec![
            fake_tc(stable1.clone(), Some(date.clone())),
            fake_tc(stable2.clone(), None),
        ];

        let release = RustRelease::new(stable1.clone(), None, toolchains.clone());

        assert_eq!(release.toolchains(), &toolchains);
    }

    #[test]
    fn toolchains_mut() {
        let stable1 = Stable::new(1, 82, 0);
        let stable2 = Stable::new(1, 83, 0);
        let date = rust_toolchain::Date::new(2026, 12, 12);
        // doesn't really make sense to put different versions in the toolchains vec, but for this test
        // it is enough, and theoretically it would be possible.
        let toolchains = vec![
            fake_tc(stable1.clone(), Some(date.clone())),
            fake_tc(stable2.clone(), None),
        ];
        let mut release = RustRelease::new(stable1.clone(), None, toolchains.clone());

        assert_eq!(release.toolchains(), &toolchains);

        let stable3 = Stable::new(9, 9, 9);
        let extension = fake_tc(stable3, None);
        release.toolchains_mut().push(extension.clone());

        let expected = [toolchains, vec![extension]].concat();
        assert_eq!(release.toolchains(), &expected);
    }

    #[test]
    fn context() {
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        enum Checksum {
            Crc32(u32),
        }

        struct MyContext {
            checksum: Checksum,
        }

        let stable = Stable::new(1, 82, 0);
        let date = rust_toolchain::Date::new(2026, 12, 12);
        let release = RustRelease::new_with_context(
            stable.clone(),
            None,
            vec![fake_tc(stable.clone(), Some(date.clone()))],
            MyContext {
                checksum: Checksum::Crc32(0x00000000),
            },
        );

        assert_eq!(release.context().checksum, Checksum::Crc32(0x00000000));
    }

    #[test]
    fn context_mut() {
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        enum Checksum {
            Crc32(u32),
        }

        struct MyContext {
            checksum: Checksum,
        }

        let stable = Stable::new(1, 82, 0);
        let date = rust_toolchain::Date::new(2026, 12, 12);
        let mut release = RustRelease::new_with_context(
            stable.clone(),
            None,
            vec![fake_tc(stable.clone(), Some(date.clone()))],
            MyContext {
                checksum: Checksum::Crc32(0x00000000),
            },
        );

        assert_eq!(release.context().checksum, Checksum::Crc32(0x00000000));

        let replacement = MyContext {
            checksum: Checksum::Crc32(0xFFFFFFFF),
        };
        *release.context_mut() = replacement;

        assert_eq!(release.context.checksum, Checksum::Crc32(0xFFFFFFFF));
    }

    #[test]
    fn pattern_match() {
        let stable = Stable::new(1, 82, 0);
        let release = RustRelease::new(stable.clone(), None, vec![fake_tc(stable.clone(), None)]);

        let RustRelease {
            version,
            toolchains,
            context,
            ..
        } = release;

        assert_eq!(version, stable);
        assert_eq!(toolchains.len(), 1);
        assert_eq!(context, ());
    }
}
