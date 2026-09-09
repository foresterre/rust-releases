use crate::manifest::ReleaseManifest;
use rust_releases_core::{Beta, BetaReleases, Nightly, NightlyReleases, Stable, StableReleases};
use rust_releases_io::BoxFuture;

// Reads the releases of the Rust distribution.
//
// The methods which fetch a channel state the version of every release, and the release date where
// the distribution names it. The release date and toolchains of a release which does not carry them
// are held by its release manifest, which is fetched per release.
pub trait DistClient {
    type Error;

    fn stable_releases(&self) -> Result<StableReleases, Self::Error>;

    fn beta_releases(&self) -> Result<BetaReleases, Self::Error>;

    fn nightly_releases(&self) -> Result<NightlyReleases, Self::Error>;

    fn stable_manifest(&self, version: &Stable) -> Result<ReleaseManifest, Self::Error>;

    fn beta_manifest(&self, version: &Beta) -> Result<ReleaseManifest, Self::Error>;

    fn nightly_manifest(&self, version: &Nightly) -> Result<ReleaseManifest, Self::Error>;
}

pub trait AsyncDistClient: Send + Sync {
    type Error;

    fn stable_releases(&self) -> BoxFuture<'_, Result<StableReleases, Self::Error>>;

    fn beta_releases(&self) -> BoxFuture<'_, Result<BetaReleases, Self::Error>>;

    fn nightly_releases(&self) -> BoxFuture<'_, Result<NightlyReleases, Self::Error>>;

    fn stable_manifest<'a>(
        &'a self,
        version: &'a Stable,
    ) -> BoxFuture<'a, Result<ReleaseManifest, Self::Error>>;

    fn beta_manifest<'a>(
        &'a self,
        version: &'a Beta,
    ) -> BoxFuture<'a, Result<ReleaseManifest, Self::Error>>;

    fn nightly_manifest<'a>(
        &'a self,
        version: &'a Nightly,
    ) -> BoxFuture<'a, Result<ReleaseManifest, Self::Error>>;
}
