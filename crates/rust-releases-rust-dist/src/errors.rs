use rust_releases_core::Channel;

/// A result type which binds the `RustDistError` to the error type.
pub type RustDistResult<T> = Result<T, RustDistError>;

/// Top level failure cases for rust-releases-rust-dist source crate
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RustDistError {
    /// Returned in case a `Channel` is not available for the `Source`
    #[error("Channel {0} is not yet available for the 'RustDist' source type")]
    ChannelNotAvailable(Channel),

    /// Returned in case of an i/o error
    #[error("{0}")]
    Io(#[from] std::io::Error),

    /// Returned in case of a TLS error
    #[error("{0}")]
    RusotoTlsError(#[from] rusoto_core::request::TlsError),

    /// Returned in case of an `rust-releases-io` i/o error
    #[error("{0}")]
    RustReleasesIo(#[from] rust_releases_io::IoError),

    /// Returned in case the input text cannot be parsed
    #[error("{0}")]
    UnrecognizedText(#[from] std::string::FromUtf8Error),

    /// Returned in case an item could not be parsed as a number, but was expected to
    #[error("{0}")]
    UnableToParseNumber(#[from] std::num::ParseIntError),
}
