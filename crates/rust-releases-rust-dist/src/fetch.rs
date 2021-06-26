// FIXME: This file should really be tested (with mocks), but I haven't had the time yet
//   to look for a proper mocking library for Rust yet. No excuses of course, ..., but personal project
//   and such 🙄. Instead, this text serves as a header of shame.

use crate::errors::{RustDistError, RustDistResult};
use rusoto_core::credential::{AwsCredentials, StaticProvider};
use rusoto_core::{HttpClient, Region};
use rusoto_s3::{ListObjectsV2Request, S3Client, S3};
use rust_releases_io::{base_cache_dir, is_stale, Document};
use std::convert::{TryFrom, TryInto};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

// Rust currently always uses the US West 1 bucket
const RUST_DIST_REGION: Region = Region::UsWest1;

// The bucket from which the official Rust sources are distributed
const RUST_DIST_BUCKET: &str = "static-rust-lang-org";

// We only request objects which start with the following string, which currently only matches stable
// releases
const OBJECT_PREFIX: &str = "dist/rustc-";

// Directory where cached files reside for this source
const SOURCE_CACHE_DIR: &str = "source_dist_index";

// The output file path
const OUTPUT_PATH: &str = "dist_static-rust-lang-org.txt";

// amount of objects requested per chunk
const REQUEST_SIZE: i64 = 1000;

// Use the filtered index cache for up to 1 day
const TIMEOUT: Duration = Duration::from_secs(86_400);

#[derive(Clone, Debug, Eq, PartialEq)]
enum ChunkState {
    // Contains the last key in the current chunk, which is the offset key for the next call
    Offset(String),
    Complete,
}

// Client used to obtain the rust releases meta data, part by part
trait ChunkClient {
    fn download_chunk(
        &self,
        offset: Option<impl Into<String>>,
        to: &mut impl Write,
    ) -> RustDistResult<ChunkState>;

    fn download(&self, to: &mut impl Write) -> RustDistResult<()>;
}

// The default Rust Releases client
struct Client {
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

// Buffer which writes to two endpoints one after the other.
// Does not do magic tricks. Currently for rust-releases the bottleneck is the throttling by AWS and
// the throughput of the network. Since local file speed is not yet a bottleneck, we can opt for a simple
// solution to keep the downloaded files in owned memory (i.e. ready to be used, without loading them from disk) and
// write them to disk so they are persistently cached (since the data does not change very often).
struct BiWriter<T1: Write, T2: Write> {
    buffer1: T1,
    buffer2: T2,
}

impl<T1: Write, T2: Write> Write for BiWriter<T1, T2> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let _ = self.buffer1.write(buf)?;
        self.buffer2.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buffer1.flush()?;
        self.buffer2.flush()
    }
}

impl BiWriter<BufWriter<Vec<u8>>, BufWriter<File>> {
    fn try_from_path(path: impl AsRef<Path>) -> RustDistResult<Self> {
        let memory = BufWriter::new(Vec::new());
        let file = BufWriter::new(OpenOptions::new().create(true).append(true).open(path)?);

        Ok(Self {
            buffer1: memory,
            buffer2: file,
        })
    }

    fn into_owned_memory(mut self) -> RustDistResult<Vec<u8>> {
        self.flush()?;

        self.buffer1.into_inner().map_err(RustDistError::from)
    }
}

/// A data structure which holds a writer which writes both to memory and to a file.
struct PersistingMemCache<P: AsRef<Path>> {
    cache_file: P,
    buffer: BiWriter<BufWriter<Vec<u8>>, BufWriter<File>>,
}

impl<P: AsRef<Path>> PersistingMemCache<P> {
    fn try_from_path(path: P) -> RustDistResult<Self> {
        let buffer = BiWriter::try_from_path(path.as_ref())?;

        Ok(Self {
            cache_file: path,
            buffer,
        })
    }
}

impl<P: AsRef<Path>> Write for PersistingMemCache<P> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buffer.flush()
    }
}

impl<P: AsRef<Path>> TryFrom<PersistingMemCache<P>> for Document {
    type Error = RustDistError;

    fn try_from(value: PersistingMemCache<P>) -> Result<Self, Self::Error> {
        let path = value.cache_file.as_ref().to_path_buf();
        let mem = value.buffer.into_owned_memory()?;

        Ok(Document::RemoteCached(path, mem))
    }
}

fn check_cache(output_path: &Path) -> RustDistResult<Option<Document>> {
    if output_path.is_file() && !is_stale(&output_path, TIMEOUT)? {
        Ok(Some(Document::LocalPath(output_path.to_path_buf())))
    } else {
        let parent = output_path.parent().ok_or_else(|| {
            let error: std::io::Error = std::io::ErrorKind::NotFound.into();
            error
        })?;

        std::fs::create_dir_all(parent)?;
        Ok(None)
    }
}

fn cache_file_path() -> RustDistResult<PathBuf> {
    // Here we use a mutable PathBuf, and push to it.
    // If we would have used base.join(dir).join(file), we would obtain the same result,
    // but in a less efficient manner, because join takes the previous path by reference
    // and converts it to a PathBuf internally.
    let mut base = base_cache_dir()?;
    base.push(SOURCE_CACHE_DIR);
    base.push(OUTPUT_PATH);
    Ok(base)
}

pub(in crate) fn fetch() -> RustDistResult<Document> {
    let output_path = cache_file_path()?;

    // Use the locally cached version if it exists, and is not stale
    if let Some(cached) = check_cache(&output_path)? {
        return Ok(cached);
    }

    let client = Client::try_default()?;
    let mut buffer = PersistingMemCache::try_from_path(output_path)?;

    client.download(&mut buffer)?;

    buffer.try_into()
}

fn write_objects(
    buffer: &mut impl std::io::Write,
    objects: &[rusoto_s3::Object],
) -> Option<String> {
    for object in objects {
        let key = object.key.clone().unwrap_or_else(|| "".to_string());

        let _ = buffer.write(format!("{}\n", key).as_bytes());
    }

    let _ = buffer.flush();

    // return the last detected key
    objects
        .last()
        .and_then(|obj| obj.key.as_ref().map(|o| o.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    // @runWith cargo test --all-features --package rust-releases --lib source::rust_dist_with_cli::dl::tests::test_fetch_meta_manifest -- --exact
    #[test]
    fn test_fetch_meta_manifest() {
        __internal_dl_test!({
            let meta = fetch();
            assert!(meta.is_ok());
        })
    }
}
