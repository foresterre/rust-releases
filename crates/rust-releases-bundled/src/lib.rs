#![deny(clippy::all)]
#![deny(unsafe_code)]

mod generated;
mod source;
mod table;

pub use crate::source::BundledReleases;
