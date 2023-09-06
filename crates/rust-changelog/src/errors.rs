use rust_releases_core::Channel;

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
    #[error("Unable to parse release date in a release entry '{0}': {1}")]
    TimeParseError(String, time::error::Parse),

    /// Returned in a case a release entry does not contain a recognizable release date
    #[error("Unable to find a valid release date in a release entry")]
    NoDateInChangelogItem,

    /// Returned in a case a release entry does not contain a recognizable release version
    #[error("Unable to find a valid version in a release entry")]
    NoVersionInChangelogItem,

    /// Returned in case the base cache dir could not be found
    #[error(transparent)]
    BaseCacheDir(#[from] rust_releases_io::BaseCacheDirError),

    /// Returned in case a cached client error is returned
    #[error(transparent)]
    CachedClient(#[from] rust_releases_io::CachedClientError),

    /// Returned in case a staleness check error is returned
    #[error(transparent)]
    IsStale(#[from] rust_releases_io::IsStaleError),

    /// Returned in case of semver error on the hot path
    #[error("{0}, input was: {1}")]
    SemverError(rust_releases_core::semver::Error, String),

    /// Returned in case a input resource cannot be parsed as UTF-8
    #[error(transparent)]
    UnrecognizedText(#[from] std::str::Utf8Error),
}
