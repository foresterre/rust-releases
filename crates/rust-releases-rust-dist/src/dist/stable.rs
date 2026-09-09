use crate::client::{AsyncDistClient, DistClient};
use crate::dist::RustDist;
use crate::dist::extend;
use crate::manifest::{Detail, ReleaseManifest};
use rust_releases_core::{RustRelease, Stable, StableReleases};

#[derive(Clone, Debug)]
pub struct StableDist<'dist, C> {
    client: &'dist C,
}

impl<C> RustDist<C> {
    pub fn stable(&self) -> StableDist<'_, C> {
        StableDist {
            client: &self.client,
        }
    }
}

impl<'dist, C> StableDist<'dist, C> {
    pub fn client(&self) -> &'dist C {
        self.client
    }
}

impl<C: DistClient> StableDist<'_, C> {
    pub fn fetch(&self) -> Result<StableReleases, C::Error> {
        self.client.stable_releases()
    }

    pub fn fetch_detailed(&self, detail: Detail) -> Result<StableReleases, C::Error> {
        self.extend_all(self.fetch()?, detail)
    }

    pub fn manifest(&self, version: &Stable) -> Result<ReleaseManifest, C::Error> {
        self.client.stable_manifest(version)
    }

    pub fn extend(
        &self,
        release: RustRelease<Stable>,
        detail: Detail,
    ) -> Result<RustRelease<Stable>, C::Error> {
        extend::extend(self.client, release, detail, C::stable_manifest)
    }

    pub fn extend_all(
        &self,
        releases: StableReleases,
        detail: Detail,
    ) -> Result<StableReleases, C::Error> {
        extend::extend_all(self.client, releases, detail, C::stable_manifest)
    }
}

impl<C: AsyncDistClient> StableDist<'_, C> {
    pub async fn fetch_async(&self) -> Result<StableReleases, C::Error> {
        self.client.stable_releases().await
    }

    pub async fn fetch_detailed_async(&self, detail: Detail) -> Result<StableReleases, C::Error> {
        self.extend_all_async(self.fetch_async().await?, detail)
            .await
    }

    pub async fn manifest_async(&self, version: &Stable) -> Result<ReleaseManifest, C::Error> {
        self.client.stable_manifest(version).await
    }

    pub async fn extend_async(
        &self,
        release: RustRelease<Stable>,
        detail: Detail,
    ) -> Result<RustRelease<Stable>, C::Error> {
        extend::extend_async(self.client, release, detail, C::stable_manifest).await
    }

    pub async fn extend_all_async(
        &self,
        releases: StableReleases,
        detail: Detail,
    ) -> Result<StableReleases, C::Error> {
        extend::extend_all_async(self.client, releases, detail, C::stable_manifest).await
    }
}
