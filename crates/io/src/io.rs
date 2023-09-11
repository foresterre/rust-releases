use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{io, time};

/// Determines whether a stored resource is stale
pub fn is_stale<P: AsRef<Path>>(path: P, timeout: Duration) -> Result<bool, IsStaleError> {
    let metadata = std::fs::metadata(path).map_err(IsStaleError::ReadMetadata)?;
    let modification = metadata
        .modified()
        .map_err(IsStaleError::ReadModificationDate)?;
    let duration = modification
        .elapsed()
        .map_err(IsStaleError::ElapsedSinceModified)?;

    Ok(timeout < duration)
}

/// Returned in case the staleness check [`is_stale`] faults.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum IsStaleError {
    /// Returned when the staleness check could not be completed because the
    /// cache file metadata could not be read.
    #[error("Failed to check if file is stale: unable to read metadata ({0})")]
    ReadMetadata(io::Error),

    /// Returned when the staleness check could not be completed because the
    /// cache file modification time could not be read.
    #[error("Failed to check if file is stale: unable to read modification date ({0})")]
    ReadModificationDate(io::Error),

    /// Returned when the staleness check could not be completed because the
    /// modification date of the cache file is more recent than the current system time.
    /// The modification date should not be in the future.
    #[error("Failed to check if file is stale: modification date is more recent than the current system time ({0})")]
    ElapsedSinceModified(time::SystemTimeError),
}

/// The default cache dir used by `rust-releases` crates
pub fn base_cache_dir() -> Result<PathBuf, BaseCacheDirError> {
    let cache = directories_next::ProjectDirs::from("com", "ilumeo", "rust-releases")
        .ok_or(BaseCacheDirError)?;
    let cache = cache.cache_dir();

    Ok(cache.to_path_buf())
}

/// Returned when the base cache folder is used, but can not be located.
#[derive(Debug, thiserror::Error)]
#[error("Unable to locate base cache folder")]
pub struct BaseCacheDirError;
