use crate::aws::keys::{self, ChannelName};
use crate::date;
use rust_releases_core::Nightly;
use std::collections::BTreeSet;

const PATH_PREFIX: &str = "static.rust-lang.org/dist/";

const CHANNEL_PREFIX: &str = "channel-rust-";

const MANIFEST_SUFFIX: &str = ".toml";

// The listing of dated release manifests which the Rust project publishes as a single object. It
// names the dates which published a nightly, from 2016-03-08 onwards.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct NightlyManifestIndex {
    nightly: BTreeSet<Nightly>,
}

impl NightlyManifestIndex {
    pub fn parse(buffer: &[u8]) -> Result<Self, ManifestIndexError> {
        let content = std::str::from_utf8(buffer).map_err(ManifestIndexError::UnrecognizedText)?;

        let mut index = Self::default();

        for (offset, line) in content.lines().enumerate() {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            let entry = parse_entry(line).ok_or_else(|| ManifestIndexError::Entry {
                line: offset + 1,
                entry: line.to_string(),
            })?;

            if let (ChannelName::Nightly, date) = entry {
                index.nightly.insert(Nightly { date });
            }
        }

        Ok(index)
    }

    pub fn nightly(&self) -> impl Iterator<Item = &Nightly> {
        self.nightly.iter()
    }
}

fn parse_entry(line: &str) -> Option<(ChannelName, rust_releases_core::rust_release::date::Date)> {
    let rest = line.strip_prefix(PATH_PREFIX)?;
    let (date, file) = rest.split_once('/')?;
    let date = date::parse(date)?;

    let name = file
        .strip_prefix(CHANNEL_PREFIX)?
        .strip_suffix(MANIFEST_SUFFIX)?;

    Some((keys::channel_name(name)?, date))
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ManifestIndexError {
    #[error(transparent)]
    UnrecognizedText(#[from] std::str::Utf8Error),

    #[error("Line {line} of the manifest index could not be parsed: '{entry}'")]
    Entry { line: usize, entry: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn index(lines: &[&str]) -> NightlyManifestIndex {
        NightlyManifestIndex::parse(lines.join("\n").as_bytes()).unwrap()
    }

    #[test]
    fn collect_a_nightly_release() {
        let index = index(&["static.rust-lang.org/dist/2016-03-08/channel-rust-nightly.toml"]);

        assert_eq!(
            index.nightly().collect::<Vec<_>>(),
            vec![&Nightly::new(2016, 3, 8)]
        );
    }

    #[yare::parameterized(
        stable = { "static.rust-lang.org/dist/2016-04-14/channel-rust-1.8.0.toml" },
        beta = { "static.rust-lang.org/dist/2023-11-13/channel-rust-1.75.0-beta.1.toml" },
        rolling_beta = { "static.rust-lang.org/dist/2023-11-13/channel-rust-beta.toml" },
        rolling_stable = { "static.rust-lang.org/dist/2020-11-19/channel-rust-stable.toml" },
        two_component_stable = { "static.rust-lang.org/dist/2020-11-19/channel-rust-1.48.toml" },
    )]
    fn skip_a_manifest_which_is_not_a_nightly_release(line: &str) {
        assert!(index(&[line]).nightly().next().is_none());
    }

    #[test]
    fn a_date_which_is_listed_more_than_once_is_collected_once() {
        let index = index(&[
            "static.rust-lang.org/dist/2016-03-08/channel-rust-nightly.toml",
            "static.rust-lang.org/dist/2016-03-08/channel-rust-nightly.toml",
        ]);

        assert_eq!(index.nightly().count(), 1);
    }

    #[yare::parameterized(
        unknown_host = { "example.com/dist/2016-03-08/channel-rust-nightly.toml" },
        no_date = { "static.rust-lang.org/dist/channel-rust-nightly.toml" },
        no_channel_prefix = { "static.rust-lang.org/dist/2016-03-08/rust-nightly.toml" },
        no_toml_suffix = { "static.rust-lang.org/dist/2016-03-08/channel-rust-nightly" },
        unknown_channel = { "static.rust-lang.org/dist/2016-03-08/channel-rust-gamma.toml" },
    )]
    fn report_an_unrecognized_entry(line: &str) {
        let error = NightlyManifestIndex::parse(line.as_bytes()).unwrap_err();

        assert!(matches!(error, ManifestIndexError::Entry { line: 1, .. }));
    }

    #[test]
    fn report_an_index_which_is_not_utf8() {
        let error = NightlyManifestIndex::parse(&[0xff, 0xfe]).unwrap_err();

        assert!(matches!(error, ManifestIndexError::UnrecognizedText(_)));
    }
}
