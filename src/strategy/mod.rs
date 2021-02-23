use crate::{Channel, ReleaseIndex, TResult};

pub mod dist_index;
pub mod from_manifests;
pub mod releases_md;

pub trait Strategy {
    fn build_index(&self) -> TResult<ReleaseIndex>;
}

pub trait FetchResources
where
    Self: Sized,
{
    fn fetch_channel(channel: Channel) -> TResult<Self>;
}
