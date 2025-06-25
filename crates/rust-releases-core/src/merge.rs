use rust_release::toolchain::ReleaseToolchain;
use rust_release::{rust_toolchain, RustRelease};

/// A merge candidate is a [`RustRelease`] minus the version, and all fields are optional
/// because they may not be present for a specific release source type.
/// E.g. if the releases are constructed from the GitHub releases repo, there may
/// be insufficient information about the available toolchains, while that information
/// does exist in the Rust release S3 bucket.
///
/// For example, if releases from these two sources are merged into one, the
/// release metadata obtained from Rust's S3 bucket may be used to fill out that
/// missing piece of release information.
///
/// For TypeScript developers, this type is essentially `Partial<Omit<MergeCandidate, 'version'>>` ;).
///
/// [`RustRelease`]: RustRelease
pub struct MergeCandidate<C> {
    // Double option because a RustRelease release_date is already an option, and it may be absent here
    // TODO: Maybe it should just be a single option? isn't the absence the same regardless of whether it happened in the source or not?
    //       use case: I feel like I'm always using .flatten() now anyways in resolvers
    pub release_date: Option<Option<rust_toolchain::Date>>,
    pub toolchains: Option<Vec<ReleaseToolchain>>,
    pub context: Option<C>,
}

impl<C> Default for MergeCandidate<C> {
    fn default() -> Self {
        Self {
            release_date: None,
            toolchains: None,
            context: None,
        }
    }
}

impl<C> From<Merge<C>> for MergeCandidate<C> {
    fn from(mr: Merge<C>) -> Self {
        Self {
            release_date: Some(mr.release_date),
            toolchains: Some(mr.toolchains),
            context: Some(mr.context),
        }
    }
}

impl<V, C> From<RustRelease<V, C>> for MergeCandidate<C> {
    fn from(rr: RustRelease<V, C>) -> Self {
        Self {
            release_date: Some(rr.release_date),
            toolchains: Some(rr.toolchains),
            context: Some(rr.context),
        }
    }
}

/// A merge candidate is a [`RustRelease`] minus the version.
///
/// For TypeScript developers, this type is essentially `Omit<MergeCandidate, 'version'>` ;).
///
/// [`RustRelease`]: RustRelease
pub struct Merge<C> {
    pub release_date: Option<rust_toolchain::Date>,
    pub toolchains: Vec<ReleaseToolchain>,
    pub context: C,
}

impl<C> Merge<C> {
    pub fn to_version<V: Clone>(self, version: &V) -> RustRelease<V, C> {
        RustRelease {
            version: version.clone(),
            release_date: self.release_date,
            toolchains: self.toolchains,
            context: self.context,
        }
    }
}
