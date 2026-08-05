use crate::client::errors::IoError;
use crate::client::remote_client::HttpClient;
use crate::transport::{AsyncHttpTransport, BoxFuture, HttpTransport};
use crate::{
    AsyncRustReleasesClient, ClientError, Document, IsStaleError, ResourceFile, RetrievalLocation,
    RetrievedDocument, RustReleasesClient, is_stale,
};
use std::fs;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

const DEFAULT_MEMORY_SIZE: usize = 4096;

/// The client to download and cache rust releases.
///
/// If a cached file is not present, or if a cached file is present, but the copy is outdated,
/// the client will download a new copy of the given resource and store it to the `cache_folder`.
/// If a cached file is present, and the copy is not outdated, the cached file will be returned
/// instead.
///
/// Like the [`HttpClient`] it wraps, this client is agnostic of the HTTP client
/// used to download resources.
#[derive(Clone, Debug)]
pub struct HttpCachedClient<T> {
    client: HttpClient<T>,
    cache_folder: PathBuf,
    cache_timeout: Duration,
}

impl<T> HttpCachedClient<T> {
    /// Create a new [`HttpCachedClient`].
    ///
    /// ```
    /// use std::time::Duration;
    /// use rust_releases_io::{base_cache_dir, HttpClient, HttpCachedClient, UreqTransport};
    ///
    /// let req_timeout = Duration::from_secs(5);
    /// let cache_folder = base_cache_dir().unwrap();
    /// let cache_timeout = Duration::from_secs(86_400);
    ///
    /// let http = HttpClient::new(UreqTransport::new()).with_timeout(req_timeout);
    /// let _client = HttpCachedClient::new(http, cache_folder, cache_timeout);
    /// ```
    pub fn new(client: HttpClient<T>, cache_folder: PathBuf, cache_timeout: Duration) -> Self {
        Self {
            client,
            cache_folder,
            cache_timeout,
        }
    }

    /// The client used to download resources which are absent from the cache.
    pub fn client(&self) -> &HttpClient<T> {
        &self.client
    }

    /// Returns the cached document, if it exists and is not stale yet.
    fn cached(&self, path: &Path) -> Result<Option<RetrievedDocument>, HttpCachedClientError> {
        if !path.exists() || is_stale(path, self.cache_timeout)? {
            return Ok(None);
        }

        let document = Document::new(read_from_path(path)?);

        Ok(Some(RetrievedDocument::new(
            document,
            RetrievalLocation::Path(path.to_path_buf()),
        )))
    }

    /// Store the retrieved document at the given cache `path`.
    fn store(
        &self,
        retrieved: &mut RetrievedDocument,
        path: &Path,
    ) -> Result<(), HttpCachedClientError> {
        setup_cache_folder(path)?;

        write_document_and_cache(retrieved.mut_document(), path)
    }
}

#[cfg(feature = "ureq")]
impl HttpCachedClient<crate::UreqTransport> {
    /// Create a new [`HttpCachedClient`], which downloads resources over the
    /// [`UreqTransport`].
    ///
    /// ```
    /// use std::time::Duration;
    /// use rust_releases_io::{base_cache_dir, HttpCachedClient};
    ///
    /// let cache_folder = base_cache_dir().unwrap();
    /// let cache_timeout = Duration::from_secs(86_400);
    ///
    /// let _client = HttpCachedClient::new_with_default_client(cache_folder, cache_timeout);
    /// ```
    ///
    /// [`UreqTransport`]: crate::UreqTransport
    pub fn new_with_default_client(cache_folder: PathBuf, cache_timeout: Duration) -> Self {
        Self::new(HttpClient::default(), cache_folder, cache_timeout)
    }
}

impl<T: HttpTransport> RustReleasesClient for HttpCachedClient<T> {
    type Error = HttpCachedClientError;

    fn fetch(&self, resource: ResourceFile) -> Result<RetrievedDocument, Self::Error> {
        let path = self.cache_folder.join(resource.name());

        if let Some(cached) = self.cached(&path)? {
            return Ok(cached);
        }

        let mut retrieved = self.client.fetch(resource)?;

        self.store(&mut retrieved, &path)?;

        Ok(retrieved)
    }
}

impl<T: AsyncHttpTransport> AsyncRustReleasesClient for HttpCachedClient<T> {
    type Error = HttpCachedClientError;

    fn fetch<'a>(
        &'a self,
        resource: ResourceFile<'a, 'a>,
    ) -> BoxFuture<'a, Result<RetrievedDocument, Self::Error>> {
        Box::pin(async move {
            let path = self.cache_folder.join(resource.name());

            if let Some(cached) = self.cached(&path)? {
                return Ok(cached);
            }

            let mut retrieved = self.client.fetch(resource).await?;

            self.store(&mut retrieved, &path)?;

            Ok(retrieved)
        })
    }
}

fn read_from_path(path: &Path) -> Result<Vec<u8>, HttpCachedClientError> {
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
fn setup_cache_folder(manifest_path: &Path) -> Result<(), HttpCachedClientError> {
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
) -> Result<(), HttpCachedClientError> {
    let mut file = fs::File::create(file_path)
        .map_err(|err| IoError::inaccessible(err, file_path.to_path_buf()))?;

    let mut writer = BufWriter::new(&mut file);
    writer
        .write_all(document.buffer())
        .map_err(|err| IoError::inaccessible(err, file_path.to_path_buf()))?;

    Ok(())
}

/// A list of errors which may be produced by [`HttpCachedClient::fetch`].
///
/// [`HttpCachedClient::fetch`]: RustReleasesClient::fetch
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum HttpCachedClientError {
    /// Returned if the client could not fetch an item.
    #[error(transparent)]
    Client(#[from] ClientError),

    /// Returned in case of an `std::io::Error`.
    #[error(transparent)]
    Io(#[from] IoError),

    /// Returned in case it wasn't possible to check whether the cache file is
    /// stale or not.
    #[error(transparent)]
    IsStale(#[from] IsStaleError),
}
