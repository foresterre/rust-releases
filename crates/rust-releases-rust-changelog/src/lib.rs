#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(unsafe_code)]
//! Please, see the [`rust-releases`] for additional documentation on how this crate can be used.
//!
//! [`rust-releases`]: https://docs.rs/rust-releases

use rust_releases_core::{semver, Channel, FetchResources, Release, ReleaseIndex, Source};
use rust_releases_io::Document;
#[cfg(test)]
#[macro_use]
extern crate rust_releases_io;

pub(crate) mod errors;
pub(crate) mod fetch;

use crate::fetch::fetch;

pub use errors::{RustChangelogError, RustChangelogResult};

/// A [`Source`] which obtains release data from the official Rust changelog.
///
/// [`Source`]: rust_releases_core::Source
pub struct RustChangelog {
    source: Document,
}

impl RustChangelog {
    #[cfg(test)]
    pub(crate) fn from_document(source: Document) -> Self {
        Self { source }
    }
}

impl Source for RustChangelog {
    type Error = RustChangelogError;

    fn build_index(&self) -> Result<ReleaseIndex, Self::Error> {
        let contents = self.source.load()?;
        let content = String::from_utf8(contents).map_err(RustChangelogError::UnrecognizedText)?;

        let releases = content
            .lines()
            .filter(|s| s.starts_with("Version"))
            .filter_map(|s| {
                s.split_ascii_whitespace()
                    .nth(1)
                    .and_then(|s| semver::Version::parse(s).map(Release::new_stable).ok())
            });

        Ok(releases.collect())
    }
}

impl FetchResources for RustChangelog {
    type Error = RustChangelogError;

    fn fetch_channel(channel: Channel) -> Result<Self, Self::Error> {
        if let Channel::Stable = channel {
            let source = fetch()?;
            Ok(Self { source })
        } else {
            Err(RustChangelogError::ChannelNotAvailable(channel))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::RustChangelog;
    use rust_releases_core::{semver, Channel, FetchResources, Release, ReleaseIndex};
    use rust_releases_io::Document;
    use yare::parameterized;

    #[test]
    fn source_dist_index() {
        let path = [
            env!("CARGO_MANIFEST_DIR"),
            "/../../resources/rust_changelog/RELEASES.md",
        ]
        .join("");
        let strategy = RustChangelog::from_document(Document::LocalPath(path.into()));
        let index = ReleaseIndex::from_source(strategy).unwrap();

        assert!(index.releases().len() > 50);
        assert_eq!(
            index.releases()[0],
            Release::new_stable(semver::Version::new(1, 50, 0))
        );
    }

    #[parameterized(
        beta = { Channel::Beta },
        nightly = { Channel::Nightly },
    )]
    fn fetch_unsupported_channel(channel: Channel) {
        __internal_dl_test!({
            let file = RustChangelog::fetch_channel(channel);
            assert!(file.is_err());
        })
    }

    #[test]
    fn fetch_supported_channel() {
        __internal_dl_test!({
            let file = RustChangelog::fetch_channel(Channel::Stable);
            assert!(file.is_ok());
        })
    }
}
