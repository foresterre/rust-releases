use crate::source::Source;
use crate::TResult;
use std::iter;
use std::iter::FromIterator;

/// A Rust version release of any channel (stable, beta, nightly)
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
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

    /// Returns an iterator over the latest stable releases, where only the latest
    /// patch release is returned.
    ///
    /// If, for example, there are three patch releases for a minor release with version `1.5.x`
    /// (`1.5.0`, `1.5.1` and `1.5.2`), this iterator will only return `1.5.2` and skip over the other
    /// two patch releases.
    pub fn stable_releases_iterator<'release>(
        &'release self,
    ) -> StableReleaseIterator<impl Iterator<Item = &'release Release>> {
        StableReleaseIterator {
            iter: self.index.iter().peekable(),
        }
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

/// An iterator over the latest regular releases.
///
/// Here, a regular release is the latest patch version for a certain minor release version.
pub struct StableReleaseIterator<'release, I: Iterator<Item = &'release Release>> {
    iter: iter::Peekable<I>,
}

impl<'release, I: Iterator<Item = &'release Release>> Iterator
    for StableReleaseIterator<'release, I>
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.iter.next();

        current.map(|it| {
            let minor = it.version().minor;

            while let Some(release) = self.iter.peek() {
                if release.version().minor == minor {
                    self.iter.next();
                } else {
                    break;
                }
            }

            it
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::channel_manifests::ChannelManifests;
    use crate::source::{Document, RustChangelog};
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

    #[test]
    fn stable_releases_iterator() {
        let path = [
            env!("CARGO_MANIFEST_DIR"),
            "/resources/rust_changelog/RELEASES.md",
        ]
        .join("");
        let source = RustChangelog::from_document(Document::LocalPath(path.into()));
        let index = ReleaseIndex::from_source(source).unwrap();

        let releases = index.stable_releases_iterator().collect::<Vec<_>>();

        assert_eq!(releases.len(), 53);
        assert_eq!(releases[0].version(), &semver::Version::new(1, 50, 0));
        assert_eq!(releases[5].version(), &semver::Version::new(1, 45, 2));
        assert_eq!(releases[10].version(), &semver::Version::new(1, 40, 0));
        assert_eq!(releases[20].version(), &semver::Version::new(1, 30, 1));
        assert_eq!(releases[50].version(), &semver::Version::new(1, 0, 0));
        assert_eq!(releases[52].version(), &semver::Version::new(0, 11, 0));
    }
}
