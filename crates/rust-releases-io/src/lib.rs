//! Defines i/o data structures and routines used by various [`Source`] implementations
//!
//! [`Source`]: https://docs.rs/rust-releases/latest/rust_releases/source/index.html
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(unsafe_code)]

mod client;
mod document;
mod io;

pub use crate::{
    client::{ResourceFile, RustReleasesClient},
    document::{Document, RetrievalLocation, RetrievedDocument},
    io::{base_cache_dir, is_stale, BaseCacheDirError, IsStaleError},
};

pub use crate::client::{cached_client::HttpCachedClient, cached_client::HttpCachedClientError};
pub use crate::client::{fs_client::FsClient, fs_client::FsClientError};
pub use crate::client::{remote_client::ClientError, remote_client::HttpClient};
