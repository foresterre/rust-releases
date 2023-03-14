use aws_config::InvalidAppName;
use aws_sdk_s3::error::ListObjectsV2Error;
use aws_smithy_http::result::SdkError;
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
    /// Returned in case of an error related to the AWS SDK
    #[error(transparent)]
    AwsError(#[from] AwsError),

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
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Returned in case of the base cache folder could not be found.
    #[error(transparent)]
    BaseCacheDir(#[from] rust_releases_io::BaseCacheDirError),

    /// Returned when the staleness check fails.
    #[error(transparent)]
    IsStale(#[from] rust_releases_io::IsStaleError),

    /// Returned in case the input text cannot be parsed.
    #[error(transparent)]
    UnrecognizedText(#[from] std::str::Utf8Error),

    /// Returned in case a component of a `semver` version could not be parsed as a number.
    ///
    /// The component is usually the `major`, `minor` or `patch` version.
    #[error("The '{0}' component of the version number could not be parsed. The input was: '{1}'")]
    UnableToParseVersionNumberComponent(&'static &'static str, String),
}

/// Errors returned by the AWS SDK.
#[derive(Debug, thiserror::Error)]
pub enum AwsError {
    #[error("Unable to build an anonymous AWS S3 request: failed to disable signing")]
    DisableSigning,

    /// Returned when the app name is invalid. Since the app name is configured by the library,
    /// it's a bug when this error is returned.
    #[error("Could not configure AWS S3 client: {0}")]
    InvalidAppName(#[from] InvalidAppName),

    /// Failed to build the input required to make an anonymous AWS S3 request.
    #[error("Unable to build the list objects operation")]
    ListObjectsBuildOperationInput,

    /// Returned when it's not possible to list the S3 objects in the Rust bucket, required to
    /// build our releases index.
    #[error("Unable to fetch Rust distribution index: {0}")]
    ListObjectsError(Box<SdkError<ListObjectsV2Error>>),

    /// Failed to build the operation required to make an anonymous AWS S3 request.
    #[error("Unable to make list objects operation")]
    ListObjectsMakeOperation,
}
