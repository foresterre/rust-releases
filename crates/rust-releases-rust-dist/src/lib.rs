#![deny(clippy::all)]
#![deny(unsafe_code)]
#![allow(clippy::upper_case_acronyms)]

mod cache;
mod client;
mod error;
mod index;
mod source;

#[cfg(feature = "aws")]
mod aws;

pub use crate::cache::{CachedDistIndexClient, CachedDistIndexError};
pub use crate::client::{AsyncDistIndexClient, DistIndexClient};
pub use crate::error::RustDistError;
pub use crate::index::DistIndexError;
pub use crate::source::RustDist;

#[cfg(feature = "aws")]
pub use crate::aws::{AwsError, AwsIndexClient, BlockingAwsIndexClient};

#[cfg(feature = "aws")]
pub use crate::source::AwsSetupError;
