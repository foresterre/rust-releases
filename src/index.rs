use crate::dl::DocumentSource;
use crate::release_manifest::parse_release_manifest;
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
    use crate::dl::{fetch_meta_manifest, fetch_release_manifests};
    use crate::manifests::MetaManifest;
    use crate::release_channel::Channel;

    #[test]
    fn test_parse_meta_manifest() {
        let meta_file = fetch_meta_manifest().unwrap();
        let buffer = meta_file.load().unwrap();
        let meta_manifest = MetaManifest::try_from_str(String::from_utf8(buffer).unwrap()).unwrap();
        let documents = fetch_release_manifests(&meta_manifest, Channel::Stable).unwrap();
        let index = ReleaseIndex::try_from_documents(&documents);

        assert!(index.is_ok());
    }
}
