use crate::source::DocumentSource;
use crate::strategy::{FetchResources, Strategy};
use crate::{Channel, ReleaseIndex, TResult};

pub struct DistIndex {
    source: DocumentSource,
}

impl Strategy for DistIndex {
    fn build_index(&self) -> TResult<ReleaseIndex> {
        unimplemented!()
    }
}

impl FetchResources for DistIndex {
    fn fetch_channel(channel: Channel) -> TResult<Self> {
        unimplemented!()
    }
}
