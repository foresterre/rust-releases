use crate::source::DocumentSource;
use crate::strategy::from_manifests::dl::{fetch_meta_manifest, fetch_release_manifests};
use crate::strategy::from_manifests::meta_manifest::MetaManifest;
use crate::strategy::from_manifests::release_manifest::parse_release_manifest;
use crate::strategy::{FetchResources, Strategy};
use crate::{Channel, Release, ReleaseIndex, RustReleasesError, TResult};

mod dl;
mod meta_manifest;
mod release_manifest;

pub struct FromManifests {
    documents: Vec<DocumentSource>,
}

impl FromManifests {
    #[cfg(test)]
    pub(crate) fn from_documents<I: IntoIterator<Item = DocumentSource>>(iter: I) -> Self {
        Self {
            documents: iter.into_iter().collect(),
        }
    }
}

impl Strategy for FromManifests {
    fn build_index(&self) -> TResult<ReleaseIndex> {
        let releases = self
            .documents
            .iter()
            .map(|document| {
                document.load().and_then(|content| {
                    parse_release_manifest(&content).map(|version| Release::new(version))
                })
            })
            .collect::<TResult<Vec<_>>>()?;

        Ok(ReleaseIndex::new(releases))
    }
}

impl FetchResources for FromManifests {
    fn fetch_channel(channel: Channel) -> TResult<Self> {
        let source = fetch_meta_manifest()?;
        let content = source.load()?;
        let content =
            String::from_utf8(content).map_err(|_| RustReleasesError::ParseMetaManifest)?;

        let meta_manifest = MetaManifest::try_from_str(&content)?;

        let release_manifests = fetch_release_manifests(&meta_manifest, channel)?;

        Ok(Self {
            documents: release_manifests,
        })
    }
}
