//! Defines i/o data structures and routines used by various [`Source`] implementations
//!
//! [`Source`]: https://docs.rs/rust-releases/latest/rust_releases/source/index.html
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(unsafe_code)]

pub(crate) mod document;
pub(crate) mod errors;
pub(crate) mod io;

pub use crate::{
    document::Document, errors::IoError, errors::IoResult, io::base_cache_dir, io::is_stale,
};

#[cfg(feature = "http_client")]
pub use crate::io::download_if_not_stale;

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
