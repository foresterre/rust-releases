pub use crate::index::linear::StableReleaseIterator;
use crate::source::Source;
use crate::TResult;
use std::cmp::Ordering;
use std::iter::FromIterator;

/// Module for a linear iterator.
pub(crate) mod linear;

/// A Rust version release of any channel (stable, beta, nightly)
#[derive(Debug, Eq, PartialEq)]
pub struct Release {
    version: semver::Version,
}

impl Release {
    pub fn new(version: semver::Version) -> Self {
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

    /// Returns a vector with releases.
    pub fn releases(&self) -> Vec<&Release> {
        self.index.iter().collect()
    }

    /// Returns the most recent release.
    ///
    /// Returns `None` if the index has not registered any release.
    pub fn most_recent(&self) -> Option<&Release> {
        self.index.first()
    }

    /// Returns the least recent (oldest) registered release.
    ///
    /// Returns `None` if the index has not registered any release.
    pub fn least_recent(&self) -> Option<&Release> {
        self.index.last()
    }

    /// Returns an iterator over the latest stable releases, where only the latest
    /// patch release is returned.
    ///
    /// # Example
    ///
    /// If the index is aware of the following releases: `["0.9.0", "1.0.0", "1.0.1", "1.1.0"]`,
    /// the iterator will return, in order, releases for the following versions:
    /// 1) `"1.1.0"`, 2) `"1.0.1"`, 3) `"0.9.0"`
    ///
    /// Version `"1.0.0"` is skipped, since this iterator only returns the latest patch release for
    /// a version.
    pub fn stable_releases_iterator(&self) -> impl Iterator<Item = &Release> {
        StableReleaseIterator {
            iter: self.index.iter().peekable(),
        }
    }

    /// Returns an iterator over the latest stable releases, where only the latest
    /// patch release is returned.
    ///
    /// # Example
    ///
    /// If the index is aware of the following releases: `["0.9.0", "1.0.0", "1.0.1", "1.1.0"]`,
    /// the iterator will return, in order, releases for the following versions:
    /// 1) `"1.1.0"`, 2) `"1.0.1"`, 3) `"1.0.0"`, 4) `"0.9.0"`.
    pub fn all_releases_iterator(&self) -> impl Iterator<Item = &Release> {
        self.index.iter()
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
    use crate::source::{Document, RustChangelog};
    use yare::parameterized;

    fn setup_index() -> ReleaseIndex {
        let path = [
            env!("CARGO_MANIFEST_DIR"),
            "/resources/rust_changelog/RELEASES.md",
        ]
        .join("");
        let source = RustChangelog::from_document(Document::LocalPath(path.into()));
        ReleaseIndex::from_source(source).unwrap()
    }

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

    #[test]
    fn most_recent() {
        let index = setup_index();

        let recent = index.most_recent();
        assert_eq!(recent.unwrap().version(), &semver::Version::new(1, 50, 0));
    }

    #[test]
    fn least_recent() {
        let index = setup_index();

        let recent = index.least_recent();
        assert_eq!(recent.unwrap().version(), &semver::Version::new(0, 11, 0));
    }

    #[test]
    fn stable_releases_iterator() {
        let index = setup_index();

        let releases = index.stable_releases_iterator().collect::<Vec<_>>();

        assert_eq!(releases.len(), 53);
        assert_eq!(releases[0].version(), &semver::Version::new(1, 50, 0));
        assert_eq!(releases[5].version(), &semver::Version::new(1, 45, 2));
        assert_eq!(releases[10].version(), &semver::Version::new(1, 40, 0));
        assert_eq!(releases[20].version(), &semver::Version::new(1, 30, 1));
        assert_eq!(releases[50].version(), &semver::Version::new(1, 0, 0));
        assert_eq!(releases[52].version(), &semver::Version::new(0, 11, 0));
    }

    #[test]
    fn all_releases_iterator() {
        let index = setup_index();

        let releases = index.all_releases_iterator().collect::<Vec<_>>();

        assert_eq!(releases.len(), 74);
        assert_eq!(releases[0].version(), &semver::Version::new(1, 50, 0));
        assert_eq!(releases[5].version(), &semver::Version::new(1, 45, 2));
        assert_eq!(releases[10].version(), &semver::Version::new(1, 43, 1));
        assert_eq!(releases[20].version(), &semver::Version::new(1, 35, 0));
        assert_eq!(releases[50].version(), &semver::Version::new(1, 17, 0));
        assert_eq!(releases[73].version(), &semver::Version::new(0, 11, 0));
    }
}
