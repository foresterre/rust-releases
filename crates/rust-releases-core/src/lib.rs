//! Defines the core routines required to implement a [`Source`].
//!
//! Please, see the [`rust-releases`] for additional documentation on how this crate can be used.
//!
//! [`Source`]: crate::Source
//! [`rust-releases`]: https://docs.rs/rust-releases
#![allow(missing_docs)]
#![deny(clippy::all)]
#![deny(unsafe_code)]

pub use crate::releases::{BetaReleases, NightlyReleases, StableReleases};
use rust_release::date::Date;
use rust_release::toolchain::Toolchain;
/// Defines release channels, such as the stable, beta and nightly release channels.
pub use rust_release::{self, Beta, Nightly, RustRelease, Stable};

pub mod channel;
pub mod merge;
pub mod releases;

pub struct RustReleases {
    stable: StableReleases,
    beta: BetaReleases,
    nightly: NightlyReleases,
}

impl Default for RustReleases {
    fn default() -> Self {
        Self {
            stable: StableReleases::default(),
            beta: BetaReleases::default(),
            nightly: NightlyReleases::default(),
        }
    }
}

impl RustReleases {
    /// Iterate over set of stable releases
    pub fn stable(&self) -> impl IntoIterator<Item = &RustRelease<Stable>> {
        self.stable.iter()
    }

    /// Iterate over set of beta releases
    pub fn beta(&self) -> impl IntoIterator<Item = &RustRelease<Beta>> {
        self.beta.iter()
    }

    /// Iterate over set of nightly releases
    pub fn nightly(&self) -> impl IntoIterator<Item = &RustRelease<Nightly>> {
        self.nightly.iter()
    }
}

/// A `PartialRustRelease` is like a [`RustRelease`] minus the version, and all fields are optional
/// because they may not be present for a specific release source type.
/// E.g. if the releases are constructed from the GitHub releases repo, there may
/// be insufficient information about the available toolchains, while that information
/// does exist in the Rust release S3 bucket.
///
/// For example, if releases from these two sources are merged into one, the
/// release metadata obtained from Rust's S3 bucket may be used to fill out that
/// missing piece of release information.
///
/// For TypeScript developers, this type is essentially `Partial<Omit<RustRelease, 'version'>>` ;).
///
/// [`RustRelease`]: RustRelease
#[derive(Debug, Default)]
pub struct PartialRustRelease {
    pub release_date: Option<Date>,
    pub toolchains: Option<Vec<Toolchain>>,
}

impl<V> From<RustRelease<V>> for PartialRustRelease {
    fn from(rr: RustRelease<V>) -> Self {
        Self {
            release_date: rr.release_date,
            toolchains: Some(rr.toolchains),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::resolver::{ConflictResolutionBuilder, ReleaseDateResolver, ToolchainsResolver};
    use crate::StableReleases;
    use rust_release::RustRelease;
    use rust_release::Stable;

    #[test]
    fn empty_merge_is_empty() {
        let left = StableReleases::default();
        let right = StableReleases::default();

        let merge = left.merge_with(right, |version, _lhs, _rhs| RustRelease {
            version,
            release_date: None,
            toolchains: vec![],
        });

        assert!(merge.is_empty());
        assert!(merge.is_empty());
        assert!(merge.is_empty());
    }

    #[test]
    fn base() {
        let mut left = StableReleases::default();
        left.add(RustRelease::new(Stable::new(1, 2, 0), None, []));
        let right = StableReleases::default();

        let combine = ConflictResolutionBuilder::default()
            .with_release_date_resolver(ReleaseDateResolver::most_recent())
            .with_toolchains_resolver(ToolchainsResolver::chain())
            .build_or_default();
        let out = left.merge_with(right, combine);

        assert_eq!(out.len(), 1);

        let first = out.iter().next().unwrap();

        assert_eq!(first.version(), &Stable::new(1, 2, 0));
        assert_eq!(first.release_date(), None);
        assert_eq!(first.toolchains.len(), 0);
    }
}
