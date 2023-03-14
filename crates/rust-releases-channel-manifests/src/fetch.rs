use crate::meta_manifest::{ManifestSource, MetaManifest};
use crate::{ChannelManifestsError, ChannelManifestsResult};
use rust_releases_core::Channel;
use rust_releases_io::{
    base_cache_dir, CachedClient, Document, ResourceFile, RetrievedDocument, RustReleasesClient,
};
use std::path::PathBuf;
use std::time::Duration;

// URL to meta manifest
const META_MANIFEST: &str = "https://static.rust-lang.org/manifests.txt";
// 1 day timeout for the meta manifest
const META_MANIFEST_STALENESS_TIMEOUT: Duration = Duration::from_secs(86_400);
// Max duration timeout for the individual release manifests (these manifests should not get outdated)
const RELEASE_MANIFEST_STALENESS_TIMEOUT: Duration = Duration::from_secs(u64::MAX);
// Directory where cached files reside for this source
const SOURCE_CACHE_DIR: &str = "source_channel_manifests";

/// Download the meta manifest, unless it exists in the cache and is not stale
pub(crate) fn fetch_meta_manifest() -> ChannelManifestsResult<Document> {
    let cache = from_manifests_cache_dir()?;

    let client = CachedClient::new(cache, META_MANIFEST_STALENESS_TIMEOUT);
    let resource_file = ResourceFile::new(META_MANIFEST, "manifests.txt");

    let retrieved_document = client.fetch(resource_file)?;

    Ok(retrieved_document.into_document())
}

/// Download the the release manifests for a certain channel, unless they exists in the cache and
/// are not stale
pub(crate) fn fetch_release_manifests(
    meta_manifest: &MetaManifest,
    channel: Channel,
) -> ChannelManifestsResult<Vec<Document>> {
    let sources = meta_manifest.manifests();
    let cache = from_manifests_cache_dir()?;

    let client = CachedClient::new(cache, RELEASE_MANIFEST_STALENESS_TIMEOUT);

    let manifests = sources
        .iter()
        .filter(|source| source.channel() == channel)
        .map(|source| {
            let manifest = manifest_file_name(source);
            let resource_file = ResourceFile::new(source.url(), &manifest);

            client
                .fetch(resource_file)
                .map_err(ChannelManifestsError::CachedClient)
                .map(RetrievedDocument::into_document)
        })
        .collect::<ChannelManifestsResult<Vec<Document>>>()?;

    Ok(manifests)
}

fn from_manifests_cache_dir() -> ChannelManifestsResult<PathBuf> {
    let cache = base_cache_dir()?;
    Ok(cache.join(SOURCE_CACHE_DIR))
}

fn manifest_file_name(source: &ManifestSource) -> String {
    format!(
        "{}_{}.toml",
        Into::<&'static str>::into(source.channel()),
        source.date()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_meta_manifest() {
        __internal_dl_test!({
            let meta = fetch_meta_manifest();
            assert!(meta.is_ok());
        })
    }

    #[test]
    fn test_fetch_release_manifest_stable() {
        __internal_dl_test!({
            let meta = fetch_meta_manifest().unwrap();
            let content = std::str::from_utf8(meta.buffer()).unwrap();

            let meta_manifest = MetaManifest::try_from_str(content).unwrap();
            let result = fetch_release_manifests(&meta_manifest, Channel::Stable);

            assert!(result.is_ok());
        })
    }
}
