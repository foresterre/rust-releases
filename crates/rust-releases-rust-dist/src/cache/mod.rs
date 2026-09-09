mod codec;
mod error;
mod paths;
mod store;

pub use crate::cache::codec::CacheEntryError;
pub use crate::cache::error::CachedDistError;

use crate::cache::store::{cached_manifest, cached_releases, store};
use crate::client::{AsyncDistClient, DistClient};
use crate::manifest::ReleaseManifest;
use rust_releases_core::{Beta, BetaReleases, Nightly, NightlyReleases, Stable, StableReleases};
use rust_releases_io::BoxFuture;
use std::path::{Path, PathBuf};
use std::time::Duration;

// A channel takes a request per thousand objects of the bucket to list, and a release manifest is
// about a megabyte, so what the client read is kept on disk, and re-used until it goes stale. The
// releases of a channel go stale sooner than a release manifest: a published manifest does not
// change, while a new release joins its channel.
pub const DEFAULT_RELEASES_TIMEOUT: Duration = Duration::from_secs(86_400);

pub const DEFAULT_MANIFEST_TIMEOUT: Duration = Duration::from_secs(2_592_000);

#[derive(Clone, Debug)]
pub struct CachedDistClient<C> {
    client: C,
    cache_folder: PathBuf,
    releases_timeout: Duration,
    manifest_timeout: Duration,
}

impl<C> CachedDistClient<C> {
    pub fn new(client: C, cache_folder: PathBuf) -> Self {
        Self {
            client,
            cache_folder,
            releases_timeout: DEFAULT_RELEASES_TIMEOUT,
            manifest_timeout: DEFAULT_MANIFEST_TIMEOUT,
        }
    }

    pub fn with_releases_timeout(self, releases_timeout: Duration) -> Self {
        Self {
            releases_timeout,
            ..self
        }
    }

    pub fn with_manifest_timeout(self, manifest_timeout: Duration) -> Self {
        Self {
            manifest_timeout,
            ..self
        }
    }

    pub fn client(&self) -> &C {
        &self.client
    }

    pub fn cache_folder(&self) -> &Path {
        &self.cache_folder
    }

    pub fn releases_timeout(&self) -> Duration {
        self.releases_timeout
    }

    pub fn manifest_timeout(&self) -> Duration {
        self.manifest_timeout
    }
}

impl<C: DistClient> DistClient for CachedDistClient<C> {
    type Error = CachedDistError<C::Error>;

    fn stable_releases(&self) -> Result<StableReleases, Self::Error> {
        let cache_file = self.stable_releases_cache_file();

        if let Some(cached) = cached_releases(&cache_file, self.releases_timeout)? {
            return Ok(cached);
        }

        let releases = self
            .client
            .stable_releases()
            .map_err(CachedDistError::Client)?;

        store(&cache_file, &codec::encode(&releases))?;

        Ok(releases)
    }

    fn beta_releases(&self) -> Result<BetaReleases, Self::Error> {
        let cache_file = self.beta_releases_cache_file();

        if let Some(cached) = cached_releases(&cache_file, self.releases_timeout)? {
            return Ok(cached);
        }

        let releases = self
            .client
            .beta_releases()
            .map_err(CachedDistError::Client)?;

        store(&cache_file, &codec::encode(&releases))?;

        Ok(releases)
    }

    fn nightly_releases(&self) -> Result<NightlyReleases, Self::Error> {
        let cache_file = self.nightly_releases_cache_file();

        if let Some(cached) = cached_releases(&cache_file, self.releases_timeout)? {
            return Ok(cached);
        }

        let releases = self
            .client
            .nightly_releases()
            .map_err(CachedDistError::Client)?;

        store(&cache_file, &codec::encode(&releases))?;

        Ok(releases)
    }

    fn stable_manifest(&self, version: &Stable) -> Result<ReleaseManifest, Self::Error> {
        let cache_file = self.stable_manifest_cache_file(version);

        if let Some(cached) = cached_manifest(&cache_file, self.manifest_timeout)? {
            return Ok(cached);
        }

        let manifest = self
            .client
            .stable_manifest(version)
            .map_err(CachedDistError::Client)?;

        store(&cache_file, manifest.buffer())?;

        Ok(manifest)
    }

    fn beta_manifest(&self, version: &Beta) -> Result<ReleaseManifest, Self::Error> {
        let cache_file = self.beta_manifest_cache_file(version);

        if let Some(cached) = cached_manifest(&cache_file, self.manifest_timeout)? {
            return Ok(cached);
        }

        let manifest = self
            .client
            .beta_manifest(version)
            .map_err(CachedDistError::Client)?;

        store(&cache_file, manifest.buffer())?;

        Ok(manifest)
    }

