use crate::semver;
use std::cmp::Ordering;

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
