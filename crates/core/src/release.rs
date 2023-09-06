use crate::semver;
use std::cmp::Ordering;

/// A Rust release with an associated version.
///
/// A release may be associated with a channel (stable, beta, nightly).
///
/// **Breaking change for > 0.15*: After the release of `rust-releases` 0.15, a breaking change will
/// be made to add support for beta and nightly versions.  
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Release {
    // Fixme: for beta or nightly versions, dates should be used
    // Fixme: should be paired with a channel
    version: semver::Version,
}

impl Release {
    /// Construct a new stable release
    pub fn new_stable(version: semver::Version) -> Self {
        Self { version }
    }

    /// Whether this is a minor release
    pub fn is_minor(&self) -> bool {
        self.version.major != 0 && self.version.patch == 0 && self.version.build.is_empty()
    }

    /// Whether this is a patch release
    pub fn is_patch(&self) -> bool {
        self.version.patch != 0 && self.version.build.is_empty()
    }

    /// Get the Rust version for this release
    pub fn version(&self) -> &semver::Version {
        &self.version
    }
}

impl PartialOrd for Release {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Release {
    fn cmp(&self, other: &Self) -> Ordering {
        other.version().cmp(self.version())
    }
}
