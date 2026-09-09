use crate::client::{AsyncDistIndexClient, DistIndexClient};
use aws_config::{AppName, BehaviorVersion, InvalidAppName};
use aws_sdk_s3::config::Region;
use aws_sdk_s3::operation::list_objects_v2::{ListObjectsV2Error, ListObjectsV2Output};
use aws_sdk_s3::types::Object;
use rust_releases_io::{BoxFuture, Document};
use std::io::Write;

// Rust currently always uses the US West 1 bucket
const RUST_DIST_REGION: Region = Region::from_static("us-west-1");

// The bucket from which the official Rust sources are distributed
const RUST_DIST_BUCKET: &str = "static-rust-lang-org";

// We only request objects which start with the following string, which currently only matches stable
// releases
const OBJECT_PREFIX: &str = "dist/rustc-";

// amount of objects requested per chunk
const REQUEST_SIZE: i32 = 1000;

const APP_NAME: &str = "rust-releases+`github|foresterre|rust-releases`";

#[derive(Clone, Debug)]
pub struct AwsIndexClient {
    client: aws_sdk_s3::Client,
}

impl AwsIndexClient {
    pub async fn new() -> Result<Self, AwsError> {
        let app_name = AppName::new(APP_NAME).map_err(AwsError::InvalidAppName)?;

        let config = aws_config::defaults(BehaviorVersion::v2026_01_12())
            .no_credentials()
            .app_name(app_name)
            .region(RUST_DIST_REGION)
            .load()
            .await;

        Ok(Self::with_client(aws_sdk_s3::Client::new(&config)))
    }

    pub fn with_client(client: aws_sdk_s3::Client) -> Self {
        Self { client }
    }

    pub fn client(&self) -> &aws_sdk_s3::Client {
        &self.client
    }

    async fn download_index(&self) -> Result<Document, AwsError> {
        let mut buffer = Vec::new();
        let mut offset = None;

        while let ChunkState::Offset(next_offset) =
            self.download_chunk(offset.take(), &mut buffer).await?
        {
            offset = Some(next_offset);
        }

        Ok(Document::new(buffer))
    }

    async fn download_chunk(
        &self,
        offset: Option<String>,
        to: &mut impl Write,
    ) -> Result<ChunkState, AwsError> {
        let raw = list_objects(&self.client, offset).await?;

        match raw.is_truncated {
            Some(truncated) if !truncated => return Ok(ChunkState::Complete),
            _ => {}
        }

        let objects = raw.contents.ok_or(AwsError::ChunkMetadataMissing)?;
        let state = match write_objects(to, &objects) {
            Some(key) => ChunkState::Offset(key),
            None => ChunkState::Complete,
        };

        Ok(state)
    }
}

impl AsyncDistIndexClient for AwsIndexClient {
    type Error = AwsError;

    fn download(&self) -> BoxFuture<'_, Result<Document, Self::Error>> {
        Box::pin(self.download_index())
    }
}

// Runs the asynchronous [`AwsIndexClient`] on a runtime of its own, so the index can be downloaded
// from a blocking context.
#[derive(Debug)]
pub struct BlockingAwsIndexClient {
    client: AwsIndexClient,
    runtime: tokio::runtime::Runtime,
}

impl BlockingAwsIndexClient {
    pub fn new() -> Result<Self, AwsError> {
        let runtime = tokio::runtime::Runtime::new().map_err(AwsError::Runtime)?;
        let client = runtime.block_on(AwsIndexClient::new())?;

        Ok(Self { client, runtime })
    }

    pub fn with_client(client: AwsIndexClient) -> Result<Self, AwsError> {
        let runtime = tokio::runtime::Runtime::new().map_err(AwsError::Runtime)?;

        Ok(Self { client, runtime })
    }

    pub fn client(&self) -> &AwsIndexClient {
        &self.client
    }
}

impl DistIndexClient for BlockingAwsIndexClient {
    type Error = AwsError;

    fn download(&self) -> Result<Document, Self::Error> {
        self.runtime.block_on(self.client.download_index())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ChunkState {
    // Contains the last key in the current chunk, which is the offset key for the next call
    Offset(String),
    Complete,
}

async fn list_objects(
    client: &aws_sdk_s3::Client,
    offset: Option<impl Into<String>>,
) -> Result<ListObjectsV2Output, AwsError> {
    client
        .list_objects_v2()
        .bucket(RUST_DIST_BUCKET)
        .max_keys(REQUEST_SIZE)
        .set_start_after(offset.map(Into::into))
        .prefix(OBJECT_PREFIX)
        .send()
        .await
        .map_err(|e| AwsError::ListObjectsError(Box::new(e.into_service_error())))
}

fn write_objects(buffer: &mut impl Write, objects: &[Object]) -> Option<String> {
    for object in objects {
        if let Some(key) = object.key.as_deref() {
            let _ = buffer.write(format!("{}\n", key).as_bytes());
        }
    }

    let _ = buffer.flush();

    // return the last detected key
    objects
        .last()
        .and_then(|obj| obj.key.as_ref().map(|o| o.to_string()))
}

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
    #[error("Unable to fetch Rust distribution index: {0}")]
    ListObjectsError(Box<ListObjectsV2Error>),

    /// Returned when the AWS Object returned does not have meta data. In such case
    /// we can't get path of the object which we use to determine the release version.
    #[error("Unable to obtain release metadata")]
    ChunkMetadataMissing,

    /// Returned when the runtime on which the AWS S3 requests are sent could not be started.
    #[error("Unable to start the runtime used to send AWS S3 requests: {0}")]
    Runtime(std::io::Error),
}
