#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(unsafe_code)]
//! Please, see the [`rust-releases`] for additional documentation on how this crate can be used.
//!
//! [`rust-releases`]: https://docs.rs/rust-releases

use rust_releases_core::{RustRelease, RustReleases, Stable};
use rust_releases_io::{FsClient, HttpCachedClient, HttpClient};
use std::collections::HashSet;
use std::convert::TryFrom;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Duration;
use time::macros::format_description;

#[cfg(test)]
#[macro_use]
extern crate rust_releases_io;

mod errors;
mod fetch;

pub use errors::{RustChangelogError, RustChangelogResult};

const URL: &str = "https://raw.githubusercontent.com/rust-lang/rust/master/RELEASES.md";

#[derive(Debug)]
pub struct RemoteClient {
    client: ClientImpl,
}

impl RemoteClient {
    /// A client where files are fetched from a remote server over http.
    pub fn http_client() -> Self {
        Self {
            client: ClientImpl::Http(HttpClient::default()),
        }
    }

    /// A client where files are fetched from a remote server over http,
    /// or from the client if they're present and not expired.
    pub fn cached_http_client(folder: PathBuf, expiry: Duration) -> Self {
        Self {
            client: ClientImpl::CachedHttp(HttpCachedClient::new(folder, expiry)),
        }
    }

    pub fn exec(&mut self) -> Result<RustChangelog, ()> {
        todo!()
    }
}

#[derive(Debug)]
enum ClientImpl {
    Http(HttpClient),
    CachedHttp(HttpCachedClient),
}

#[derive(Debug, Default)]
enum RemoteUrl {
    #[default]
    GitHub,
    HttpUrl(String),
}

impl RemoteUrl {
    pub fn url(&self) -> &str {
        match self {
            Self::GitHub => URL,
            Self::HttpUrl(url) => &url,
        }
    }
}

pub struct RustChangelog {
    releases: RustReleases,
    /// Filters can be used to limit the returned results, for example to compare
    /// against the date of an unreleased version which does already exist in the
    /// changelog. If this date is at least as late as the time found in a
    /// release registration, we will say that such a version is released (i.e. published).
    filters: HashSet<Filter>,
}

impl RustChangelog {
    pub fn new(today: ReleaseDate) -> Self {
        let mut default_filters = HashSet::new();
        default_filters.insert(Filter::Date(today));

        Self {
            releases: RustReleases::default(),
            filters: default_filters,
        }
    }

    pub fn stable_releases(&self) -> impl IntoIterator<Item = &RustRelease<Stable>> {
        self.releases.stable().into_iter().filter(move |release| {
            release.release_date().is_some_and(|dt| {
                // Check if the release date passes all date filters
                self.filters.iter().all(|filter| match filter {
                    Filter::Date(before_dt) => {
                        dt.year() <= before_dt.0.year() as u16
                            && dt.month() <= before_dt.0.month().into()
                            && dt.day() <= before_dt.0.day()
                    }
                    _ => true,
                })
            })
        })
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

#[derive(Debug, Eq, Hash, PartialOrd, PartialEq)]
#[non_exhaustive]
pub enum Filter {
    Date(ReleaseDate),
}

impl RustChangelog {
    /// Fetch all known releases from the official rust changelog
    pub fn fetch_channel(channel: Channel) -> Result<Self, RustChangelogError> {
        if let Channel::Stable = channel {
            let document = fetch(None::<&Path>)?;
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
        .map_err(|_| RustChangelogError::VersionParseError(version_number.to_string()))?;

    let date = ReleaseDate::parse(&release_date[1..release_date.len() - 1])?;

    Ok((version, date))
}

#[derive(Debug, Eq, PartialOrd, PartialEq, Hash)]
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
    use rust_releases_core::{semver, Channel, Release, ReleaseIndex};
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

        let strategy = RustChangelog::from_document(document);
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
