mod beta;
mod extend;
mod nightly;
mod stable;

pub use crate::dist::beta::BetaDist;
pub use crate::dist::nightly::NightlyDist;
pub use crate::dist::stable::StableDist;

#[cfg(feature = "aws")]
use crate::aws::{AwsDistClient, AwsError, BlockingAwsDistClient};
#[cfg(feature = "aws")]
use crate::cache::CachedDistClient;
#[cfg(feature = "aws")]
use rust_releases_io::{BaseCacheDirError, base_cache_dir};
#[cfg(feature = "aws")]
use std::path::PathBuf;

#[cfg(feature = "aws")]
const DEFAULT_CACHE_FOLDER: &str = "source_dist";

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

#[cfg(feature = "aws")]
impl RustDist<CachedDistClient<BlockingAwsDistClient>> {
    pub fn new_aws_cached_client() -> Result<Self, AwsSetupError> {
        let client = BlockingAwsDistClient::new()?;

        Ok(Self::new(CachedDistClient::new(
            client,
            default_cache_folder()?,
        )))
    }
}

#[cfg(feature = "aws")]
impl RustDist<CachedDistClient<AwsDistClient>> {
    pub async fn new_async_aws_cached_client() -> Result<Self, AwsSetupError> {
        let client = AwsDistClient::new().await?;

        Ok(Self::new(CachedDistClient::new(
            client,
            default_cache_folder()?,
        )))
    }
}

#[cfg(feature = "aws")]
fn default_cache_folder() -> Result<PathBuf, BaseCacheDirError> {
    // Here we use a mutable PathBuf, and push to it.
    // If we would have used base.join(dir), we would obtain the same result,
    // but in a less efficient manner, because join takes the previous path by reference
    // and converts it to a PathBuf internally.
    let mut base = base_cache_dir()?;
    base.push(DEFAULT_CACHE_FOLDER);
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
