/// Failure cases w.r.t. parsing a Rust changelog.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ParseError {
    /// Returned in case of `time` parse errors
    #[error("Unable to parse release date in a release entry '{0}': {1}")]
    TimeParseError(String, time::error::Parse),

    /// Returned when a version string cannot be parsed as a three-component `major.minor.patch` version
    #[error("Unable to parse version '{0}")]
    VersionParseError(String),

    /// Returned in a case a release entry does not contain a recognizable release date
    #[error("Unable to find a valid release date in a release entry")]
    NoDateInChangelogItem,

    /// Returned in a case a release entry does not contain a recognizable release version
    #[error("Unable to find a valid version in a release entry")]
    NoVersionInChangelogItem,

    /// Returned in case the input resource cannot be parsed as UTF-8
    #[error(transparent)]
    UnrecognizedText(#[from] std::str::Utf8Error),
}
