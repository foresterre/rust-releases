use crate::TResult;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

pub(crate) const DEFAULT_MEMORY_SIZE: usize = 4096;

/// A DocumentSource represents a location from which a document can be accessed.
#[derive(Debug, Eq, PartialEq)]
pub enum DocumentSource {
    /// To be used when the document is present on disk (e.g. if pulled from the cache),
    ///  or accessible locally.
    LocalPath(PathBuf),
    /// To be used when the document has just been downloaded from a remote.
    /// The `PathBuf` represents the path to which the document contents were written (as cache).
    /// The `Vec<u8>` represents the document contents, so the just downloaded file doesn't have to
    ///  be written to the cache location, and read again.
    RemoteCached(PathBuf, Vec<u8>),
}

impl DocumentSource {
    pub fn load(&self) -> TResult<Vec<u8>> {
        match self {
            Self::LocalPath(path) => Self::read_all_from_disk(&path),
            Self::RemoteCached(_, buffer) => Ok(buffer.to_owned()),
        }
    }

    fn read_all_from_disk(path: &Path) -> TResult<Vec<u8>> {
        let mut reader = BufReader::new(File::open(path)?);

        let mut memory = Vec::with_capacity(DEFAULT_MEMORY_SIZE);
        reader.read_to_end(&mut memory)?;

        Ok(memory)
    }
}
