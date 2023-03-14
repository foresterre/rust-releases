use rust_releases_core::Channel;
use std::io;

/// A result type which binds the `RustDistWithCLIError` to the error type.
pub type RustDistWithCLIResult<T> = Result<T, RustDistWithCLIError>;

/// Top level failure cases for rust-releases-rust-dist-with-cli source crate
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RustDistWithCLIError {
    /// Returned in case a `Channel` is not implemented for the Source
    #[error("Channel {0} is not yet available for the 'RustDistWithCLI' source type")]
    ChannelNotAvailable(Channel),

    /// Returned in case of an `std::io::Error`.
    #[error(transparent)]
    Io(#[from] io::Error),

    /// Returned in case the base cache dir could not be found.
    #[error(transparent)]
    BaseCacheDir(#[from] rust_releases_io::BaseCacheDirError),

    /// Returned in case the input text could not be parsed
    #[error(transparent)]
    UnrecognizedText(#[from] std::str::Utf8Error),
}
