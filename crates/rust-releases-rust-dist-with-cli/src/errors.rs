use rust_releases_core::Channel;

/// A result type which binds the `RustDistWithCLIError` to the error type.
pub type RustDistWithCLIResult<T> = Result<T, RustDistWithCLIError>;

/// Top level failure cases for rust-releases-rust-dist-with-cli source crate
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RustDistWithCLIError {
    /// Returned in case a `Channel` is not implemented for the Source
    #[error("Channel {0} is not yet available for the 'RustDistWithCLI' source type")]
    ChannelNotAvailable(Channel),

    /// Returned in case of an `rust-releases-io` i/o error
    #[error("{0}")]
    RustReleasesIo(#[from] rust_releases_io::IoError),

    /// Returned in case the input text could not be parsed
    #[error("{0}")]
    UnrecognizedText(#[from] std::string::FromUtf8Error),
}
