use crate::source::Source;
use crate::TResult;
use std::iter::FromIterator;

/// A Rust version release of any channel (stable, beta, nightly)
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Release {
    version: semver::Version,
}

impl Release {
    pub(crate) fn new(version: semver::Version) -> Self {
        Self { version }
    }

    /// Whether this is a minor release
    pub fn is_minor(&self) -> bool {
        self.version.major != 0 && self.version.minor == 0 && self.version.patch == 0
    }

    /// Whether this is a patch release
    pub fn is_patch(&self) -> bool {
        self.version.patch != 0
    }

    /// Get the Rust version for this release
    pub fn version(&self) -> &semver::Version {
        &self.version
    }
}

/// A release index is a data structure holding known Rust releases.
/// Releases are indexed from the newest to the oldest known release.
#[derive(Debug)]
pub struct ReleaseIndex {
    index: Vec<Release>,
}

impl ReleaseIndex {
    /// Create a new `ReleaseIndex` from a given source.
    /// Releases available in the index may vary based on the type of `Source`.
    pub fn from_source<S: Source>(source: S) -> TResult<Self> {
        source.build_index()
    }

    pub fn releases(&self) -> Vec<&Release> {
        self.index.iter().collect()
    }
}

impl FromIterator<Release> for ReleaseIndex {
    /// Create a new `ReleaseIndex` from a given iterable.
    ///
    /// NB: Releases should already be sorted from the newest to the oldest known release.
    fn from_iter<T: IntoIterator<Item = Release>>(iter: T) -> Self {
        Self {
            index: iter.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::channel_manifests::ChannelManifests;
    use crate::source::Document;
    use yare::parameterized;

    #[parameterized(
        stable = { "/resources/channel_manifests/stable_2016-04-12.toml", "1.8.0" },
        beta = { "/resources/channel_manifests/beta_2016-03-23.toml", "1.8.0-beta.2" },
        nightly = { "/resources/channel_manifests/nightly_2016-03-08.toml", "1.9.0-nightly" },
    )]
    fn release_index(resource: &str, expected_version: &str) {
        let expected_version = semver::Version::parse(expected_version).unwrap();

        let path = [env!("CARGO_MANIFEST_DIR"), resource].join("");
        let strategy = ChannelManifests::from_documents(vec![Document::LocalPath(path.into())]);
        let index = ReleaseIndex::from_source(strategy).unwrap();

        assert_eq!(index.releases()[0].version(), &expected_version);
    }
}
