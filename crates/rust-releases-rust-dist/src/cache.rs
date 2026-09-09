use crate::client::{AsyncDistIndexClient, DistIndexClient};
use rust_releases_io::{BoxFuture, Document, IoError, IsStaleError, is_stale};
use std::fs;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

// The index does not change very often, and downloading it takes a thousand or so requests, so a
// downloaded index is kept on disk, and re-used until it goes stale.
#[derive(Clone, Debug)]
pub struct CachedDistIndexClient<C> {
    client: C,
    cache_file: PathBuf,
    cache_timeout: Duration,
}

impl<C> CachedDistIndexClient<C> {
    pub fn new(client: C, cache_file: PathBuf, cache_timeout: Duration) -> Self {
        Self {
            client,
            cache_file,
            cache_timeout,
        }
    }

    pub fn client(&self) -> &C {
        &self.client
    }

    pub fn cache_file(&self) -> &Path {
        &self.cache_file
    }

    pub fn cache_timeout(&self) -> Duration {
        self.cache_timeout
    }

    fn cached<E>(&self) -> Result<Option<Document>, CachedDistIndexError<E>> {
        if !self.cache_file.is_file() || is_stale(&self.cache_file, self.cache_timeout)? {
            return Ok(None);
        }

        let buffer = fs::read(&self.cache_file)
            .map_err(|error| IoError::inaccessible(error, self.cache_file.clone()))?;

        Ok(Some(Document::new(buffer)))
    }

    fn store<E>(&self, document: &Document) -> Result<(), CachedDistIndexError<E>> {
        if let Some(cache_folder) = self.cache_file.parent() {
            fs::create_dir_all(cache_folder)
                .map_err(|error| IoError::inaccessible(error, cache_folder.to_path_buf()))?;
        }

        let file = fs::File::create(&self.cache_file)
            .map_err(|error| IoError::inaccessible(error, self.cache_file.clone()))?;

        let mut writer = BufWriter::new(file);
        writer
            .write_all(document.buffer())
            .map_err(|error| IoError::inaccessible(error, self.cache_file.clone()))?;
        writer
            .flush()
            .map_err(|error| IoError::inaccessible(error, self.cache_file.clone()))?;

        Ok(())
    }
}

impl<C: DistIndexClient> DistIndexClient for CachedDistIndexClient<C> {
    type Error = CachedDistIndexError<C::Error>;

    fn download(&self) -> Result<Document, Self::Error> {
        if let Some(cached) = self.cached()? {
            return Ok(cached);
        }

        let document = self
            .client
            .download()
            .map_err(CachedDistIndexError::Client)?;

        self.store(&document)?;

        Ok(document)
    }
}

impl<C: AsyncDistIndexClient> AsyncDistIndexClient for CachedDistIndexClient<C> {
    type Error = CachedDistIndexError<C::Error>;

    fn download(&self) -> BoxFuture<'_, Result<Document, Self::Error>> {
        Box::pin(async move {
            if let Some(cached) = self.cached()? {
                return Ok(cached);
            }

            let document = self
                .client
                .download()
                .await
                .map_err(CachedDistIndexError::Client)?;

            self.store(&document)?;

            Ok(document)
        })
    }
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CachedDistIndexError<E> {
    #[error("Failed to download the Rust distribution index: {0}")]
    Client(#[source] E),

    #[error(transparent)]
    Io(#[from] IoError),

    #[error(transparent)]
    IsStale(#[from] IsStaleError),
}
