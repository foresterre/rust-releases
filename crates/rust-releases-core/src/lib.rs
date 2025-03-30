//! Defines the core routines required to implement a [`Source`].
//!
//! Please, see the [`rust-releases`] for additional documentation on how this crate can be used.
//!
//! [`Source`]: crate::Source
//! [`rust-releases`]: https://docs.rs/rust-releases
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(unsafe_code)]

/// Defines release channels, such as the stable, beta and nightly release channels.
pub(crate) mod channel;

/// Errors for this crate.
pub(crate) mod errors;

/// Defines the `ReleaseIndex`.
pub(crate) mod index;

/// Defines a `Release`
pub(crate) mod release;

/// Re-export the semver crate to the root scope
pub use semver;

pub use crate::{
    channel::Channel, errors::CoreError, errors::CoreResult, index::ReleaseIndex, release::Release,
};

/// A `Source` is a set of inputs from which a release index can be built.
pub trait Source {
    /// The error to be returned when an index can not be build for a source.
    type Error;

    /// Build a release index from a data set.
    fn build_index(&self) -> Result<ReleaseIndex, Self::Error>;
}
