use crate::errors::{AwsError, RustDistError, RustDistResult};
use aws_config::AppName;
use aws_sdk_s3::config::Region;
use aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Output;
use aws_sdk_s3::types::Object;
use rust_releases_io::{base_cache_dir, is_stale, Document};
use std::convert::{TryFrom, TryInto};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

// Rust currently always uses the US West 1 bucket
const RUST_DIST_REGION: Region = Region::from_static("us-west-1");

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
const REQUEST_SIZE: i32 = 1000;

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
    aws_s3_client: aws_sdk_s3::Client,
    runtime: tokio::runtime::Runtime,
}

impl Client {
    pub fn try_default() -> RustDistResult<Self> {
        let runtime = tokio::runtime::Runtime::new()?;

        let app_name = AppName::new("rust-releases+`github|foresterre|rust-releases`")
            .map_err(AwsError::InvalidAppName)?;

        let config = runtime.block_on(
            aws_config::from_env()
                .no_credentials()
                .app_name(app_name)
                .region(RUST_DIST_REGION)
                .load(),
        );

        let aws_s3_client = aws_sdk_s3::Client::new(&config);

        Ok(Self {
            aws_s3_client,
            runtime,
        })
    }
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

impl ChunkClient for Client {
    fn download_chunk(
        &self,
        offset: Option<impl Into<String>>,
        to: &mut impl Write,
    ) -> RustDistResult<ChunkState> {
        let raw = self
            .runtime
            .block_on(list_objects(&self.aws_s3_client, offset))?;

        let objects = raw.contents.ok_or(RustDistError::ChunkMetadataMissing)?;
        let state = match write_objects(to, &objects) {
            Some(key) => ChunkState::Offset(key),
            None => ChunkState::Complete,
        };

        Ok(state)
    }

    fn download(&self, to: &mut impl Write) -> RustDistResult<()> {
        let mut offset = None;

        while let ChunkState::Offset(next_offset) = self.download_chunk(offset.to_owned(), to)? {
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
struct PersistingMemCache {
    buffer: BiWriter<BufWriter<Vec<u8>>, BufWriter<File>>,
}

impl PersistingMemCache {
    fn try_from_path<P: AsRef<Path>>(path: P) -> RustDistResult<Self> {
        let buffer = BiWriter::try_from_path(path.as_ref())?;

        Ok(Self { buffer })
    }
}

impl Write for PersistingMemCache {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buffer.flush()
    }
}

impl TryFrom<PersistingMemCache> for Document {
    type Error = RustDistError;

    fn try_from(value: PersistingMemCache) -> Result<Self, Self::Error> {
        let mem = value.buffer.into_owned_memory()?;

        Ok(Document::new(mem))
    }
}

fn check_cache(output_path: &Path) -> RustDistResult<Option<Document>> {
    if output_path.is_file() && !is_stale(output_path, TIMEOUT)? {
        let buffer = fs::read(output_path)?;
        let document = Document::new(buffer);

        Ok(Some(document))
    } else {
        let parent = output_path.parent().ok_or_else(|| {
            let error: std::io::Error = std::io::ErrorKind::NotFound.into();
            error
        })?;

        fs::create_dir_all(parent)?;
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

pub(crate) fn fetch() -> RustDistResult<Document> {
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

#[cfg(test)]
mod tests {
    use super::*;

    // @runWith cargo test --all-features --package rust-releases --lib source::rust_dist::fetch::tests::test_fetch_meta_manifest -- --exact
    #[test]
    fn test_fetch_meta_manifest() {
        __internal_dl_test!({
            let meta = fetch();
            assert!(meta.is_ok());
        })
    }
}
