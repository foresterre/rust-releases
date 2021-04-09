/// A result type which binds the `CoreError` to the error type.
pub type CoreResult<T> = Result<T, CoreError>;

/// Top level failure cases for rust-releases-core
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CoreError {
    /// Returned in case of an i/o error
    #[error("{0}")]
    Io(#[from] std::io::Error),

    /// Returned in the event that the parsing a release channel with a given identifier does not exist
    #[error("Release channel '{0}' was not found")]
    NoSuchChannel(String),
}
