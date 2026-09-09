use crate::date;
use crate::releases::ReleaseCollection;
use crate::version;
use rust_releases_core::{Beta, Nightly, RustRelease, Stable};
use std::fmt::Debug;

pub fn encode<R: ReleaseCollection>(releases: &R) -> Vec<u8>
where
    R::Version: Version,
{
    let mut buffer = String::new();

    for release in releases.releases() {
        buffer.push_str(&release.version.encode());

        if let Some(date) = release.release_date() {
            buffer.push(' ');
            buffer.push_str(&date.ymd().to_string());
        }

        buffer.push('\n');
    }

    buffer.into_bytes()
}

pub fn decode<R: ReleaseCollection>(buffer: &[u8]) -> Result<R, CacheEntryError>
where
    R::Version: Version,
{
    let content = std::str::from_utf8(buffer).map_err(CacheEntryError::UnrecognizedText)?;

    let mut releases = R::empty();

    for (offset, line) in content.lines().enumerate() {
        if line.is_empty() {
            continue;
        }

        let release = decode_release(line).ok_or_else(|| CacheEntryError::Entry {
            line: offset + 1,
            entry: line.to_string(),
        })?;

        releases.add(release);
    }

    Ok(releases)
}

fn decode_release<V: Version + Debug>(line: &str) -> Option<RustRelease<V>> {
    let (version, release_date) = match line.split_once(' ') {
        Some((version, date)) => (version, Some(date::parse(date)?)),
        None => (line, None),
    };

    Some(RustRelease::new(V::decode(version)?, release_date, []))
}

pub trait Version: Sized {
    fn encode(&self) -> String;

    fn decode(value: &str) -> Option<Self>;
}

impl Version for Stable {
    fn encode(&self) -> String {
        self.version.to_string()
    }

    fn decode(value: &str) -> Option<Self> {
        version::rust_version(value).map(Stable::from)
    }
}

impl Version for Beta {
    fn encode(&self) -> String {
        version::beta_name(self)
    }

    fn decode(value: &str) -> Option<Self> {
        version::beta(value)
    }
}

impl Version for Nightly {
    fn encode(&self) -> String {
        self.date.ymd().to_string()
    }

    fn decode(value: &str) -> Option<Self> {
        date::parse(value).map(|date| Nightly { date })
    }
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CacheEntryError {
    #[error(transparent)]
    UnrecognizedText(#[from] std::str::Utf8Error),

    #[error("Line {line} of the cached releases could not be parsed: '{entry}'")]
    Entry { line: usize, entry: String },
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_releases_core::rust_release::date::Date;
    use rust_releases_core::{BetaReleases, NightlyReleases, StableReleases};

    #[test]
    fn encode_a_release_without_a_release_date() {
        let releases = StableReleases::new([RustRelease::new(Stable::new(1, 53, 0), None, [])]);

        assert_eq!(encode(&releases), b"1.53.0\n");
    }

    #[test]
    fn encode_a_release_with_a_release_date() {
        let releases = BetaReleases::new([RustRelease::new(
            Beta::new(1, 75, 0, Some(1)),
            Some(Date::new(2023, 11, 13)),
            [],
        )]);

        assert_eq!(encode(&releases), b"1.75.0-beta.1 2023-11-13\n");
    }

    #[test]
    fn decode_the_stable_releases_which_were_encoded() {
        let releases = StableReleases::new([
            RustRelease::new(Stable::new(1, 0, 0), None, []),
            RustRelease::new(Stable::new(1, 53, 0), Some(Date::new(2021, 6, 17)), []),
        ]);

        let decoded = decode::<StableReleases>(&encode(&releases)).unwrap();

        assert_eq!(decoded, releases);
        assert_eq!(
            decoded.iter().last().unwrap().release_date(),
            Some(&Date::new(2021, 6, 17))
        );
    }

    #[test]
    fn decode_the_beta_releases_which_were_encoded() {
        let releases = BetaReleases::new([
            RustRelease::new(Beta::new(1, 75, 0, None), None, []),
            RustRelease::new(Beta::new(1, 75, 0, Some(1)), None, []),
        ]);

        assert_eq!(
            decode::<BetaReleases>(&encode(&releases)).unwrap(),
            releases
        );
    }

    #[test]
    fn decode_the_nightly_releases_which_were_encoded() {
        let releases = NightlyReleases::new([RustRelease::new(
            Nightly::new(2016, 3, 8),
            Some(Date::new(2016, 3, 8)),
            [],
        )]);

        let decoded = decode::<NightlyReleases>(&encode(&releases)).unwrap();

        assert_eq!(decoded, releases);
        assert_eq!(
            decoded.iter().next().unwrap().release_date(),
            Some(&Date::new(2016, 3, 8))
        );
    }

    #[test]
    fn decode_an_empty_entry() {
        assert!(decode::<StableReleases>(b"").unwrap().is_empty());
    }

    #[test]
    fn report_an_entry_which_is_not_utf8() {
        let error = decode::<StableReleases>(&[0xff, 0xfe]).unwrap_err();

        assert!(matches!(error, CacheEntryError::UnrecognizedText(_)));
    }

    #[yare::parameterized(
        not_a_version = { "nightly\n" },
        unparsable_date = { "1.53.0 yesterday\n" },
        beta_version = { "1.75.0-beta.1\n" },
    )]
    fn report_an_unrecognized_entry(entry: &str) {
        let error = decode::<StableReleases>(entry.as_bytes()).unwrap_err();

        assert!(matches!(error, CacheEntryError::Entry { line: 1, .. }));
    }
}
