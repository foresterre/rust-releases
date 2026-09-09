#![deny(clippy::all)]
#![deny(unsafe_code)]
#![allow(clippy::upper_case_acronyms)]

// The trait the crate is built around, and the two implementations of it
mod cache;
mod client;

#[cfg(feature = "aws")]
mod aws;

// The releases of a channel, and the release manifest which details one of them
mod dist;
mod manifest;

// Shared by the modules above
mod date;
mod releases;
mod version;

pub use crate::cache::{
    CacheEntryError, CachedDistClient, CachedDistError, DEFAULT_MANIFEST_TIMEOUT,
    DEFAULT_RELEASES_TIMEOUT,
};
pub use crate::client::{AsyncDistClient, DistClient};
pub use crate::dist::{BetaDist, NightlyDist, RustDist, StableDist};
pub use crate::manifest::{Detail, ReleaseManifest, ReleaseManifestError};

#[cfg(feature = "aws")]
pub use crate::aws::{AwsDistClient, AwsError, BlockingAwsDistClient, ManifestIndexError};

#[cfg(feature = "aws")]
pub use crate::dist::AwsSetupError;
