//! Defines i/o data structures and routines used by various [`Source`] implementations
//!
//! [`Source`]: https://docs.rs/rust-releases/latest/rust_releases/source/index.html
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(unsafe_code)]

mod client;
mod document;
mod io;
mod transport;

pub use crate::{
    client::{AsyncRustReleasesClient, ResourceFile, RustReleasesClient},
    document::{Document, RetrievalLocation, RetrievedDocument},
    io::{BaseCacheDirError, IsStaleError, base_cache_dir, is_stale},
};

pub use crate::client::errors::IoError;
pub use crate::client::{cached_client::HttpCachedClient, cached_client::HttpCachedClientError};
pub use crate::client::{fs_client::FsClient, fs_client::FsClientError};
pub use crate::client::{remote_client::ClientError, remote_client::HttpClient};

pub use crate::transport::{
    AsyncHttpTransport, BoxFuture, HttpTransport, TransportError, TransportResult, timeout::Timeout,
};

#[cfg(feature = "reqwest")]
pub use crate::transport::transport_reqwest::ReqwestTransport;
#[cfg(feature = "ureq")]
pub use crate::transport::transport_ureq::UreqTransport;
