// FIXME: This file should really be tested (with mocks), but I haven't had the time yet
//   to look for a proper mocking library for Rust yet. No excuses of course, ..., but personal project
//   and such 🙄. Instead, this text serves as a header of shame.

use crate::download::{ChunkClient, Client};
use crate::errors::{RustDistError, RustDistResult};
use rust_releases_io::{base_cache_dir, is_stale, Document};
use std::convert::{TryFrom, TryInto};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

// Directory where cached files reside for this source
const SOURCE_CACHE_DIR: &str = "source_dist_index";

// The output file path
const OUTPUT_PATH: &str = "dist_static-rust-lang-org.txt";

// Use the filtered index cache for up to 1 day
const TIMEOUT: Duration = Duration::from_secs(86_400);

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
struct PersistingBuffer<P: AsRef<Path>> {
    cache_file: P,
    buffer: BiWriter<BufWriter<Vec<u8>>, BufWriter<File>>,
}

impl<P: AsRef<Path>> PersistingBuffer<P> {
    fn try_from_path(path: P) -> RustDistResult<Self> {
        let buffer = BiWriter::try_from_path(path.as_ref())?;

        Ok(Self {
            cache_file: path,
            buffer,
        })
    }
}

impl<P: AsRef<Path>> Write for PersistingBuffer<P> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buffer.flush()
    }
}

impl<P: AsRef<Path>> TryFrom<PersistingBuffer<P>> for Document {
    type Error = RustDistError;

    fn try_from(value: PersistingBuffer<P>) -> Result<Self, Self::Error> {
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
    let mut buffer = PersistingBuffer::try_from_path(output_path)?;

    client.download(&mut buffer)?;

    buffer.try_into()
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
