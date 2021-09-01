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

    /// Returned in case of of `chrono` parse errors
    #[error("Unable to parse release date in a release entry '{0}'")]
    ChronoParseError(String),

    /// Returned in a case a release entry does not contain a recognizable release date
    #[error("Unable to find a valid release date in a release entry")]
    NoDateInChangelogItem,

    /// Returned in a case a release entry does not contain a recognizable release version
    #[error("Unable to find a valid version in a release entry")]
    NoVersionInChangelogItem,

    /// Returned in case a `rust-releases-io` error is caught
    #[error("{0}")]
    RustReleasesIoError(#[from] IoError),

    /// Returned in case of semver error on the hot path
    #[error("{0}, input was: {1}")]
    SemverError(rust_releases_core::semver::Error, String),

    /// Returned in case a input resource cannot be parsed as UTF-8
    #[error("{0}")]
    UnrecognizedText(#[from] std::string::FromUtf8Error),
}
