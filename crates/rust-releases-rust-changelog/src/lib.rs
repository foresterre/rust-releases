#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(unsafe_code)]
//! Please, see the [`rust-releases`] for additional documentation on how this crate can be used.
//!
//! [`rust-releases`]: https://docs.rs/rust-releases

pub use errors::ParseError;
use rust_release::toolchain::RustVersion;
use rust_releases_core::releases::StableReleases;
use rust_releases_core::{rust_release, RustRelease, Stable};
use rust_releases_io::{Document, ResourceFile, RustReleasesClient};
use std::str::FromStr;
use time::macros::format_description;

// Re-export all clients so callers don't need a separate rust-releases-io dependency.
pub use rust_releases_io::{FsClient, HttpCachedClient, HttpClient};
pub use url::Url;

mod errors;
mod url;

const URL: &str = "https://raw.githubusercontent.com/rust-lang/rust/master/RELEASES.md";
const RESOURCE_NAME: &str = "RELEASES.md";

/// Fetches the Rust changelog document using a provided client.
pub struct Client<C> {
    client: C,
}

impl<C: RustReleasesClient> Client<C> {
    /// Create a new client wrapper.
    pub fn new(client: C) -> Self {
        Self { client }
    }

    /// Fetch the changelog document.
    ///
    /// The error type is that of the provided client.
    pub fn fetch(&self) -> Result<RustChangelog, C::Error> {
        let url = Url::default(); // TODO: make it configurable.
        let document = self
            .client
            .fetch(ResourceFile::new(url.as_ref(), RESOURCE_NAME))?
            .into_document();

        Ok(RustChangelog {
            document,
            today: ReleaseDate::today(),
        })
    }
}

/// The fetched, unparsed Rust changelog document.
pub struct RustChangelog {
    document: Document,
    today: ReleaseDate,
}

impl RustChangelog {
    /// Parse the document into a set of stable releases.
    pub fn try_parse(self) -> Result<StableReleases, ParseError> {
        let buffer = self.document.buffer();
        let content = std::str::from_utf8(buffer).map_err(ParseError::UnrecognizedText)?;

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

    #[cfg(test)]
    fn with_date(mut self, date: ReleaseDate) -> Self {
        self.today = date;
        self
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
) -> Option<Result<RustRelease<Stable>, ParseError>> {
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
        Err(ParseError::VersionParseError(_)) => None,
        // In any ony other error case, we forward the error
        Err(err) => Some(Err(err)),
    }
}

fn parse_release<'line>(
    mut parts: impl Iterator<Item = &'line str>,
) -> Result<(Stable, ReleaseDate), ParseError> {
    let version_number = parts.nth(1).ok_or(ParseError::NoVersionInChangelogItem)?;
    let release_date = parts.next().ok_or(ParseError::NoDateInChangelogItem)?;

    let stable = version_number
        .parse::<RustVersion>()
        .map(Stable::from)
        .map_err(|_| ParseError::VersionParseError(version_number.to_string()))?;

    let date = ReleaseDate::try_parse(&release_date[1..release_date.len() - 1])?;

    Ok((stable, date))
}

#[derive(Debug)]
struct ReleaseDate(time::Date);

impl ReleaseDate {
    fn today() -> Self {
        let date = time::OffsetDateTime::now_utc().date();

        Self(date)
    }

    fn try_parse(from: &str) -> Result<Self, ParseError> {
        from.parse::<ReleaseDate>()
    }

    fn is_available(&self, today: &Self) -> bool {
        today.0 >= self.0
    }
}

impl FromStr for ReleaseDate {
    type Err = ParseError;

    fn from_str(item: &str) -> Result<Self, Self::Err> {
        let format = format_description!("[year]-[month]-[day]");

        let result = time::Date::parse(item.trim(), &format)
            .map_err(|err| ParseError::TimeParseError(item.to_string(), err))?;

        Ok(Self(result))
    }
}

#[cfg(test)]
mod tests {
    use super::ReleaseDate;
    use crate::RustChangelog;
    use rust_releases_core::Stable;
    use rust_releases_io::Document;
    use std::fs;
    use time::macros::date;

    fn changelog_from_file(rel_path: &str) -> RustChangelog {
        let path = [env!("CARGO_MANIFEST_DIR"), rel_path].join("");
        let buffer = fs::read(path).unwrap();
        RustChangelog {
            document: Document::new(buffer),
            today: ReleaseDate::today(),
        }
    }

    #[test]
    fn source_dist_index() {
        let releases = changelog_from_file("/../../resources/rust_changelog/RELEASES.md")
            .try_parse()
            .unwrap();

        assert_eq!(releases.len(), 72);
        assert_eq!(
            releases.iter().last().unwrap().version,
            Stable::new(1, 50, 0)
        );
    }

    #[test]
    fn parse_date() {
        let date = ReleaseDate::try_parse("2021-09-01").unwrap();
        let expected = date!(2021 - 09 - 01);
        assert_eq!(date.0, expected);
    }

    #[test]
    fn with_unreleased_version() {
        let date = ReleaseDate::try_parse("2021-09-01").unwrap();
        let releases =
            changelog_from_file("/../../resources/rust_changelog/RELEASES_with_unreleased.md")
                .with_date(date)
                .try_parse()
                .unwrap();

        assert_eq!(
            releases.iter().last().unwrap().version(),
            &Stable::new(1, 54, 0)
        );
    }
}
