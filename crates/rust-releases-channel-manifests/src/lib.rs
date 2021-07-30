#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(unsafe_code)]
//! Please, see the [`rust-releases`] for additional documentation on how this crate can be used.
//!
//! [`rust-releases`]: https://docs.rs/rust-releases

#[cfg(test)]
#[macro_use]
extern crate rust_releases_io;

use crate::fetch::{fetch_meta_manifest, fetch_release_manifests};
use crate::meta_manifest::MetaManifest;
use crate::release_manifest::parse_release_manifest;
use rust_releases_core::{Channel, FetchResources, Release, ReleaseIndex, Source};
use rust_releases_io::Document;
use std::collections::BTreeSet;
use std::iter::FromIterator;

pub(crate) mod errors;
pub(crate) mod fetch;
pub(crate) mod meta_manifest;
pub(crate) mod release_manifest;

pub use errors::{ChannelManifestsError, ChannelManifestsResult};

/// A [`Source`] which parses Rust release data from channel manifests, which used to be published for
/// each release. Since 2020-02-23, however, no more channel manifests have been published.
///
///
/// See <a href="https://github.com/foresterre/rust-releases/issues/9">foresterre/rust-releases#9</a> which tracks the above issue.
///
/// This source should not be used anymore. Since the input data is outdated, this source has been deprecated.
/// It will receive no further development and may be removed in the future.
#[deprecated]
pub struct ChannelManifests {
    documents: Vec<Document>,
}

#[allow(deprecated)]
impl Source for ChannelManifests {
    type Error = ChannelManifestsError;

    fn build_index(&self) -> Result<ReleaseIndex, Self::Error> {
        let releases = self
            .documents
            .iter()
            .map(|document| {
                document
                    .load()
                    .map_err(ChannelManifestsError::RustReleasesIoError)
                    .and_then(|content| parse_release_manifest(&content).map(Release::new_stable))
            })
            .collect::<ChannelManifestsResult<BTreeSet<_>>>()?;

        Ok(ReleaseIndex::from_iter(releases))
    }
}

#[allow(deprecated)]
impl FetchResources for ChannelManifests {
    type Error = ChannelManifestsError;

    fn fetch_channel(channel: Channel) -> Result<Self, Self::Error> {
        let source = fetch_meta_manifest()?;
        let content = source.load()?;
        let content =
            String::from_utf8(content).map_err(|_| ChannelManifestsError::ParseMetaManifest)?;

        let meta_manifest = MetaManifest::try_from_str(&content)?;

        let release_manifests = fetch_release_manifests(&meta_manifest, channel)?;

        Ok(Self {
            documents: release_manifests,
        })
    }
}
