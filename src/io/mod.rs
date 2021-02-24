use crate::source::{DocumentSource, DEFAULT_MEMORY_SIZE};
use crate::{RustReleasesError, TResult};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

#[cfg(test)]
#[macro_export]
macro_rules! dl_test {
    ($expr:expr) => {{
        if cfg!(feature = "dl_test") || option_env!("RUST_RELEASES_RUN_DL_TEST").is_some() {
            $expr
        }
    }};
}

pub(crate) fn download_if_not_stale<P: AsRef<Path>>(
    url: &str,
    cache_dir: &Path,
    resource: P,
    timeout: Duration,
) -> TResult<DocumentSource> {
    let manifest_path = cache_dir.join(resource);

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

pub(crate) fn is_stale<P: AsRef<Path>>(manifest: P, timeout: Duration) -> TResult<bool> {
    let metadata = std::fs::metadata(manifest)?;
    let modification = metadata.modified()?;
    let duration = modification.elapsed()?;

    Ok(timeout < duration)
}

pub(crate) fn base_cache_dir() -> TResult<PathBuf> {
    let cache = directories_next::ProjectDirs::from("com", "ilumeo", "rust-releases")
        .ok_or(RustReleasesError::DlCache)?;
    let cache = cache.cache_dir();

    Ok(cache.to_path_buf())
}
