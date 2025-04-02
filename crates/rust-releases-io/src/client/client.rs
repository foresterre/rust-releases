use crate::client::errors::{HttpError, IoError};
use crate::{Document, ResourceFile, RetrievalLocation, RetrievedDocument, RustReleasesClient};
use std::io::Read;
use std::time::Duration;

const DEFAULT_MEMORY_SIZE: usize = 4096;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(150);

/// The client to download and cache rust releases.
///
/// If a cached file is not present, or if a cached file is present, but the copy is outdated,
/// the client will download a new copy of the given resource and store it to the `cache_folder`.
/// If a cached file is present, and the copy is not outdated, the cached file will be returned
/// instead.
pub struct Client {
    timeout: Duration,
}

impl Client {
    /// Create a new [`Client`].
    ///
    /// ```
    /// use std::time::Duration;
    /// use rust_releases_io::Client;
    /// let timeout = Duration::from_secs(86_400);
    ///
    /// let _client = Client::new(timeout);
    /// ```
    pub fn new(timeout: Duration) -> Self {
        Self { timeout }
    }
}

impl Default for Client {
    /// Create a new [`Client`].
    ///
    /// ```
    /// use rust_releases_io::Client;
    ///
    /// let _client = Client::default();
    /// ```
    fn default() -> Self {
        Self {
            timeout: DEFAULT_TIMEOUT,
        }
    }
}

impl RustReleasesClient for Client {
    type Error = ClientError;

    fn fetch(&self, resource: ResourceFile) -> Result<RetrievedDocument, Self::Error> {
        let mut reader = fetch_file(resource.url(), self.timeout)?;

        // write to memory
        let document = write_document(&mut reader)?;

        Ok(RetrievedDocument::new(
            document,
            RetrievalLocation::RemoteUrl(resource.url.to_string()),
        ))
    }
}

fn fetch_file(url: &str, timeout: Duration) -> Result<Box<dyn Read + Send + Sync>, ClientError> {
    let config = ureq::Agent::config_builder()
        .user_agent("rust-releases (github.com/foresterre/rust-releases/issues)")
        .proxy(ureq::Proxy::try_from_env())
        .timeout_global(Some(timeout))
        .build();

    let agent = config.new_agent();

    let response = agent.get(url).call().map_err(|err| HttpError {
        error: Box::new(err),
    })?;

    let reader = Box::new(response.into_body().into_reader());

    Ok(reader)
}

fn write_document(reader: &mut Box<dyn Read + Send + Sync>) -> Result<Document, ClientError> {
    let mut buffer = Vec::with_capacity(DEFAULT_MEMORY_SIZE);

    let bytes_read = reader
        .read_to_end(&mut buffer)
        .map_err(IoError::auxiliary)?;

    if bytes_read == 0 {
        return Err(ClientError::Empty);
    }

    Ok(Document::new(buffer))
}

/// A list of errors which may be produced by [`Client::fetch`].
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ClientError {
    /// Returned if an empty document was fetched.
    #[error("Received empty file")]
    Empty,

    /// Returned if the HTTP client could not fetch an item
    #[error(transparent)]
    Http(#[from] HttpError),

    /// Returned in case of an `std::io::Error`.
    #[error(transparent)]
    Io(#[from] IoError),
}
