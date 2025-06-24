use rust_release::rust_toolchain::channel::Stable;
use rust_release::toolchain::ReleaseToolchain;
use rust_release::{rust_toolchain, RustRelease};

pub struct MergeCandidate<'a, C> {
    // Double option because a RustRelease release_date is already an option, and it may be absent here
    pub release_date: Option<Option<&'a rust_toolchain::Date>>,
    pub toolchains: Option<&'a Vec<ReleaseToolchain>>,
    pub context: Option<&'a C>,
}

impl<C> Default for MergeCandidate<'_, C> {
    fn default() -> Self {
        Self {
            release_date: None,
            toolchains: None,
            context: None,
        }
    }
}

impl<'a, C> From<&'a Merge<C>> for MergeCandidate<'a, C> {
    fn from(mr: &'a Merge<C>) -> Self {
        Self {
            release_date: Some(mr.release_date.as_ref()),
            toolchains: Some(mr.toolchains.as_ref()),
            context: Some(&mr.context),
        }
    }
}

impl<'a, V, C> From<&'a RustRelease<V, C>> for MergeCandidate<'a, C> {
    fn from(rr: &'a RustRelease<V, C>) -> Self {
        Self {
            release_date: Some(rr.release_date.as_ref()),
            toolchains: Some(rr.toolchains.as_ref()),
            context: Some(&rr.context),
        }
    }
}

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
