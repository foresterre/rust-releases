#![deny(clippy::all)]
#![allow(clippy::upper_case_acronyms)]

use crate::{Channel, ReleaseIndex, TResult};

pub use channel_manifests::{ChannelManifests, ChannelManifestsError};
pub use rust_changelog::{RustChangelog, RustChangelogError};
pub use rust_dist::{RustDist, RustDistError};
pub use rust_dist_with_cli::{DistIndexError, RustDistWithCLI};

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

pub mod channel_manifests;
pub mod rust_changelog;
pub mod rust_dist;
pub mod rust_dist_with_cli;

pub trait Source {
    fn build_index(&self) -> TResult<ReleaseIndex>;
}

pub trait FetchResources
where
    Self: Sized,
{
    fn fetch_channel(channel: Channel) -> TResult<Self>;
}

pub(crate) const DEFAULT_MEMORY_SIZE: usize = 4096;

/// A `Document` represents a resource which can be used as an input to construct a `ReleaseIndex`.
#[derive(Debug, Eq, PartialEq)]
pub enum Document {
    /// To be used when the document is present on disk (e.g. if pulled from the cache),
    ///  or accessible locally.
    LocalPath(PathBuf),
    /// To be used when the document has just been downloaded from a remote.
    /// The `PathBuf` represents the path to which the document contents were written (as cache).
    /// The `Vec<u8>` represents the document contents, so the just downloaded file doesn't have to
    ///  be written to the cache location, and read again.
    RemoteCached(PathBuf, Vec<u8>),
}

impl Document {
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
