use crate::channel::Channel;
use crate::source::{DocumentSource, DEFAULT_MEMORY_SIZE};
use crate::strategy::from_manifests::meta_manifest::{ManifestSource, MetaManifest};
use crate::{RustReleasesError, TResult};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

const META_MANIFEST: &str = "https://static.rust-lang.org/manifests.txt";
// 1 day timeout for the meta manifest
const META_MANIFEST_STALENESS_TIMEOUT: Duration = Duration::from_secs(86_400);
// 1 year timeout for the individual release manifests (these manifests should not get outdated)
const RELEASE_MANIFEST_STALENESS_TIMEOUT: Duration = Duration::from_secs(31_557_600);

/// Download the meta manifest, unless it exists in the cache and is not stale
pub fn fetch_meta_manifest() -> TResult<DocumentSource> {
    let cache = cache_dir()?;
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
pub fn fetch_release_manifests(
    meta_manifest: &MetaManifest,
    channel: Channel,
) -> TResult<Vec<DocumentSource>> {
    let sources = meta_manifest.manifests();
    let cache = cache_dir()?;

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

fn download_if_not_stale<P: AsRef<Path>>(
    url: &str,
    cache_dir: &Path,
    manifest: P,
    timeout: Duration,
) -> TResult<DocumentSource> {
    let manifest_path = cache_dir.join(manifest);

    if manifest_path.exists() && !is_stale(&manifest_path, timeout)? {
        return Ok(DocumentSource::LocalPath(manifest_path));
    } else {
        std::fs::create_dir_all(cache_dir)?;
    }

    let response = attohttpc::get(url)
        .header(
            "User-Agent",
            "rust-releases (github.com/foresterre/rust-releases/issues)",
        )
        .send()?;

    // write to memory
    let mut memory = Vec::with_capacity(DEFAULT_MEMORY_SIZE);
    response.write_to(&mut memory)?;

    // write memory to disk
    let mut file = std::fs::File::create(&manifest_path)?;
    let mut writer = BufWriter::new(&mut file);
    writer.write_all(&memory)?;

    Ok(DocumentSource::RemoteCached(manifest_path, memory))
}

fn is_stale<P: AsRef<Path>>(manifest: P, timeout: Duration) -> TResult<bool> {
    let metadata = std::fs::metadata(manifest)?;
    let modification = metadata.modified()?;
    let duration = modification.elapsed()?;

    Ok(timeout < duration)
}

fn manifest_file_name(source: &ManifestSource) -> String {
    format!(
        "{}_{}.toml",
        Into::<&'static str>::into(source.channel()),
        source.date()
    )
}

fn cache_dir() -> TResult<PathBuf> {
    let cache = directories_next::ProjectDirs::from("com", "ilumeo", "rust-releases")
        .ok_or(RustReleasesError::DlCache)?;
    let cache = cache.cache_dir();
    let cache = cache.join("index");

    Ok(cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    macro_rules! dl_test {
        ($expr:expr) => {{
            if cfg!(feature = "dl_test") || option_env!("RUST_RELEASES_RUN_DL_TEST").is_some() {
                $expr
            }
        }};
    }

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
