use crate::{Channel, ReleaseIndex, TResult};

pub mod from_manifests;

pub trait Strategy {
    fn build_index(&self) -> TResult<ReleaseIndex>;
}

pub trait FetchResources
where
    Self: Sized,
{
    fn fetch_channel(channel: Channel) -> TResult<Self>;
}
