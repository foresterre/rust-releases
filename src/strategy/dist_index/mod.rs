use crate::source::DocumentSource;
use crate::strategy::{FetchResources, Strategy};
use crate::{Channel, Release, ReleaseIndex, TResult};
use std::collections::BTreeSet;

pub struct DistIndex {
    source: DocumentSource,
}

impl DistIndex {
    #[cfg(test)]
    pub(crate) fn from_document(source: DocumentSource) -> Self {
        Self { source }
    }
}

impl Strategy for DistIndex {
    fn build_index(&self) -> TResult<ReleaseIndex> {
        let contents = self.source.load()?;
        let content = String::from_utf8(contents).map_err(DistIndexError::UnrecognizedText)?;

        // NB: poor man's parsing for stable releases only
        let versions = content
            .lines()
            .filter(|s| !s.trim().starts_with("PRE"))
            .filter_map(|line| {
                line.split_ascii_whitespace()
                    .nth(3)
                    .filter(|s| s.starts_with("rust-1"))
            })
            .filter_map(|s| s.split('-').nth(1))
            .flat_map(|s| semver::Version::parse(s).map(Release::new))
            .collect::<BTreeSet<_>>();

        Ok(ReleaseIndex::new(versions))
    }
}

impl FetchResources for DistIndex {
    fn fetch_channel(_channel: Channel) -> TResult<Self> {
        unimplemented!()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DistIndexError {
    #[error("{0}")]
    UnrecognizedText(#[from] std::string::FromUtf8Error),
}

#[cfg(test)]
mod tests {
    use crate::source::DocumentSource;
    use crate::strategy::dist_index::DistIndex;
    use crate::ReleaseIndex;

    #[test]
    fn strategy_dist_index() {
        let expected_version = semver::Version::parse("1.0.0").unwrap();

        let path = [env!("CARGO_MANIFEST_DIR"), "/resources/dist_index/dist.txt"].join("");
        let strategy = DistIndex::from_document(DocumentSource::LocalPath(path.into()));
        let index = ReleaseIndex::with_strategy(strategy).unwrap();

        assert!(index.releases().len() > 50);
        assert_eq!(index.releases()[0].version(), &expected_version);
    }
}
