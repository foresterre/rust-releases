//! Defines the core routines required to implement a [`Source`].
//!
//! Please, see the [`rust-releases`] for additional documentation on how this crate can be used.
//!
//! [`Source`]: crate::Source
//! [`rust-releases`]: https://docs.rs/rust-releases
// #![deny(missing_docs)]
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

/// TODO docs
pub trait IndexBuilder {
    /// The error to be returned when an index can not be build for a source.
    type Error;

    /// Build a release index from a data set.
    fn build_index<T: Resource>(&self, resource: T) -> Result<ReleaseIndex, Self::Error>;
}

/// todo
pub trait Resource {
    fn consume(self) -> Self;

    fn read(&self) -> &Self;

    fn from_spec<Spec: ResourceSpec>(spec: Spec) -> Self;
}

pub struct ResourceBuilder<R: Resource> {
    into_resource: R,
}

impl<R: Resource> ResourceSpec for ResourceBuilder<R> {
    fn add_toolchain(&mut self, toolchain: ()) {}
}

pub trait ResourceSpec {
    fn add_toolchain(&mut self, toolchain: ());
}

/// With `FetchResources`, the set of inputs required to build a release index can be fetched.
pub trait FetchResources
where
    Self: Sized,
{
    /// The error to be returned when a resource can not be fetched.
    type Error;

    /// Fetch a set of inputs for a release channel.
    fn fetch_channel<R: Resource>(&self, channel: Channel) -> Result<R, Self::Error>;
}
