use crate::source::DocumentSource;
use crate::strategy::{FetchResources, Strategy};
use crate::{Channel, ReleaseIndex, TResult};

pub struct ReleasesMd {
    source: DocumentSource,
}

impl ReleasesMd {
    #[cfg(test)]
    pub(crate) fn from_document(source: DocumentSource) -> Self {
        Self { source }
    }
}

impl Strategy for ReleasesMd {
    fn build_index(&self) -> TResult<ReleaseIndex> {
        unimplemented!()
    }
}

impl FetchResources for ReleasesMd {
    fn fetch_channel(_channel: Channel) -> TResult<Self> {
        unimplemented!()
    }
}
