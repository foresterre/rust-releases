use crate::client::{AsyncDistIndexClient, DistIndexClient};
use crate::error::RustDistError;
use crate::index;
use rust_releases_core::StableReleases;
use rust_releases_io::Document;

#[cfg(feature = "aws")]
use crate::aws::{AwsError, AwsIndexClient, BlockingAwsIndexClient};
#[cfg(feature = "aws")]
use crate::cache::CachedDistIndexClient;
#[cfg(feature = "aws")]
use rust_releases_io::{BaseCacheDirError, base_cache_dir};
#[cfg(feature = "aws")]
use std::path::PathBuf;
#[cfg(feature = "aws")]
use std::time::Duration;

#[cfg(feature = "aws")]
const DEFAULT_CACHE_FOLDER: &str = "source_dist_index";

#[cfg(feature = "aws")]
const DEFAULT_CACHE_FILE: &str = "dist_static-rust-lang-org.txt";

#[cfg(feature = "aws")]
const DEFAULT_CACHE_TIMEOUT: Duration = Duration::from_secs(86_400);

#[derive(Clone, Debug)]
pub struct RustDist<C> {
    client: C,
}

impl<C> RustDist<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }

    pub fn client(&self) -> &C {
        &self.client
    }
}

impl<C: DistIndexClient> RustDist<C> {
    pub fn fetch(&self) -> Result<StableReleases, RustDistError<C::Error>> {
        let index = self.client.download().map_err(RustDistError::Download)?;

        collect_index(&index)
    }
}

impl<C: AsyncDistIndexClient> RustDist<C> {
    pub async fn fetch_async(&self) -> Result<StableReleases, RustDistError<C::Error>> {
        let index = self
            .client
            .download()
            .await
            .map_err(RustDistError::Download)?;

        collect_index(&index)
    }
}

#[cfg(feature = "aws")]
impl RustDist<CachedDistIndexClient<BlockingAwsIndexClient>> {
    pub fn new_aws_cached_client() -> Result<Self, AwsSetupError> {
        let client = BlockingAwsIndexClient::new()?;

        Ok(Self::new(CachedDistIndexClient::new(
            client,
            default_cache_file()?,
            DEFAULT_CACHE_TIMEOUT,
        )))
    }
}

#[cfg(feature = "aws")]
impl RustDist<CachedDistIndexClient<AwsIndexClient>> {
    pub async fn new_async_aws_cached_client() -> Result<Self, AwsSetupError> {
        let client = AwsIndexClient::new().await?;

        Ok(Self::new(CachedDistIndexClient::new(
            client,
            default_cache_file()?,
            DEFAULT_CACHE_TIMEOUT,
        )))
    }
}

fn collect_index<E>(index: &Document) -> Result<StableReleases, RustDistError<E>> {
    index::stable_releases(index.buffer()).map_err(RustDistError::Parse)
}

#[cfg(feature = "aws")]
fn default_cache_file() -> Result<PathBuf, BaseCacheDirError> {
    // Here we use a mutable PathBuf, and push to it.
    // If we would have used base.join(dir).join(file), we would obtain the same result,
    // but in a less efficient manner, because join takes the previous path by reference
    // and converts it to a PathBuf internally.
    let mut base = base_cache_dir()?;
    base.push(DEFAULT_CACHE_FOLDER);
    base.push(DEFAULT_CACHE_FILE);
    Ok(base)
}

#[cfg(feature = "aws")]
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum AwsSetupError {
    #[error(transparent)]
    Aws(#[from] AwsError),

    #[error(transparent)]
    BaseCacheDir(#[from] BaseCacheDirError),
}
