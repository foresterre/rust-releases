#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(unsafe_code)]
#![allow(clippy::upper_case_acronyms)]
//! Please, see the [`rust-releases`] for additional documentation on how this crate can be used.
//!
//! [`rust-releases`]: https://docs.rs/rust-releases

#[cfg(test)]
#[macro_use]
extern crate rust_releases_io;

use regex::{Captures, Regex};
use rust_releases_core::{semver, Channel, FetchResources, Release, ReleaseIndex, Source};
use rust_releases_io::Document;
use std::collections::BTreeSet;
use std::iter::FromIterator;

pub(crate) mod errors;
pub(crate) mod fetch;

pub use crate::errors::{RustDistError, RustDistResult};

/// A [`Source`] which obtains its input data from the Rust distribution bucket on AWS S3.
///
/// [`Source`]: rust_releases_core::Source
pub struct RustDist {
    source: Document,
}

impl RustDist {
    #[cfg(test)]
    pub(crate) fn from_document(source: Document) -> Self {
        Self { source }
    }
}

lazy_static::lazy_static! {
    static ref MATCHER: Regex =
        Regex::new(r"(?m)^dist/rustc-(?P<major>\d+).(?P<minor>\d+).(?P<patch>\d+)(?:\-(alpha|beta|nightly)(\.\d+))?").unwrap();
}

impl Source for RustDist {
    type Error = RustDistError;

    fn build_index(&self) -> Result<ReleaseIndex, Self::Error> {
        let contents = self.source.load()?;
        let content = String::from_utf8(contents).map_err(RustDistError::UnrecognizedText)?;

        let releases = MATCHER
            .captures_iter(&content)
            .map(parse_release)
            .collect::<RustDistResult<BTreeSet<Release>>>()?;

        Ok(ReleaseIndex::from_iter(releases))
    }
}

fn parse_release(capture: Captures) -> RustDistResult<Release> {
    let major = capture["major"]
        .parse::<u64>()
        .map_err(RustDistError::UnableToParseNumber)?;
    let minor = capture["minor"]
        .parse::<u64>()
        .map_err(RustDistError::UnableToParseNumber)?;
    let patch = capture["patch"]
        .parse::<u64>()
        .map_err(RustDistError::UnableToParseNumber)?;

    Ok(Release::new_stable(semver::Version::new(
        major, minor, patch,
    )))
}

impl FetchResources for RustDist {
    type Error = RustDistError;

    fn fetch_channel(channel: Channel) -> Result<Self, Self::Error> {
        if let Channel::Stable = channel {
            let source = fetch::fetch()?;
            Ok(Self { source })
        } else {
            Err(RustDistError::ChannelNotAvailable(channel))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::RustDist;
    use rust_releases_core::{semver, Release, ReleaseIndex};
    use rust_releases_io::Document;

    #[test]
    fn source_rust_dist() {
        let path = [
            env!("CARGO_MANIFEST_DIR"),
            "/../..//resources/rust_dist/dist_static-rust-lang-org.txt",
        ]
        .join("");
        let strategy = RustDist::from_document(Document::LocalPath(path.into()));
        let index = ReleaseIndex::from_source(strategy).unwrap();

        // 71 releases including minor releases from 1.0.0 to 1.51.0
        assert_eq!(index.releases().len(), 71);
        assert_eq!(
            index.releases()[0],
            Release::new_stable(semver::Version::new(1, 51, 0))
        );
        assert_eq!(
            index.releases()[70],
            Release::new_stable(semver::Version::new(1, 0, 0))
        );
    }
}
