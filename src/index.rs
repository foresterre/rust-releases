use crate::dl::DocumentSource;
use crate::strategy::release_manifests::release_manifest::parse_release_manifest;
use crate::TResult;
use std::io::Write;

pub use semver;

/// A Rust version release of any channel (stable, beta, nightly)
#[derive(Debug)]
pub struct Release {
    version: semver::Version,
}

impl Release {
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
    /// Attempt to build an index by parsing release manifests
    pub fn try_from_documents(documents: &[DocumentSource]) -> TResult<Self> {
        let releases = documents
            .iter()
            .map(|document| {
                document.load().and_then(|content| {
                    parse_release_manifest(&content).map(|version| Release { version })
                })
            })
            .collect::<TResult<Vec<_>>>()?;

        Ok(Self { releases })
    }

    // TODO
    #[allow(unused)]
    pub(crate) fn try_from_index(_buffer: &[u8]) -> TResult<Self> {
        unimplemented!()
    }

    // TODO
    #[allow(unused)]
    pub(crate) fn write_index<W: Write>(&self, _writer: &mut W) -> TResult<()> {
        //writer.write_all(&vec![]).map_err(From::from)
        unimplemented!()
    }

    /// Access all releases for this release index bundle
    pub fn releases(&self) -> &[Release] {
        &self.releases
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yare::parameterized;

    #[parameterized(
        stable = { "/resources/stable_2016-04-12.toml", "1.8.0" },
        beta = { "/resources/beta_2016-03-23.toml", "1.8.0-beta.2" },
        nightly = { "/resources/nightly_2016-03-08.toml", "1.9.0-nightly" },
    )]
    fn release_index(resource: &str, expected_version: &str) {
        let expected_version = semver::Version::parse(expected_version).unwrap();

        let path = [env!("CARGO_MANIFEST_DIR"), resource].join("");
        let index = ReleaseIndex::try_from_documents(&vec![DocumentSource::LocalPath(path.into())])
            .unwrap();

        assert_eq!(index.releases()[0].version(), &expected_version);
    }
}
