use crate::aws::manifest_index::ManifestIndexError;
use crate::manifest::ReleaseManifestError;
use aws_config::InvalidAppName;
use aws_sdk_s3::operation::get_object::GetObjectError;
use aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Error;
use aws_sdk_s3::primitives::ByteStreamError;

/// Errors returned by the AWS SDK.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum AwsError {
    /// Returned when the app name is invalid. Since the app name is configured by the library,
    /// it's a bug when this error is returned.
    #[error("Could not configure AWS S3 client: {0}")]
    InvalidAppName(#[from] InvalidAppName),

    /// Returned when it's not possible to list the S3 objects in the Rust bucket, required to
    /// build our releases index.
    #[error(
        "Unable to list the objects of the Rust distribution bucket with the prefix '{prefix}': {source}"
    )]
    ListObjectsError {
        prefix: String,
        #[source]
        source: Box<ListObjectsV2Error>,
    },

    /// Returned when the S3 object which holds a release manifest cannot be fetched.
    #[error("Unable to fetch the object '{key}' of the Rust distribution bucket: {source}")]
    GetObjectError {
        key: String,
        #[source]
        source: Box<GetObjectError>,
    },

    /// Returned when the body of the S3 object which holds a release manifest cannot be read.
    #[error("Unable to read the object '{key}' of the Rust distribution bucket: {source}")]
    ObjectBody {
        key: String,
        #[source]
        source: ByteStreamError,
    },

    /// Returned when the object which holds a release manifest cannot be parsed.
    #[error("Unable to parse the release manifest '{key}': {source}")]
    ReleaseManifest {
        key: String,
        #[source]
        source: ReleaseManifestError,
    },

    /// Returned when the object which lists the dated release manifests cannot be parsed.
    #[error("Unable to parse the manifest index '{key}': {source}")]
    ManifestIndex {
        key: String,
        #[source]
        source: ManifestIndexError,
    },

    /// Returned when the runtime on which the AWS S3 requests are sent could not be started.
    #[error("Unable to start the runtime used to send AWS S3 requests: {0}")]
    Runtime(std::io::Error),
}
