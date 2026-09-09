use regex::{Captures, Regex};
use rust_releases_core::{RustRelease, Stable, StableReleases};

lazy_static::lazy_static! {
    static ref MATCHER: Regex =
        Regex::new(r"(?m)^dist/rustc-(?P<major>\d+).(?P<minor>\d+).(?P<patch>\d+)(?:\-(alpha|beta|nightly)(\.\d+))?").unwrap();
}

pub fn stable_releases(buffer: &[u8]) -> Result<StableReleases, DistIndexError> {
    let content = std::str::from_utf8(buffer).map_err(DistIndexError::UnrecognizedText)?;

    let mut releases = StableReleases::default();
    for capture in MATCHER.captures_iter(content) {
        releases.add(parse_release(capture)?);
    }

    Ok(releases)
}

fn parse_release(capture: Captures) -> Result<RustRelease<Stable>, DistIndexError> {
    const MAJOR: &str = "major";
    const MINOR: &str = "minor";
    const PATCH: &str = "patch";

    let major = capture[MAJOR].parse::<u64>().map_err(|_| {
        DistIndexError::UnableToParseVersionNumberComponent(&MAJOR, capture[MAJOR].to_string())
    })?;
    let minor = capture[MINOR].parse::<u64>().map_err(|_| {
        DistIndexError::UnableToParseVersionNumberComponent(&MINOR, capture[MINOR].to_string())
    })?;
    let patch = capture[PATCH].parse::<u64>().map_err(|_| {
        DistIndexError::UnableToParseVersionNumberComponent(&PATCH, capture[PATCH].to_string())
    })?;

    let stable = Stable::new(major, minor, patch);

    Ok(RustRelease::new(stable, None, []))
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum DistIndexError {
    /// Returned in case the input text cannot be parsed.
    #[error(transparent)]
    UnrecognizedText(#[from] std::str::Utf8Error),

    /// Returned in case a component of a `semver` version could not be parsed as a number.
    ///
    /// The component is usually the `major`, `minor` or `patch` version.
    #[error("The '{0}' component of the version number could not be parsed. The input was: '{1}'")]
    UnableToParseVersionNumberComponent(&'static &'static str, String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_an_object_key() {
        let releases = stable_releases(b"dist/rustc-1.53.0-x86_64-apple-darwin.tar.gz\n").unwrap();
        let release = releases.iter().next().unwrap();

        assert_eq!(release.version(), &Stable::new(1, 53, 0));
        assert_eq!(release.release_date(), None);
        assert!(release.toolchains().is_empty());
    }

    #[test]
    fn parse_every_artifact_of_a_release_into_a_single_release() {
        let index = b"dist/rustc-1.53.0-x86_64-apple-darwin.tar.gz\n\
                      dist/rustc-1.53.0-x86_64-apple-darwin.tar.gz.asc\n\
                      dist/rustc-1.53.0-x86_64-unknown-linux-gnu.tar.xz\n";

        let releases = stable_releases(index).unwrap();

        assert_eq!(releases.len(), 1);
    }

    #[test]
    fn skip_object_keys_which_are_not_a_rustc_artifact() {
        let index = b"dist/rust-1.53.0-x86_64-apple-darwin.tar.gz\n\
                      dist/2021-06-17/rustc-1.53.0-x86_64-apple-darwin.tar.gz\n";

        assert!(stable_releases(index).unwrap().is_empty());
    }

    #[test]
    fn report_an_index_which_is_not_utf8() {
        let error = stable_releases(&[0xff, 0xfe]).unwrap_err();

        assert!(matches!(error, DistIndexError::UnrecognizedText(_)));
    }
}
