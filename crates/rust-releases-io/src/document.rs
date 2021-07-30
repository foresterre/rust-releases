use crate::IoResult;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

pub(crate) const DEFAULT_MEMORY_SIZE: usize = 4096;

/// A `Document` represents a resource which can be used as an input to construct a `ReleaseIndex`.
#[derive(Debug, Eq, PartialEq)]
pub enum Document {
    /// This variant can be used when the document is present on disk (e.g. if pulled from the cache),
    ///  or accessible locally by following a path.
    LocalPath(PathBuf),
    /// This variant can be used when the document is present in memory
    Memory(Vec<u8>),
    /// This variant can be used when the document has just been downloaded from a remote server,
    /// but had to be both written to disk, and used immediately (often in combination with something
    /// that implements [`Write`]).
    ///
    /// The `PathBuf` represents the path to which the document contents were written (e.g. as cache).
    /// The `Vec<u8>` represents the document contents, so the just downloaded file doesn't have to
    ///  be written to the cache location, and read again.
    ///
    /// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
    RemoteCached(PathBuf, Vec<u8>),
}

impl Document {
    /// Load the document to a buffer consisting of bytes (`u8`).
    // FIXME: may clone the resource, maybe also provide a version which does not clone?
    pub fn load(&self) -> IoResult<Vec<u8>> {
        match self {
            Self::LocalPath(path) => Self::read_from_path(path),
            Self::Memory(buffer) => Ok(buffer.to_owned()),
            Self::RemoteCached(_, buffer) => Ok(buffer.to_owned()),
        }
    }

    fn read_from_path(path: &Path) -> IoResult<Vec<u8>> {
        let mut reader = BufReader::new(File::open(path)?);

        let mut memory = Vec::with_capacity(DEFAULT_MEMORY_SIZE);
        reader.read_to_end(&mut memory)?;

        Ok(memory)
    }
}
