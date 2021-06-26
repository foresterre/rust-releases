use rust_releases_core::Channel;

/// A result type which binds the `RustDistError` to the error type.
pub type RustDistResult<T> = Result<T, RustDistError>;

/// Top level failure cases for rust-releases-rust-dist source crate
///
// FIXME: These should be enhanced by providing more detailed errors. We often simply bubble up
//  errors (like i/o) directly, but some of these do not provide enough information to be useful while
//  debugging (e.g. file not found, but which file?). In addition, they currently expose internals like
//  rusoto, which should be opaque, so the inner working can neatly be replaced without breaking changes.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RustDistError {
    /// Returned in case a `Channel` is not available for the `Source`.
    #[error("Channel {0} is not yet available for the 'RustDist' source type")]
    ChannelNotAvailable(Channel),

    /// Returned when the AWS Object returned does not have meta data. In such case
    /// we can't get path of the object which we use to determine the release version.
    #[error("Unable to obtain release metadata")]
    ChunkMetadataMissing,

    /// Returned when we can't consume the inner in-memory buffered writer.
    #[error("Unable to flush chunk: '{0}'")]
    ChunkWriteFlushError(#[from] std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>),

    /// Returned in case of an i/o error.
    #[error("{0}")]
    Io(#[from] std::io::Error),

    /// Unable to fetch metadata about the available Rust releases.
    #[error("Unable to fetch metadata about the available Rust releases: '{0}'")]
    UnableToFetch(#[from] rusoto_core::RusotoError<rusoto_s3::ListObjectsV2Error>),

    /// Returned in case of an `rust-releases-io` i/o error.
    #[error("{0}")]
    RustReleasesIo(#[from] rust_releases_io::IoError),

    /// Returned in case of a TLS error.
    #[error("{0}")]
    SecureConnectionError(#[from] rusoto_core::request::TlsError),

    /// Returned in case the input text cannot be parsed.
    #[error("{0}")]
    UnrecognizedText(#[from] std::string::FromUtf8Error),

    /// Returned in case a component of a `semver` version could not be parsed as a number.
    ///
    /// The component is usually the `major`, `minor` or `patch` version.
    #[error("The '{0}' component of the version number could not be parsed. The input was: '{1}'")]
    UnableToParseVersionNumberComponent(&'static &'static str, String),
}
