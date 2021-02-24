use crate::strategy::Strategy;
use crate::TResult;

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

/// A release index is a data structure holding known Rust releases
#[derive(Debug)]
pub struct ReleaseIndex {
    releases: Vec<Release>,
}

impl ReleaseIndex {
    pub(crate) fn new<I: IntoIterator<Item = Release>>(releases: I) -> Self {
        Self {
            releases: releases.into_iter().collect(),
        }
    }

    /// Attempt to build an index using a certain given strategy
    pub fn with_strategy<S: Strategy>(strategy: S) -> TResult<Self> {
        strategy.build_index()
    }

    /// Access all releases for this release index bundle
    pub fn releases(&self) -> &[Release] {
        &self.releases
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::DocumentSource;
    use crate::strategy::from_manifests::FromManifests;
    use yare::parameterized;

    #[parameterized(
        stable = { "/resources/stable_2016-04-12.toml", "1.8.0" },
        beta = { "/resources/beta_2016-03-23.toml", "1.8.0-beta.2" },
        nightly = { "/resources/nightly_2016-03-08.toml", "1.9.0-nightly" },
    )]
    fn release_index(resource: &str, expected_version: &str) {
        let expected_version = semver::Version::parse(expected_version).unwrap();

        let path = [env!("CARGO_MANIFEST_DIR"), resource].join("");
        let strategy = FromManifests::from_documents(vec![DocumentSource::LocalPath(path.into())]);
        let index = ReleaseIndex::with_strategy(strategy).unwrap();

        assert_eq!(index.releases()[0].version(), &expected_version);
    }
}
