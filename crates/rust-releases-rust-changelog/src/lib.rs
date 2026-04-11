#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(unsafe_code)]
//! Please, see the [`rust-releases`] for additional documentation on how this crate can be used.
//!
//! [`rust-releases`]: https://docs.rs/rust-releases
#[cfg(test)]
extern crate rust_releases_io;
use rust_release::toolchain::RustVersion;
use rust_releases_core::channel::Channel;
use rust_releases_core::releases::StableReleases;
use rust_releases_core::{rust_release, RustRelease, Stable};
use rust_releases_io::Document;

pub(crate) mod errors;
pub(crate) mod fetch;

use crate::fetch::fetch;

pub use errors::{RustChangelogError, RustChangelogResult};
use std::str::FromStr;
use time::macros::format_description;

/// A source which obtains release data from the official Rust changelog.
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

    /// Build an index of all known stable releases from the official Rust changelog.
    pub fn build_index(&self) -> Result<StableReleases, RustChangelogError> {
        let buffer = self.source.buffer();
        let content = std::str::from_utf8(buffer).map_err(RustChangelogError::UnrecognizedText)?;

        let mut releases = StableReleases::default();
        for line in content.lines().filter(|s| s.starts_with("Version")) {
            match create_release(line, &self.today) {
                Some(Ok(release)) => releases.add(release),
                Some(Err(e)) => return Err(e),
                None => {}
            }
        }

        Ok(releases)
    }

    /// Fetch all known releases from the official rust changelog
    pub fn fetch_channel(channel: Channel) -> Result<Self, RustChangelogError> {
        if let Channel::Stable = channel {
            // todo: add support for custom cache locations
            let document = fetch(None::<&str>)?;
            Ok(Self::from_document(document))
        } else {
            Err(RustChangelogError::ChannelNotAvailable(channel))
        }
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
fn create_release(
    line: &str,
    today: &ReleaseDate,
) -> Option<RustChangelogResult<RustRelease<Stable>>> {
    let parsed = parse_release(line.split_ascii_whitespace());

    match parsed {
        // If the version and date can be parsed, and the version has been released
        Ok((stable, date)) if date.is_available(today) => {
            let release_date = rust_release::date::Date::new(
                date.0.year() as u16,
                date.0.month() as u8,
                date.0.day(),
            );
            Some(Ok(RustRelease::new(stable, Some(release_date), [])))
        }
        // If the version and date can be parsed, but the version is not yet released
        Ok(_) => None,
        // VersionParseError covers pre-release versions (1.0.0-alpha, 1.0.0-beta.1) and
        // two-component versions (0.10, 0.9, etc.)
        Err(RustChangelogError::VersionParseError(_)) => None,
        // In any ony other error case, we forward the error
        Err(err) => Some(Err(err)),
    }
}

fn parse_release<'line>(
    mut parts: impl Iterator<Item = &'line str>,
) -> Result<(Stable, ReleaseDate), RustChangelogError> {
    let version_number = parts
        .nth(1)
        .ok_or(RustChangelogError::NoVersionInChangelogItem)?;
    let release_date = parts
        .next()
        .ok_or(RustChangelogError::NoDateInChangelogItem)?;

    let stable = version_number
        .parse::<RustVersion>()
        .map(Stable::from)
        .map_err(|_| RustChangelogError::VersionParseError(version_number.to_string()))?;

    let date = ReleaseDate::parse(&release_date[1..release_date.len() - 1])?;

    Ok((stable, date))
}

#[derive(Debug)]
struct ReleaseDate(time::Date);

impl ReleaseDate {
    fn today() -> Self {
        let date = time::OffsetDateTime::now_utc().date();

        Self(date)
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

        let result = time::Date::parse(item.trim(), &format)
            .map_err(|err| RustChangelogError::TimeParseError(item.to_string(), err))?;

        Ok(Self(result))
    }
}

#[cfg(test)]
mod tests {
    use super::ReleaseDate;
    use crate::RustChangelog;
    use rust_releases_core::channel::Channel;
    use rust_releases_core::Stable;
    use rust_releases_io::Document;
    use std::fs;
    use time::macros::date;
    use yare::parameterized;

    #[test]
    fn source_dist_index() {
        let path = [
            env!("CARGO_MANIFEST_DIR"),
            "/../../resources/rust_changelog/RELEASES.md",
        ]
        .join("");

        let buffer = fs::read(path).unwrap();
        let document = Document::new(buffer);

        let source = RustChangelog::from_document(document);
        let releases = source.build_index().unwrap();

        assert_eq!(releases.len(), 72);
        assert_eq!(
            releases.iter().last().unwrap().version,
            Stable::new(1, 50, 0)
        );
    }

    #[test]
    fn parse_date() {
        let date = ReleaseDate::parse("2021-09-01").unwrap();
        let expected = date!(2021 - 09 - 01);
        assert_eq!(date.0, expected);
    }

    #[test]
    fn with_unreleased_version() {
        let path = [
            env!("CARGO_MANIFEST_DIR"),
            "/../../resources/rust_changelog/RELEASES_with_unreleased.md",
        ]
        .join("");
        let buffer = fs::read(path).unwrap();
        let document = Document::new(buffer);

        let date = ReleaseDate::parse("2021-09-01").unwrap();
        let strategy = RustChangelog::from_document_with_date(document, date);
        let index = strategy.build_index().unwrap();

        assert_eq!(
            index.iter().last().unwrap().version(),
            &Stable::new(1, 54, 0)
        );
    }

    #[parameterized(
        beta = { Channel::Beta },
        nightly = { Channel::Nightly },
    )]
    fn fetch_unsupported_channel(channel: Channel) {
        let file = RustChangelog::fetch_channel(channel);
        assert!(file.is_err());
    }

    #[test]
    fn fetch_supported_channel() {
        let file = RustChangelog::fetch_channel(Channel::Stable);
        assert!(file.is_ok());
    }
}
