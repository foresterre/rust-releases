use rust_release::date::Date;
use rust_release::toolchain::TargetToolchain;
use rust_release::RustRelease;

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
    pub toolchains: Option<Vec<TargetToolchain>>,
}

impl<V> From<RustRelease<V>> for PartialRustRelease {
    fn from(rr: RustRelease<V>) -> Self {
        Self {
            release_date: rr.release_date,
            toolchains: Some(rr.toolchains),
        }
    }
}
