use crate::{IoError, IoResult};
use std::path::{Path, PathBuf};
use std::time::Duration;

#[cfg(feature = "http_client")]
use crate::{document::DEFAULT_MEMORY_SIZE, Document};

#[cfg(feature = "http_client")]
/// Download a resource assuming it's not stale
pub fn download_if_not_stale<P: AsRef<Path>>(
    url: &str,
    cache_dir: &Path,
    resource_path: P,
    timeout: Duration,
) -> IoResult<Document> {
    use attohttpc as http_client;
    use std::io::{BufWriter, Write};

    let manifest_path = cache_dir.join(resource_path);

    if manifest_path.exists() && !is_stale(&manifest_path, timeout)? {
        return Ok(Document::LocalPath(manifest_path));
    } else {
        std::fs::create_dir_all(cache_dir)?;
    }

    let response = http_client::get(url)
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

    Ok(Document::RemoteCached(manifest_path, memory))
}

/// Determines whether a stored resource is stale
pub fn is_stale<P: AsRef<Path>>(path: P, timeout: Duration) -> IoResult<bool> {
    let metadata = std::fs::metadata(path)?;
    let modification = metadata.modified()?;
    let duration = modification.elapsed()?;

    Ok(timeout < duration)
}

/// The default cache dir used by `rust-releases` crates
pub fn base_cache_dir() -> IoResult<PathBuf> {
    let cache = directories_next::ProjectDirs::from("com", "ilumeo", "rust-releases")
        .ok_or(IoError::DlCache)?;
    let cache = cache.cache_dir();

    Ok(cache.to_path_buf())
}
