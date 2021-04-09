#[cfg(feature = "http_client")]
use attohttpc as http_client;

/// A result type which binds the `RustReleasesCoreError` to the error type.
pub type IoResult<T> = Result<T, IoError>;

/// Top level failure cases for rust-releases-core
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum IoError {
    /// Returned if the cache is not accessible
    #[error("Unable to create or access RustReleases cache")]
    DlCache,

    #[cfg(feature = "http_client")]
    /// Returned if the HTTP client could not fetch an item
    #[error("{0}")]
    HttpClient(#[from] http_client::Error),

    /// Returned in case of an i/o error
    #[error("{0}")]
    Io(#[from] std::io::Error),

    /// Returned in the event that the parsing a release channel with a given identifier does not exist
    #[error("Release channel '{0}' was not found")]
    NoSuchChannel(String),

    /// Returned if the system time could not be obtained
    #[error("{0}")]
    SystemTime(#[from] std::time::SystemTimeError),
}
