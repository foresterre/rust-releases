use crate::client::client::Client;
use crate::client::errors::{HttpError, IoError};
use crate::{
    is_stale, ClientError, Document, IsStaleError, ResourceFile, RetrievalLocation,
    RetrievedDocument, RustReleasesClient,
};
use std::fs;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

const DEFAULT_MEMORY_SIZE: usize = 4096;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(150);

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
        let path = self.cache_folder.join(resource.name());
        let exists = path.exists();

        // Returned the cached document if it exists and is not stale
        if exists && !is_stale(&path, self.cache_timeout)? {
            let buffer = read_from_path(&path)?;
            let document = Document::new(buffer);

            return Ok(RetrievedDocument::new(
                document,
                RetrievalLocation::Cache(path),
            ));
        }

        // Ensure we have a place to put the cached document.
        if !exists {
            setup_cache_folder(&path)?;
        }

        let client = Client::new(DEFAULT_TIMEOUT);
        let mut retrieved = client.fetch(resource).map_err(CachedClientError::from)?;

        let document = retrieved.mut_document();

        // write to memory
        write_document_and_cache(document, &path)?;

        Ok(retrieved)
    }
}

fn read_from_path(path: &Path) -> Result<Vec<u8>, CachedClientError> {
    let mut reader = BufReader::new(
        fs::File::open(path).map_err(|err| IoError::inaccessible(err, path.to_path_buf()))?,
    );

    let mut memory = Vec::with_capacity(DEFAULT_MEMORY_SIZE);
    reader
        .read_to_end(&mut memory)
        .map_err(IoError::auxiliary)?;

    Ok(memory)
}

/// `manifest_path` should include the cache folder and name of the manifest file.
fn setup_cache_folder(manifest_path: &Path) -> Result<(), CachedClientError> {
    fn create_dir_all(path: &Path) -> Result<(), IoError> {
        fs::create_dir_all(path).map_err(|err| IoError::inaccessible(err, path.to_path_buf()))
    }

    // Check we're not at the root of the file system.
    if let Some(cache_folder) = manifest_path.parent() {
        // Check that the cache folder doesn't exist yet.
        match fs::metadata(cache_folder) {
            // If the folder already exists we don't need to do anything.
            Ok(m) if m.is_dir() => Ok(()),
            // A file with the same name exists. In the common tree based filesystem where only directories
            // can hold files, this should never happen, since we're already in the `manifest_path.parent()`
            // call.
            Ok(_) => Err(IoError::is_file(cache_folder.to_path_buf())),
            // If the folder is not found, we create it.
            Err(err) if err.kind() == io::ErrorKind::NotFound => create_dir_all(cache_folder),
            // If the folder
            Err(err) => Err(IoError::inaccessible(err, cache_folder.to_path_buf())),
        }?;
    }

    Ok(())
}

fn write_document_and_cache(
    document: &mut Document,
    file_path: &Path,
) -> Result<(), CachedClientError> {
    let mut file = fs::File::create(file_path)
        .map_err(|err| IoError::inaccessible(err, file_path.to_path_buf()))?;

    let mut writer = BufWriter::new(&mut file);
    writer
        .write_all(document.buffer())
        .map_err(|err| IoError::inaccessible(err, file_path.to_path_buf()))?;

    Ok(())
}

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

impl From<ClientError> for CachedClientError {
    fn from(err: ClientError) -> Self {
        match err {
            ClientError::Empty => CachedClientError::EmptyFile,
            ClientError::Http(err) => CachedClientError::Http(err),
            ClientError::Io(err) => CachedClientError::Io(err),
        }
    }
}
