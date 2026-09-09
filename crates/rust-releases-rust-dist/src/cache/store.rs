use crate::cache::codec::{self, Version};
use crate::cache::error::CachedDistError;
use crate::manifest::ReleaseManifest;
use crate::releases::ReleaseCollection;
use rust_releases_io::{IoError, is_stale};
use std::fs;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::Duration;

pub fn cached_releases<R, E>(
    cache_file: &Path,
    cache_timeout: Duration,
) -> Result<Option<R>, CachedDistError<E>>
where
    R: ReleaseCollection,
    R::Version: Version,
{
    match cached(cache_file, cache_timeout)? {
        Some(buffer) => codec::decode(&buffer)
            .map(Some)
            .map_err(CachedDistError::CacheEntry),
        None => Ok(None),
    }
}

pub fn cached_manifest<E>(
    cache_file: &Path,
    cache_timeout: Duration,
) -> Result<Option<ReleaseManifest>, CachedDistError<E>> {
    match cached(cache_file, cache_timeout)? {
        Some(buffer) => ReleaseManifest::parse(&buffer).map(Some).map_err(|source| {
            CachedDistError::ReleaseManifest {
                path: cache_file.to_path_buf(),
                source,
            }
        }),
        None => Ok(None),
    }
}

pub fn store<E>(cache_file: &Path, buffer: &[u8]) -> Result<(), CachedDistError<E>> {
    if let Some(cache_folder) = cache_file.parent() {
        fs::create_dir_all(cache_folder)
            .map_err(|error| IoError::inaccessible(error, cache_folder.to_path_buf()))?;
    }

    let file = fs::File::create(cache_file)
        .map_err(|error| IoError::inaccessible(error, cache_file.to_path_buf()))?;

    let mut writer = BufWriter::new(file);
    writer
        .write_all(buffer)
        .map_err(|error| IoError::inaccessible(error, cache_file.to_path_buf()))?;
    writer
        .flush()
        .map_err(|error| IoError::inaccessible(error, cache_file.to_path_buf()))?;

    Ok(())
}

fn cached<E>(
    cache_file: &Path,
    cache_timeout: Duration,
) -> Result<Option<Vec<u8>>, CachedDistError<E>> {
    if !cache_file.is_file() || is_stale(cache_file, cache_timeout)? {
        return Ok(None);
    }

    let buffer = fs::read(cache_file)
        .map_err(|error| IoError::inaccessible(error, cache_file.to_path_buf()))?;

    Ok(Some(buffer))
}
