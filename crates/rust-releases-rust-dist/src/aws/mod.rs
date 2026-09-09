mod bucket;
mod collect;
mod error;
mod keys;
mod manifest_index;

pub use crate::aws::error::AwsError;
pub use crate::aws::manifest_index::ManifestIndexError;

use crate::aws::bucket::RustDistBucket;
use crate::aws::manifest_index::NightlyManifestIndex;
use crate::client::{AsyncDistClient, DistClient};
use crate::manifest::ReleaseManifest;
use rust_releases_core::{Beta, BetaReleases, Nightly, NightlyReleases, Stable, StableReleases};
use rust_releases_io::BoxFuture;

// Reads the Rust distribution from the S3 bucket it is served from.
#[derive(Clone, Debug)]
pub struct AwsDistClient {
    bucket: RustDistBucket,
}

impl AwsDistClient {
    pub async fn new() -> Result<Self, AwsError> {
        Ok(Self {
            bucket: RustDistBucket::new().await?,
        })
    }

    pub fn with_client(client: aws_sdk_s3::Client) -> Self {
        Self {
            bucket: RustDistBucket::with_client(client),
        }
    }

    pub fn client(&self) -> &aws_sdk_s3::Client {
        self.bucket.client()
    }

    async fn stable(&self) -> Result<StableReleases, AwsError> {
        let manifests = self.bucket.list(keys::CHANNEL_MANIFEST_PREFIX).await?;

        let mut artifacts = Vec::new();

        for prefix in keys::artifact_prefixes_up_to_1d8d0() {
            artifacts.extend(self.bucket.list(&prefix).await?);
        }

        Ok(collect::create_stable_releases(
            manifests.iter().map(String::as_str),
            artifacts.iter().map(String::as_str),
        ))
    }

    async fn beta(&self) -> Result<BetaReleases, AwsError> {
        let manifests = self.bucket.list(keys::CHANNEL_MANIFEST_PREFIX).await?;

        Ok(collect::create_beta_releases(
            manifests.iter().map(String::as_str),
        ))
    }

    // The nightly releases come from the manifest index rather than from the dated folders of the
    // bucket: the index names the dates which actually published a nightly, where roughly one in
    // sixty dated folders publishes none, and it takes a single request instead of five. The dates
    // it omits, before 2016-03-08, publish a v1 manifest which this crate cannot read anyway.
    async fn nightly(&self) -> Result<NightlyReleases, AwsError> {
        let index = self.manifest_index().await?;

        Ok(collect::create_nightly_releases(index.nightly().cloned()))
    }

    async fn manifest_index(&self) -> Result<NightlyManifestIndex, AwsError> {
        let key = keys::MANIFEST_INDEX_KEY;
        let document = self.bucket.get(key).await?;

        NightlyManifestIndex::parse(document.buffer()).map_err(|source| AwsError::ManifestIndex {
            key: key.to_string(),
            source,
        })
    }

    async fn manifest(&self, key: String) -> Result<ReleaseManifest, AwsError> {
        let document = self.bucket.get(&key).await?;

        ReleaseManifest::parse(document.buffer())
            .map_err(|source| AwsError::ReleaseManifest { key, source })
    }
}

impl AsyncDistClient for AwsDistClient {
    type Error = AwsError;

    fn stable_releases(&self) -> BoxFuture<'_, Result<StableReleases, Self::Error>> {
        Box::pin(self.stable())
    }

    fn beta_releases(&self) -> BoxFuture<'_, Result<BetaReleases, Self::Error>> {
        Box::pin(self.beta())
    }

    fn nightly_releases(&self) -> BoxFuture<'_, Result<NightlyReleases, Self::Error>> {
        Box::pin(self.nightly())
    }

    fn stable_manifest<'a>(
        &'a self,
        version: &'a Stable,
    ) -> BoxFuture<'a, Result<ReleaseManifest, Self::Error>> {
        Box::pin(self.manifest(keys::stable_manifest(version)))
    }

    fn beta_manifest<'a>(
        &'a self,
        version: &'a Beta,
    ) -> BoxFuture<'a, Result<ReleaseManifest, Self::Error>> {
        Box::pin(self.manifest(keys::beta_manifest(version)))
    }

    fn nightly_manifest<'a>(
        &'a self,
        version: &'a Nightly,
    ) -> BoxFuture<'a, Result<ReleaseManifest, Self::Error>> {
        Box::pin(self.manifest(keys::nightly_manifest(version)))
    }
}

// Runs the asynchronous [`AwsDistClient`] on a runtime of its own, so the distribution can be read
// from a blocking context.
#[derive(Debug)]
pub struct BlockingAwsDistClient {
    client: AwsDistClient,
    runtime: tokio::runtime::Runtime,
}

impl BlockingAwsDistClient {
    pub fn new() -> Result<Self, AwsError> {
        let runtime = tokio::runtime::Runtime::new().map_err(AwsError::Runtime)?;
        let client = runtime.block_on(AwsDistClient::new())?;

        Ok(Self { client, runtime })
    }

    pub fn with_client(client: AwsDistClient) -> Result<Self, AwsError> {
        let runtime = tokio::runtime::Runtime::new().map_err(AwsError::Runtime)?;

        Ok(Self { client, runtime })
    }

    pub fn client(&self) -> &AwsDistClient {
        &self.client
    }
}

impl DistClient for BlockingAwsDistClient {
    type Error = AwsError;

    fn stable_releases(&self) -> Result<StableReleases, Self::Error> {
        self.runtime.block_on(self.client.stable())
    }

    fn beta_releases(&self) -> Result<BetaReleases, Self::Error> {
        self.runtime.block_on(self.client.beta())
    }

    fn nightly_releases(&self) -> Result<NightlyReleases, Self::Error> {
        self.runtime.block_on(self.client.nightly())
    }

    fn stable_manifest(&self, version: &Stable) -> Result<ReleaseManifest, Self::Error> {
        self.runtime
            .block_on(self.client.manifest(keys::stable_manifest(version)))
    }

    fn beta_manifest(&self, version: &Beta) -> Result<ReleaseManifest, Self::Error> {
        self.runtime
            .block_on(self.client.manifest(keys::beta_manifest(version)))
    }

    fn nightly_manifest(&self, version: &Nightly) -> Result<ReleaseManifest, Self::Error> {
        self.runtime
            .block_on(self.client.manifest(keys::nightly_manifest(version)))
    }
}
