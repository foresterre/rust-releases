#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(unsafe_code)]
#![allow(clippy::upper_case_acronyms)]
//! Please, see the [`rust-releases`] for additional documentation on how this crate can be used.
//!
//! [`rust-releases`]: https://docs.rs/rust-releases

use regex::{Captures, Regex};
use rust_releases_core::channel::Channel;
use rust_releases_core::releases::StableReleases;
use rust_releases_core::{RustRelease, Stable};
use rust_releases_io::Document;

pub(crate) mod errors;
pub(crate) mod fetch;

pub use crate::errors::{RustDistError, RustDistResult};

/// A source which obtains its input data from the Rust distribution bucket on AWS S3.
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

impl RustDist {
    /// Build an index of all known stable releases from the Rust distribution bucket.
    pub fn build_index(&self) -> Result<StableReleases, RustDistError> {
        let buffer = self.source.buffer();
        let content = std::str::from_utf8(buffer).map_err(RustDistError::UnrecognizedText)?;

        let mut releases = StableReleases::default();
        for capture in MATCHER.captures_iter(content) {
            releases.add(parse_release(capture)?);
        }

        Ok(releases)
    }
}

fn parse_release(capture: Captures) -> RustDistResult<RustRelease<Stable>> {
    const MAJOR: &str = "major";
    const MINOR: &str = "minor";
    const PATCH: &str = "patch";

    let major = capture[MAJOR].parse::<u64>().map_err(|_| {
        RustDistError::UnableToParseVersionNumberComponent(&MAJOR, capture[MAJOR].to_string())
    })?;
    let minor = capture[MINOR].parse::<u64>().map_err(|_| {
        RustDistError::UnableToParseVersionNumberComponent(&MINOR, capture[MINOR].to_string())
    })?;
    let patch = capture[PATCH].parse::<u64>().map_err(|_| {
        RustDistError::UnableToParseVersionNumberComponent(&PATCH, capture[PATCH].to_string())
    })?;

    let stable = Stable::new(major, minor, patch);

    Ok(RustRelease::new(stable, None, []))
}

impl RustDist {
    /// Fetch all known releases from the rust S3 distribution bucket
    pub fn fetch_channel(channel: Channel) -> Result<Self, RustDistError> {
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
    use rust_releases_core::Stable;
    use rust_releases_io::Document;
    use std::fs;

    #[test]
    fn source_rust_dist() {
        let path = [
            env!("CARGO_MANIFEST_DIR"),
            "/../../resources/rust_dist/dist_static-rust-lang-org.txt",
        ]
        .join("");

        let buffer = fs::read(path).unwrap();
        let document = Document::new(buffer);

        let source = RustDist::from_document(document);
        let releases = source.build_index().unwrap();

        // 74 releases including minor releases from 1.0.0 to 1.53.0
        assert_eq!(releases.len(), 74);
        assert_eq!(
            releases.iter().last().unwrap().version,
            Stable::new(1, 53, 0)
        );
        assert_eq!(
            releases.iter().next().unwrap().version,
            Stable::new(1, 0, 0)
        );
    }
}
