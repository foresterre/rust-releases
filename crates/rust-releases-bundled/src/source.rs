use crate::generated;
use rust_releases_core::rust_release::date::Date;

#[cfg(feature = "beta")]
use rust_releases_core::BetaReleases;
#[cfg(feature = "nightly")]
use rust_releases_core::NightlyReleases;
#[cfg(feature = "stable")]
use rust_releases_core::StableReleases;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct BundledReleases;

impl BundledReleases {
    pub fn new() -> Self {
        Self
    }

    pub fn generated_on(&self) -> Date {
        generated::generated_on()
    }

    #[cfg(feature = "stable")]
    pub fn stable(&self) -> StableReleases {
        generated::stable_releases()
    }

    #[cfg(feature = "beta")]
    pub fn beta(&self) -> BetaReleases {
        generated::beta_releases()
    }

    #[cfg(feature = "nightly")]
    pub fn nightly(&self) -> NightlyReleases {
        generated::nightly_releases()
    }
}
