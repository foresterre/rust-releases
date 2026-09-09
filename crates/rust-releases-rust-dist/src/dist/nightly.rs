use crate::client::{AsyncDistClient, DistClient};
use crate::dist::RustDist;
use crate::dist::extend;
use crate::manifest::{Detail, ReleaseManifest};
use rust_releases_core::{Nightly, NightlyReleases, RustRelease};

#[derive(Clone, Debug)]
pub struct NightlyDist<'dist, C> {
    client: &'dist C,
}

impl<C> RustDist<C> {
    pub fn nightly(&self) -> NightlyDist<'_, C> {
        NightlyDist {
            client: &self.client,
        }
    }
}

impl<'dist, C> NightlyDist<'dist, C> {
    pub fn client(&self) -> &'dist C {
        self.client
    }
}

impl<C: DistClient> NightlyDist<'_, C> {
    pub fn fetch(&self) -> Result<NightlyReleases, C::Error> {
        self.client.nightly_releases()
    }

    pub fn fetch_detailed(&self, detail: Detail) -> Result<NightlyReleases, C::Error> {
        self.extend_all(self.fetch()?, detail)
    }

    pub fn manifest(&self, version: &Nightly) -> Result<ReleaseManifest, C::Error> {
        self.client.nightly_manifest(version)
    }

    pub fn extend(
        &self,
        release: RustRelease<Nightly>,
        detail: Detail,
    ) -> Result<RustRelease<Nightly>, C::Error> {
        extend::extend(self.client, release, detail, C::nightly_manifest)
    }

    pub fn extend_all(
        &self,
        releases: NightlyReleases,
        detail: Detail,
    ) -> Result<NightlyReleases, C::Error> {
        extend::extend_all(self.client, releases, detail, C::nightly_manifest)
    }
}

impl<C: AsyncDistClient> NightlyDist<'_, C> {
    pub async fn fetch_async(&self) -> Result<NightlyReleases, C::Error> {
        self.client.nightly_releases().await
    }

    pub async fn fetch_detailed_async(&self, detail: Detail) -> Result<NightlyReleases, C::Error> {
        self.extend_all_async(self.fetch_async().await?, detail)
            .await
    }

    pub async fn manifest_async(&self, version: &Nightly) -> Result<ReleaseManifest, C::Error> {
        self.client.nightly_manifest(version).await
    }

    pub async fn extend_async(
        &self,
        release: RustRelease<Nightly>,
        detail: Detail,
    ) -> Result<RustRelease<Nightly>, C::Error> {
        extend::extend_async(self.client, release, detail, C::nightly_manifest).await
    }

    pub async fn extend_all_async(
        &self,
        releases: NightlyReleases,
        detail: Detail,
    ) -> Result<NightlyReleases, C::Error> {
        extend::extend_all_async(self.client, releases, detail, C::nightly_manifest).await
    }
}
