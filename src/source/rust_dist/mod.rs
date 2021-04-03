use crate::source::Document;
use crate::source::{FetchResources, Source};
use crate::{Channel, ReleaseIndex, TResult};

pub(in crate::source::rust_dist) mod dl;

pub struct RustDist {
    source: Document,
}

impl RustDist {
    #[cfg(test)]
    #[allow(unused)]
    pub(crate) fn from_document(source: Document) -> Self {
        Self { source }
    }
}

impl Source for RustDist {
    fn build_index(&self) -> TResult<ReleaseIndex> {
        let _source = self.source.load()?;

        todo!()
    }
}

impl FetchResources for RustDist {
    fn fetch_channel(channel: Channel) -> TResult<Self> {
        if let Channel::Stable = channel {
            let source = dl::fetch()?;
            Ok(Self { source })
        } else {
            Err(RustDistError::ChannelNotAvailable(channel).into())
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RustDistError {
    #[error("Channel {0} is not yet available for the 'DistIndex' source type")]
    ChannelNotAvailable(Channel),

    #[error("{0}")]
    RusotoTlsError(#[from] rusoto_core::request::TlsError),
}
