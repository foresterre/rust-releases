use crate::aws::error::AwsError;
use aws_config::{AppName, BehaviorVersion};
use aws_sdk_s3::config::Region;
use rust_releases_io::Document;

// Rust currently always uses the US West 1 bucket
const RUST_DIST_REGION: Region = Region::from_static("us-west-1");

// The bucket from which the official Rust sources are distributed
const RUST_DIST_BUCKET: &str = "static-rust-lang-org";

// Amount of objects requested per chunk
const REQUEST_SIZE: i32 = 1000;

const APP_NAME: &str = "rust-releases+`github|foresterre|rust-releases`";

/// Wrapper for S3 actions on the Rust dist bucket
#[derive(Clone, Debug)]
pub struct RustDistBucket {
    client: aws_sdk_s3::Client,
}

impl RustDistBucket {
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

    /// List objects in the rust dist bucket
    pub async fn list(&self, prefix: &str) -> Result<Vec<String>, AwsError> {
        let mut keys = Vec::new();
        let mut token = None;

        loop {
            let page = self
                .client
                .list_objects_v2()
                .bucket(RUST_DIST_BUCKET)
                .prefix(prefix)
                .max_keys(REQUEST_SIZE)
                .set_continuation_token(token.take())
                .send()
                .await
                .map_err(|error| AwsError::ListObjectsError {
                    prefix: prefix.to_string(),
                    source: Box::new(error.into_service_error()),
                })?;

            keys.extend(
                page.contents
                    .into_iter()
                    .flatten()
                    .filter_map(|object| object.key),
            );

            match page.next_continuation_token {
                Some(next) if page.is_truncated.unwrap_or(false) => token = Some(next),
                _ => break,
            }
        }

        Ok(keys)
    }

    /// Get an object from the rust dist bucket
    pub async fn get(&self, key: &str) -> Result<Document, AwsError> {
        let object = self
            .client
            .get_object()
            .bucket(RUST_DIST_BUCKET)
            .key(key)
            .send()
            .await
            .map_err(|error| AwsError::GetObjectError {
                key: key.to_string(),
                source: Box::new(error.into_service_error()),
            })?;

        let body = object
            .body
            .collect()
            .await
            .map_err(|source| AwsError::ObjectBody {
                key: key.to_string(),
                source,
            })?;

        Ok(Document::new(body.to_vec()))
    }
}
