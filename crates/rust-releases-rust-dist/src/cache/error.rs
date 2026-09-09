use crate::cache::codec::CacheEntryError;
use crate::manifest::ReleaseManifestError;
use rust_releases_io::{IoError, IsStaleError};
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CachedDistError<E> {
    #[error("Failed to read the Rust distribution: {0}")]
    Client(#[source] E),

    #[error(transparent)]
    Io(#[from] IoError),

    #[error(transparent)]
    IsStale(#[from] IsStaleError),

    #[error("Failed to read the cached releases: {0}")]
    CacheEntry(#[source] CacheEntryError),

    #[error("Failed to read the cached release manifest '{}': {source}", path.display())]
    ReleaseManifest {
        path: PathBuf,
        #[source]
        source: ReleaseManifestError,
    },
}
