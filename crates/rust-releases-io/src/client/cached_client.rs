use crate::{
    is_stale, Document, IsStaleError, ResourceFile, RetrievalLocation, RetrievedDocument,
    RustReleasesClient,
};
use std::fs;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

const DEFAULT_MEMORY_SIZE: usize = 4096;

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
pub enum IoError {
    #[error("I/O error: {error}{}", format!(" at '{}'", .path.display()))]
    Inaccessible { error: io::Error, path: PathBuf },

    #[error("I/O error: path at '{path}' is a file, but expected a directory")]
    IsFile { path: PathBuf },

    #[error("I/O error: {error}")]
    Auxiliary { error: io::Error },
}

impl IoError {
    pub fn auxiliary(error: io::Error) -> Self {
        Self::Auxiliary { error }
    }

    pub fn inaccessible(error: io::Error, path: PathBuf) -> Self {
        Self::Inaccessible { error, path }
    }

    pub fn is_file(path: PathBuf) -> Self {
        Self::IsFile { path }
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
        if !exists {
            setup_cache_folder(&manifest_path)?;
        }

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

fn fetch_file(url: &str) -> Result<Box<dyn Read + Send + Sync>, CachedClientError> {
    let config = ureq::Agent::config_builder()
        .user_agent("rust-releases (github.com/foresterre/rust-releases/issues)")
        .proxy(ureq::Proxy::try_from_env())
        .build();

    let agent = config.new_agent();

    let response = agent.get(url).call().map_err(|err| HttpError {
        error: Box::new(err),
    })?;

    let reader = Box::new(response.into_body().into_reader());

    Ok(reader)
}

fn write_document_and_cache(
    reader: &mut Box<dyn Read + Send + Sync>,
    file_path: &Path,
) -> Result<Document, CachedClientError> {
    let mut buffer = Vec::with_capacity(DEFAULT_MEMORY_SIZE);

    let bytes_read = reader
        .read_to_end(&mut buffer)
        .map_err(|err| IoError::inaccessible(err, file_path.to_path_buf()))?;

    if bytes_read == 0 {
        return Err(CachedClientError::EmptyFile);
    }

    let mut file = fs::File::create(file_path)
        .map_err(|err| IoError::inaccessible(err, file_path.to_path_buf()))?;

    let mut writer = BufWriter::new(&mut file);
    writer
        .write_all(&buffer)
        .map_err(|err| IoError::inaccessible(err, file_path.to_path_buf()))?;

    Ok(Document::new(buffer))
}
