#![cfg(feature = "http_client")]

use crate::DEFAULT_MEMORY_SIZE;
use crate::{
    is_stale, Document, IsStaleError, ResourceFile, RetrievalLocation, RetrievedDocument,
    RustReleasesClient,
};
use std::fs;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// A list of errors which may be produced by [`CachedClient::fetch`].
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CachedClientError {
    /// Returned if the fetched file was empty.
    #[error("Received empty file")]
    EmptyFile,

    /// Returned if the HTTP client could not fetch an item
    #[error(transparent)]
    Http(#[from] HttpError),

    /// Returned in case of an `std::io::Error`.
    #[error(transparent)]
    Io(#[from] IoError),

    /// Returned in case it wasn't possible to check whether the cache file is
    /// stale or not.
    #[error(transparent)]
    IsStale(#[from] IsStaleError),
}

#[derive(Debug, thiserror::Error)]
#[error("I/O error: {error}{}", .path.as_ref().map(|p| format!(" at '{}'", p.display())).unwrap_or_else(|| "".to_string()))]
pub struct IoError {
    error: io::Error,
    path: Option<PathBuf>,
}

impl IoError {
    fn new(error: io::Error, path: Option<PathBuf>) -> Self {
        Self { error, path }
    }

    pub fn without_path(error: io::Error) -> Self {
        Self::new(error, None)
    }

    pub fn with_path(error: io::Error, path: PathBuf) -> Self {
        Self::new(error, Some(path))
    }
}

/// An error which is returned for a fault which occurred during processing of an HTTP request.
#[derive(Debug, thiserror::Error)]
#[error("HTTP error: {error}")]
pub struct HttpError {
    // We box the error since it can be very large.
    error: Box<ureq::Error>,
}

/// The client to download and cache rust releases.
///
/// If a cached file is not present, or if a cached file is present, but the copy is outdated,
/// the client will download a new copy of the given resource and store it to the `cache_folder`.
/// If a cached file is present, and the copy is not outdated, the cached file will be returned
/// instead.
pub struct CachedClient {
    cache_folder: PathBuf,
    cache_timeout: Duration,
}

impl CachedClient {
    /// Create a new [`CachedClient`].
    ///
    /// ```
    /// use std::time::Duration;
    /// use rust_releases_io::{base_cache_dir, CachedClient};
    /// let cache_folder = base_cache_dir().unwrap();
    /// let timeout = Duration::from_secs(86_400);
    ///
    /// let _client = CachedClient::new(cache_folder, timeout);
    /// ```
    pub fn new(cache_folder: PathBuf, cache_timeout: Duration) -> Self {
        Self {
            cache_folder,
            cache_timeout,
        }
    }
}

impl RustReleasesClient for CachedClient {
    type Error = CachedClientError;

    fn fetch(&self, resource: ResourceFile) -> Result<RetrievedDocument, Self::Error> {
        let manifest_path = self.cache_folder.join(resource.name());
        let exists = manifest_path.exists();

        // Returned the cached document if it exists and is not stale
        if exists && !is_stale(&manifest_path, self.cache_timeout)? {
            let buffer = read_from_path(&manifest_path)?;
            let document = Document::new(buffer);

            return Ok(RetrievedDocument::new(
                document,
                RetrievalLocation::Cache(manifest_path),
            ));
        }

        // Ensure we have a place to put the cached document.
        setup_cache_folder(&manifest_path)?;

        let mut reader = fetch_file(resource.url())?;

        // write to memory
        let document = write_document_and_cache(&mut reader, &manifest_path)?;

        Ok(RetrievedDocument::new(
            document,
            RetrievalLocation::RemoteUrl(resource.url.to_string()),
        ))
    }
}

fn read_from_path(path: &Path) -> Result<Vec<u8>, CachedClientError> {
    let mut reader = BufReader::new(
        fs::File::open(path).map_err(|err| IoError::with_path(err, path.to_path_buf()))?,
    );

    let mut memory = Vec::with_capacity(DEFAULT_MEMORY_SIZE);
    reader
        .read_to_end(&mut memory)
        .map_err(IoError::without_path)?;

    Ok(memory)
}

/// `manifest_path` should include the cache folder and name of the manifest file.
fn setup_cache_folder(manifest_path: &Path) -> Result<(), CachedClientError> {
    // Check we're not at the root of the file system.
    if let Some(cache_folder) = manifest_path.parent() {
        // Check whether the manifest already exists - if it does, the cache folder is already
        // present and doesn't need to be created.
        let manifest_exists = manifest_path
            .try_exists()
            .map_err(|err| IoError::with_path(err, manifest_path.to_path_buf()))?;

        if !manifest_exists {
            // Check that the cache folder doesn't exist yet.
            let cache_folder_metadata = fs::metadata(cache_folder)
                .map_err(|err| IoError::with_path(err, cache_folder.to_path_buf()))?;
            if !cache_folder_metadata.is_dir() {
                fs::create_dir_all(cache_folder)
                    .map_err(|err| IoError::with_path(err, cache_folder.to_path_buf()))?;
            }
        }
    }

    Ok(())
}

fn fetch_file(url: &str) -> Result<Box<dyn Read + Send + Sync>, CachedClientError> {
    let response = ureq::get(url)
        .set(
            "User-Agent",
            "rust-releases (github.com/foresterre/rust-releases/issues)",
        )
        .call()
        .map_err(|err| HttpError {
            error: Box::new(err),
        })?;

    Ok(response.into_reader())
}

fn write_document_and_cache(
    reader: &mut Box<dyn Read + Send + Sync>,
    file_path: &Path,
) -> Result<Document, CachedClientError> {
    let mut buffer = Vec::with_capacity(DEFAULT_MEMORY_SIZE);

    let bytes_read = reader
        .read_to_end(&mut buffer)
        .map_err(|err| IoError::with_path(err, file_path.to_path_buf()))?;

    if bytes_read == 0 {
        return Err(CachedClientError::EmptyFile);
    }

    let mut file = fs::File::create(file_path)
        .map_err(|err| IoError::with_path(err, file_path.to_path_buf()))?;

    let mut writer = BufWriter::new(&mut file);
    writer
        .write_all(&buffer)
        .map_err(|err| IoError::with_path(err, file_path.to_path_buf()))?;

    Ok(Document::new(buffer))
}
