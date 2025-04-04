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

pub use crate::{
    channel::Channel, errors::CoreError, errors::CoreResult, index::ReleaseIndex, release::Release,
};
/// Re-export the semver crate to the root scope
pub use semver;
use std::collections::BTreeSet;
use std::rc::Rc;

/// TODO
pub struct RustReleases {
    stable: BTreeSet<rust_release::RustRelease<rust_release::Stable>>,
    beta: BTreeSet<rust_release::RustRelease<rust_release::Beta>>,
    nightly: BTreeSet<rust_release::RustRelease<rust_release::Nightly>>,
}

impl RustReleases {
    /// TODO
    pub fn stable(
        &self,
    ) -> impl IntoIterator<Item = &rust_release::RustRelease<rust_release::Stable>> {
        self.stable.iter()
    }

    /// TODO
    pub fn beta(
        &self,
    ) -> impl IntoIterator<Item = &rust_release::RustRelease<rust_release::Stable>> {
        self.stable.iter()
    }

    /// TODO
    pub fn nightly(
        &self,
    ) -> impl IntoIterator<Item = &rust_release::RustRelease<rust_release::Stable>> {
        self.stable.iter()
    }
}