    fn nightly_manifest(&self, version: &Nightly) -> Result<ReleaseManifest, Self::Error> {
        let cache_file = self.nightly_manifest_cache_file(version);

        if let Some(cached) = cached_manifest(&cache_file, self.manifest_timeout)? {
            return Ok(cached);
        }

        let manifest = self
            .client
            .nightly_manifest(version)
            .map_err(CachedDistError::Client)?;

        store(&cache_file, manifest.buffer())?;

        Ok(manifest)
    }
}

impl<C: AsyncDistClient> AsyncDistClient for CachedDistClient<C> {
    type Error = CachedDistError<C::Error>;

    fn stable_releases(&self) -> BoxFuture<'_, Result<StableReleases, Self::Error>> {
        Box::pin(async move {
            let cache_file = self.stable_releases_cache_file();

            if let Some(cached) = cached_releases(&cache_file, self.releases_timeout)? {
                return Ok(cached);
            }

            let releases = self
                .client
                .stable_releases()
                .await
                .map_err(CachedDistError::Client)?;

            store(&cache_file, &codec::encode(&releases))?;

            Ok(releases)
        })
    }

    fn beta_releases(&self) -> BoxFuture<'_, Result<BetaReleases, Self::Error>> {
        Box::pin(async move {
            let cache_file = self.beta_releases_cache_file();

            if let Some(cached) = cached_releases(&cache_file, self.releases_timeout)? {
                return Ok(cached);
            }

            let releases = self
                .client
                .beta_releases()
                .await
                .map_err(CachedDistError::Client)?;

            store(&cache_file, &codec::encode(&releases))?;

            Ok(releases)
        })
    }

    fn nightly_releases(&self) -> BoxFuture<'_, Result<NightlyReleases, Self::Error>> {
        Box::pin(async move {
            let cache_file = self.nightly_releases_cache_file();

            if let Some(cached) = cached_releases(&cache_file, self.releases_timeout)? {
                return Ok(cached);
            }

            let releases = self
                .client
                .nightly_releases()
                .await
                .map_err(CachedDistError::Client)?;

            store(&cache_file, &codec::encode(&releases))?;

            Ok(releases)
        })
    }

    fn stable_manifest<'a>(
        &'a self,
        version: &'a Stable,
    ) -> BoxFuture<'a, Result<ReleaseManifest, Self::Error>> {
        Box::pin(async move {
            let cache_file = self.stable_manifest_cache_file(version);

            if let Some(cached) = cached_manifest(&cache_file, self.manifest_timeout)? {
                return Ok(cached);
            }

            let manifest = self
                .client
                .stable_manifest(version)
                .await
                .map_err(CachedDistError::Client)?;

            store(&cache_file, manifest.buffer())?;

            Ok(manifest)
        })
    }

    fn beta_manifest<'a>(
        &'a self,
        version: &'a Beta,
    ) -> BoxFuture<'a, Result<ReleaseManifest, Self::Error>> {
        Box::pin(async move {
            let cache_file = self.beta_manifest_cache_file(version);

            if let Some(cached) = cached_manifest(&cache_file, self.manifest_timeout)? {
                return Ok(cached);
            }

            let manifest = self
                .client
                .beta_manifest(version)
                .await
                .map_err(CachedDistError::Client)?;

            store(&cache_file, manifest.buffer())?;

            Ok(manifest)
        })
    }

    fn nightly_manifest<'a>(
        &'a self,
        version: &'a Nightly,
    ) -> BoxFuture<'a, Result<ReleaseManifest, Self::Error>> {
        Box::pin(async move {
            let cache_file = self.nightly_manifest_cache_file(version);

            if let Some(cached) = cached_manifest(&cache_file, self.manifest_timeout)? {
                return Ok(cached);
            }

            let manifest = self
                .client
                .nightly_manifest(version)
                .await
                .map_err(CachedDistError::Client)?;

            store(&cache_file, manifest.buffer())?;

            Ok(manifest)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn client() -> CachedDistClient<()> {
        CachedDistClient::new((), PathBuf::from("/cache"))
    }

    #[test]
    fn the_default_timeouts() {
        assert_eq!(client().releases_timeout(), DEFAULT_RELEASES_TIMEOUT);
        assert_eq!(client().manifest_timeout(), DEFAULT_MANIFEST_TIMEOUT);
    }

    #[test]
    fn set_the_timeouts() {
        let client = client()
            .with_releases_timeout(Duration::from_secs(1))
            .with_manifest_timeout(Duration::from_secs(2));

        assert_eq!(client.releases_timeout(), Duration::from_secs(1));
        assert_eq!(client.manifest_timeout(), Duration::from_secs(2));
    }
}
