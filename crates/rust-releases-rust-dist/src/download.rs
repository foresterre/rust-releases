use crate::{RustDistError, RustDistResult};
use rusoto_core::credential::{AwsCredentials, StaticProvider};
use rusoto_core::{HttpClient, Region};
use rusoto_s3::{ListObjectsV2Request, S3Client, S3};

use std::io::Write;

// Rust currently always uses the US West 1 bucket
const RUST_DIST_REGION: Region = Region::UsWest1;

// The bucket from which the official Rust sources are distributed
const RUST_DIST_BUCKET: &str = "static-rust-lang-org";

// amount of objects requested per chunk
const REQUEST_SIZE: i64 = 1000;

// We only request objects which start with the following string, which currently only matches stable
// releases
const OBJECT_PREFIX: &str = "dist/20";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChunkState {
    // Contains the last key in the current chunk, which is the offset key for the next call
    Offset(String),
    Complete,
}

// Client used to obtain the rust releases meta data, part by part
pub trait ChunkClient {
    fn download_chunk(
        &self,
        offset: Option<impl Into<String>>,
        to: &mut impl Write,
    ) -> RustDistResult<ChunkState>;

    fn download(&self, to: &mut impl Write) -> RustDistResult<()>;
}

// The default Rust Releases client
pub struct Client {
    s3_client: S3Client,
    runtime: tokio::runtime::Runtime,
}

impl Client {
    pub fn try_default() -> RustDistResult<Self> {
        let mut http_client = HttpClient::new().map_err(RustDistError::from)?;
        http_client.local_agent_append(
            "rust-releases (github.com/foresterre/rust-releases/issues)".to_string(),
        );

        let s3_client = S3Client::new_with(
            http_client,
            StaticProvider::from(AwsCredentials::default()),
            RUST_DIST_REGION,
        );

        let runtime = tokio::runtime::Runtime::new()?;

        Ok(Self { s3_client, runtime })
    }
}

impl ChunkClient for Client {
    fn download_chunk(
        &self,
        offset: Option<impl Into<String>>,
        to: &mut impl Write,
    ) -> RustDistResult<ChunkState> {
        let raw = self
            .runtime
            .block_on(self.s3_client.list_objects_v2(ListObjectsV2Request {
                bucket: RUST_DIST_BUCKET.to_string(),
                start_after: offset.map(Into::into),
                max_keys: Some(REQUEST_SIZE),
                prefix: Some(OBJECT_PREFIX.to_owned()),
                ..Default::default()
            }))?;

        let objects = raw.contents.ok_or(RustDistError::ChunkMetadataMissing)?;

        let state = match write_objects(to, &objects) {
            Some(key) => ChunkState::Offset(key),
            None => ChunkState::Complete,
        };

        Ok(state)
    }

    fn download(&self, to: &mut impl Write) -> RustDistResult<()> {
        let mut offset = None;

        while let Ok(ChunkState::Offset(next_offset)) = self.download_chunk(offset.to_owned(), to) {
            offset = Some(next_offset);
        }

        Ok(())
    }
}

fn write_objects(
    buffer: &mut impl std::io::Write,
    objects: &[rusoto_s3::Object],
) -> Option<String> {
    for object in objects {
        let key = object.key.as_deref().unwrap_or_else(|| "");

        let _ = writeln!(buffer, "{}", key);
    }

    let _ = buffer.flush();

    // return the last detected key
    objects
        .last()
        .and_then(|obj| obj.key.as_ref().map(|o| o.to_string()))
}
