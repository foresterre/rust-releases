//! Defines i/o data structures and routines used by various [`Source`] implementations
//!
//! [`Source`]: https://docs.rs/rust-releases/latest/rust_releases/source/index.html
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(unsafe_code)]

const DEFAULT_MEMORY_SIZE: usize = 4096;

mod client;
mod document;
mod io;

pub use crate::{
    client::{ResourceFile, RustReleasesClient},
    document::{Document, RetrievalLocation, RetrievedDocument},
    io::{base_cache_dir, is_stale, BaseCacheDirError, IsStaleError},
};

#[cfg(feature = "http_client")]
pub use crate::client::cached_client::{CachedClient, CachedClientError};

/// A macro used to feature gate tests which fetch resources from third party services.
///
/// NB: for internal use, not covered by semver.
#[macro_export]
macro_rules! __internal_dl_test {
    ($expr:expr) => {{
        if cfg!(feature = "internal_dl_test") || option_env!("RUST_RELEASES_RUN_DL_TEST").is_some()
        {
            $expr
        }
    }};
}
