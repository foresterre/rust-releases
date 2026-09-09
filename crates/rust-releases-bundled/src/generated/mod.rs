mod metadata;

#[cfg(feature = "beta")]
mod beta;
#[cfg(feature = "nightly")]
mod nightly;
#[cfg(feature = "stable")]
mod stable;

pub use crate::generated::metadata::generated_on;

#[cfg(feature = "beta")]
pub use crate::generated::beta::releases as beta_releases;
#[cfg(feature = "nightly")]
pub use crate::generated::nightly::releases as nightly_releases;
#[cfg(feature = "stable")]
pub use crate::generated::stable::releases as stable_releases;
