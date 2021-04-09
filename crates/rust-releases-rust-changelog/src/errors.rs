use rust_releases_core::Channel;
use rust_releases_io::IoError;

/// A result type which binds the `RustChangelogError` to the error type.
pub type RustChangelogResult<T> = Result<T, RustChangelogError>;

/// Top level failure cases for rust-releases-rust-changelog source crate
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RustChangelogError {
    /// Returned in case a `Channel` is not available for the `Source`
    #[error("Channel {0} is not available for the 'RustChangelog' source type")]
    ChannelNotAvailable(Channel),

    /// Returned in case a `rust-releases-io` error is caught
    #[error("{0}")]
    RustReleasesIoError(#[from] IoError),

    /// Returned in case a input resource cannot be parsed as UTF-8
    #[error("{0}")]
    UnrecognizedText(#[from] std::string::FromUtf8Error),
}
