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
use std::str::FromStr;
use time::macros::format_description;

/// A [`Source`] which obtains release data from the official Rust changelog.
///
/// [`Source`]: rust_releases_core::Source
pub struct RustChangelog {
    source: Document,

    /// Used to compare against the date of an unreleased version which does already exist in the
    /// changelog. If this date is at least as late as the time found in a release registration, we
    /// will say that such a version is released (i.e. published).
    today: ReleaseDate,
}

impl RustChangelog {
    pub(crate) fn from_document(source: Document) -> Self {
        Self {
            source,
            today: ReleaseDate::today(),
        }
    }

    #[cfg(test)]
    pub(crate) fn from_document_with_date(source: Document, date: ReleaseDate) -> Self {
        Self {
            source,
            today: date,
        }
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
            .filter_map(|line| create_release(line, &self.today))
            .collect::<Result<ReleaseIndex, Self::Error>>()?;

        Ok(releases)
    }
}

/// Create a release from a `Version ...` header in the Rust changelog file (`RELEASES.md`).
///
/// We skip a few older versions which did not use full 3-component semver versions.
/// While we could parse them as `SemverReq` requirements, adding those would not be worth the hassle
///   (at least for now).
///
/// Versions which we should be able to parse, and are based on their release date available, are
///   returned as `Some(Result<Release, Error>)`.
/// If a version is not yet available based on their release date we return `None`.
/// Versions we currently do not support are also returned as `None`.
///
/// The resulting releases can then be filtered on `Option::is_some`, to only keep relevant results.
fn create_release(line: &str, today: &ReleaseDate) -> Option<RustChangelogResult<Release>> {
    let parsed = parse_release(line.split_ascii_whitespace());

    match parsed {
        // If the version and date can be parsed, and the version has been released
        Ok((version, date)) if date.is_available(today) && version.pre.is_empty() => {
            Some(Ok(Release::new_stable(version)))
        }
        // If the version and date can be parsed, but the version is not yet released
        Ok(_) => None,
        // We skip versions 0.10, 0.9, etc. which require more lenient semver parsing
        // Unfortunately we can't access the error kind, so we have to match the string instead
        Err(RustChangelogError::SemverError(err, _))
            if err.to_string().as_str()
                == "unexpected end of input while parsing minor version number" =>
        {
            None
        }
        // In any ony other error case, we forward the error
        Err(err) => Some(Err(err)),
    }
}

impl FetchResources for RustChangelog {
    type Error = RustChangelogError;

    fn fetch_channel(channel: Channel) -> Result<Self, Self::Error> {
        if let Channel::Stable = channel {
            let document = fetch()?;
            Ok(Self::from_document(document))
        } else {
            Err(RustChangelogError::ChannelNotAvailable(channel))
        }
    }
}

fn parse_release<'line>(
    mut parts: impl Iterator<Item = &'line str>,
) -> Result<(semver::Version, ReleaseDate), RustChangelogError> {
    let version_number = parts
        .nth(1)
        .ok_or(RustChangelogError::NoVersionInChangelogItem)?;
    let release_date = parts
        .next()
        .ok_or(RustChangelogError::NoDateInChangelogItem)?;

    let version = semver::Version::parse(version_number)
        .map_err(|err| RustChangelogError::SemverError(err, version_number.to_string()))?;

    let date = ReleaseDate::parse(&release_date[1..release_date.len() - 1])?;

    Ok((version, date))
}

#[derive(Debug)]
struct ReleaseDate(time::OffsetDateTime);

impl ReleaseDate {
    fn today() -> Self {
        Self(time::OffsetDateTime::now_utc())
    }

    fn parse(from: &str) -> Result<Self, RustChangelogError> {
        from.parse::<ReleaseDate>()
    }

    fn is_available(&self, today: &Self) -> bool {
        today.0 >= self.0
    }
}

impl FromStr for ReleaseDate {
    type Err = crate::RustChangelogError;

    fn from_str(item: &str) -> Result<Self, Self::Err> {
        let format = format_description!("[year]-[month]-[day]");

        let result = time::OffsetDateTime::parse(item, &format)
            .map_err(|_| RustChangelogError::TimeParseError(item.to_string()))?;

        Ok(Self(result))
    }
}

#[cfg(test)]
mod tests {
    use super::ReleaseDate;
    use crate::RustChangelog;
    use rust_releases_core::{semver, Channel, FetchResources, Release, ReleaseIndex};
    use rust_releases_io::Document;
    use time::macros::date;
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

    #[test]
    fn parse_date() {
        let date = ReleaseDate::parse("2021-09-01").unwrap();
        let expected = date!(2021 - 09 - 01);
        assert_eq!(date.0.date(), expected);
    }

    #[test]
    fn with_unreleased_version() {
        let path = [
            env!("CARGO_MANIFEST_DIR"),
            "/../../resources/rust_changelog/RELEASES_with_unreleased.md",
        ]
        .join("");

        let date = ReleaseDate::parse("2021-09-01").unwrap();
        let strategy =
            RustChangelog::from_document_with_date(Document::LocalPath(path.into()), date);
        let index = ReleaseIndex::from_source(strategy).unwrap();

        let mut releases = index.releases().iter();

        assert_eq!(
            releases.next().unwrap().version(),
            &semver::Version::new(1, 54, 0)
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
