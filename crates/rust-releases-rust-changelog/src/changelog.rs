use rust_releases_core::rust_release::toolchain::RustVersion;
use rust_releases_core::{RustRelease, Stable, StableReleases, rust_release};
use std::str::FromStr;
use time::macros::format_description;

pub fn stable_releases(
    buffer: &[u8],
    today: &ReleaseDate,
) -> Result<StableReleases, ChangelogError> {
    let content = std::str::from_utf8(buffer).map_err(ChangelogError::UnrecognizedText)?;

    let mut releases = StableReleases::default();
    for line in content.lines().filter(|s| s.starts_with("Version")) {
        match create_release(line, today) {
            Some(Ok(release)) => releases.add(release),
            Some(Err(e)) => return Err(e),
            None => {}
        }
    }

    Ok(releases)
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
) -> Option<Result<RustRelease<Stable>, ChangelogError>> {
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
        Err(ChangelogError::VersionParseError(_)) => None,
        // In any ony other error case, we forward the error
        Err(err) => Some(Err(err)),
    }
}

fn parse_release<'line>(
    mut parts: impl Iterator<Item = &'line str>,
) -> Result<(Stable, ReleaseDate), ChangelogError> {
    let version_number = parts
        .nth(1)
        .ok_or(ChangelogError::NoVersionInChangelogItem)?;
    let release_date = parts.next().ok_or(ChangelogError::NoDateInChangelogItem)?;

    let stable = version_number
        .parse::<RustVersion>()
        .map(Stable::from)
        .map_err(|_| ChangelogError::VersionParseError(version_number.to_string()))?;

    let date = ReleaseDate::parse(&release_date[1..release_date.len() - 1])?;

    Ok((stable, date))
}

/// Used to compare against the date of an unreleased version which does already exist in the
/// changelog. If this date is at least as late as the time found in a release registration, we
/// will say that such a version is released (i.e. published).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ReleaseDate(time::Date);

impl ReleaseDate {
    pub fn today() -> Self {
        let date = time::OffsetDateTime::now_utc().date();

        Self(date)
    }

    pub fn parse(from: &str) -> Result<Self, ChangelogError> {
        from.parse::<ReleaseDate>()
    }

    fn is_available(&self, today: &Self) -> bool {
        today.0 >= self.0
    }
}

impl FromStr for ReleaseDate {
    type Err = ChangelogError;

    fn from_str(item: &str) -> Result<Self, Self::Err> {
        let format = format_description!("[year]-[month]-[day]");

        let result = time::Date::parse(item.trim(), &format)
            .map_err(|err| ChangelogError::TimeParseError(item.to_string(), err))?;

        Ok(Self(result))
    }
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ChangelogError {
    /// Returned in case of of `time` parse errors
    #[error("Unable to parse release date in a release entry '{0}': {1}")]
    TimeParseError(String, time::error::Parse),

    /// Returned when a version string cannot be parsed as a three-component `major.minor.patch` version
    #[error("Unable to parse version '{0}")]
    VersionParseError(String),

    /// Returned in a case a release entry does not contain a recognizable release date
    #[error("Unable to find a valid release date in a release entry")]
    NoDateInChangelogItem,

    /// Returned in a case a release entry does not contain a recognizable release version
    #[error("Unable to find a valid version in a release entry")]
    NoVersionInChangelogItem,

    /// Returned in case a input resource cannot be parsed as UTF-8
    #[error(transparent)]
    UnrecognizedText(#[from] std::str::Utf8Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::date;

    #[test]
    fn parse_date() {
        let date = ReleaseDate::parse("2021-09-01").unwrap();
        let expected = date!(2021 - 09 - 01);

        assert_eq!(date.0, expected);
    }

    #[test]
    fn parse_a_released_entry() {
        let today = ReleaseDate::parse("2021-09-01").unwrap();

        let releases = stable_releases(b"Version 1.50.0 (2021-02-11)\n", &today).unwrap();
        let release = releases.iter().next().unwrap();

        assert_eq!(release.version(), &Stable::new(1, 50, 0));
        assert_eq!(
            release.release_date(),
            Some(&rust_release::date::Date::new(2021, 2, 11))
        );
    }

    #[test]
    fn skip_an_entry_which_is_not_released_yet() {
        let today = ReleaseDate::parse("2021-09-01").unwrap();

        let releases = stable_releases(b"Version 1.55.0 (2021-09-09)\n", &today).unwrap();

        assert!(releases.is_empty());
    }

    #[yare::parameterized(
        alpha = { b"Version 1.0.0-alpha (2015-01-09)\n" },
        two_components = { b"Version 0.12 (2014-10-09)\n" },
    )]
    fn skip_an_entry_which_is_not_a_stable_version(line: &[u8]) {
        let today = ReleaseDate::today();

        assert!(stable_releases(line, &today).unwrap().is_empty());
    }

    #[test]
    fn skip_a_line_which_is_not_an_entry() {
        let today = ReleaseDate::today();

        let releases = stable_releases(b"============\nUnrelated text\n", &today).unwrap();

        assert!(releases.is_empty());
    }

    #[test]
    fn report_an_entry_without_a_date() {
        let today = ReleaseDate::today();

        let error = stable_releases(b"Version 1.50.0\n", &today).unwrap_err();

        assert!(matches!(error, ChangelogError::NoDateInChangelogItem));
    }

    #[test]
    fn report_an_entry_with_an_unparsable_date() {
        let today = ReleaseDate::today();

        let error = stable_releases(b"Version 1.50.0 (yesterday)\n", &today).unwrap_err();

        assert!(matches!(error, ChangelogError::TimeParseError(item, _) if item == "yesterday"));
    }

    #[test]
    fn report_a_changelog_which_is_not_utf8() {
        let today = ReleaseDate::today();

        let error = stable_releases(&[0xff, 0xfe], &today).unwrap_err();

        assert!(matches!(error, ChangelogError::UnrecognizedText(_)));
    }
}
