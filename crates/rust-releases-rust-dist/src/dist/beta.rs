use crate::client::{AsyncDistClient, DistClient};
use crate::dist::RustDist;
use crate::dist::extend;
use crate::manifest::{Detail, ReleaseManifest};
use rust_releases_core::{Beta, BetaReleases, RustRelease};

#[derive(Clone, Debug)]
pub struct BetaDist<'dist, C> {
    client: &'dist C,
}

impl<C> RustDist<C> {
    pub fn beta(&self) -> BetaDist<'_, C> {
        BetaDist {
            client: &self.client,
        }
    }
}

impl<'dist, C> BetaDist<'dist, C> {
    pub fn client(&self) -> &'dist C {
        self.client
    }
}

impl<C: DistClient> BetaDist<'_, C> {
    pub fn fetch(&self) -> Result<BetaReleases, C::Error> {
        self.client.beta_releases()
    }

    pub fn fetch_detailed(&self, detail: Detail) -> Result<BetaReleases, C::Error> {
        self.extend_all(self.fetch()?, detail)
    }

    pub fn manifest(&self, version: &Beta) -> Result<ReleaseManifest, C::Error> {
        self.client.beta_manifest(version)
    }

    pub fn extend(
        &self,
        release: RustRelease<Beta>,
        detail: Detail,
    ) -> Result<RustRelease<Beta>, C::Error> {
        extend::extend(self.client, release, detail, C::beta_manifest)
    }

    pub fn extend_all(
        &self,
        releases: BetaReleases,
        detail: Detail,
    ) -> Result<BetaReleases, C::Error> {
        extend::extend_all(self.client, releases, detail, C::beta_manifest)
    }
}

impl<C: AsyncDistClient> BetaDist<'_, C> {
    pub async fn fetch_async(&self) -> Result<BetaReleases, C::Error> {
        self.client.beta_releases().await
    }

    pub async fn fetch_detailed_async(&self, detail: Detail) -> Result<BetaReleases, C::Error> {
        self.extend_all_async(self.fetch_async().await?, detail)
            .await
    }

    pub async fn manifest_async(&self, version: &Beta) -> Result<ReleaseManifest, C::Error> {
        self.client.beta_manifest(version).await
    }

    pub async fn extend_async(
        &self,
        release: RustRelease<Beta>,
        detail: Detail,
    ) -> Result<RustRelease<Beta>, C::Error> {
        extend::extend_async(self.client, release, detail, C::beta_manifest).await
    }

    pub async fn extend_all_async(
        &self,
        releases: BetaReleases,
        detail: Detail,
    ) -> Result<BetaReleases, C::Error> {
        extend::extend_all_async(self.client, releases, detail, C::beta_manifest).await
    }
}
