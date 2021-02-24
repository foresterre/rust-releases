use crate::channel::Channel;
use crate::io::{base_cache_dir, download_if_not_stale};
use crate::source::DocumentSource;
use crate::strategy::from_manifests::meta_manifest::{ManifestSource, MetaManifest};
use crate::TResult;
use std::path::PathBuf;
use std::time::Duration;

const META_MANIFEST: &str = "https://static.rust-lang.org/manifests.txt";
// 1 day timeout for the meta manifest
const META_MANIFEST_STALENESS_TIMEOUT: Duration = Duration::from_secs(86_400);
// 1 year timeout for the individual release manifests (these manifests should not get outdated)
const RELEASE_MANIFEST_STALENESS_TIMEOUT: Duration = Duration::from_secs(31_557_600);

/// Download the meta manifest, unless it exists in the cache and is not stale
pub(in crate::strategy::from_manifests) fn fetch_meta_manifest() -> TResult<DocumentSource> {
    let cache = from_manifests_cache_dir()?;
    let manifest = download_if_not_stale(
        META_MANIFEST,
        &cache,
        "manifests.txt",
        META_MANIFEST_STALENESS_TIMEOUT,
    )?;

    Ok(manifest)
}

/// Download the the release manifests for a certain channel, unless they exists in the cache and
/// are not stale
pub(in crate::strategy::from_manifests) fn fetch_release_manifests(
    meta_manifest: &MetaManifest,
    channel: Channel,
) -> TResult<Vec<DocumentSource>> {
    let sources = meta_manifest.manifests();
    let cache = from_manifests_cache_dir()?;

    let manifests = sources
        .iter()
        .filter(|source| source.channel() == channel)
        .map(|source| {
            let manifest = manifest_file_name(source);

            download_if_not_stale(
                source.url(),
                &cache,
                manifest,
                RELEASE_MANIFEST_STALENESS_TIMEOUT,
            )
        })
        .collect::<TResult<Vec<DocumentSource>>>()?;

    Ok(manifests)
}

fn from_manifests_cache_dir() -> TResult<PathBuf> {
    let cache = base_cache_dir()?;
    Ok(cache.join("index"))
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
    use crate::dl_test;

    #[test]
    fn test_fetch_meta_manifest() {
        dl_test!({
            let meta = fetch_meta_manifest();
            assert!(meta.is_ok());
        })
    }

    #[test]
    fn test_fetch_release_manifest_stable() {
        dl_test!({
            let meta = fetch_meta_manifest().unwrap();
            let meta_manifest =
                MetaManifest::try_from_str(String::from_utf8(meta.load().unwrap()).unwrap())
                    .unwrap();

            let result = fetch_release_manifests(&meta_manifest, Channel::Stable);

            assert!(result.is_ok());
        })
    }
}
