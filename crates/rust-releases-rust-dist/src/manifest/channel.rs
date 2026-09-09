use crate::manifest::ReleaseManifestError;
use crate::version;
use rust_releases_core::rust_release::date::Date;
use rust_releases_core::rust_release::toolchain::Channel;
use rust_releases_core::{Nightly, Stable};

const NIGHTLY_PRERELEASE: &str = "nightly";

// A release manifest states the release it describes as the version of its 'rust' package, which
// reads as '1.75.0 (82e1608df 2023-12-21)'. The prerelease of that version, if any, names the
// channel it belongs to.
pub fn parse(version: &str, date: &Date) -> Result<Channel, ReleaseManifestError> {
    let unrecognized = || ReleaseManifestError::Version {
        version: version.to_string(),
    };

    let number = version
        .split_ascii_whitespace()
        .next()
        .ok_or_else(unrecognized)?;

    match number.split_once('-') {
        None => Ok(Channel::Stable(Stable::from(
            version::rust_version(number).ok_or_else(unrecognized)?,
        ))),
        // A nightly manifest states a version which is not released as such, like '1.99.0-nightly',
        // so the date the manifest was published on is what versions the release.
        Some((number, NIGHTLY_PRERELEASE)) => {
            version::rust_version(number).ok_or_else(unrecognized)?;

            Ok(Channel::Nightly(Nightly { date: date.clone() }))
        }
        Some(_) => Ok(Channel::Beta(
            version::beta(number).ok_or_else(unrecognized)?,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_releases_core::Beta;

    fn channel(version: &str) -> Channel {
        parse(version, &Date::new(2016, 3, 8)).unwrap()
    }

    #[test]
    fn the_channel_of_a_stable_version() {
        assert_eq!(
            channel("1.53.0 (53cb7b09b 2021-06-17)"),
            Channel::Stable(Stable::new(1, 53, 0))
        );
    }

    #[test]
    fn the_channel_of_a_beta_version() {
        assert_eq!(
            channel("1.8.0-beta.2 (2879d940a 2016-03-22)"),
            Channel::Beta(Beta::new(1, 8, 0, Some(2)))
        );
        assert_eq!(
            channel("1.8.0-beta (2879d940a 2016-03-22)"),
            Channel::Beta(Beta::new(1, 8, 0, None))
        );
    }

    #[test]
    fn the_channel_of_a_nightly_version_is_dated_by_its_manifest() {
        assert_eq!(
            channel("1.9.0-nightly (388ccda45 2016-03-07)"),
            Channel::Nightly(Nightly::new(2016, 3, 8))
        );
    }

    #[yare::parameterized(
        two_components = { "1.53 (53cb7b09b 2021-06-17)" },
        not_a_number = { "1.x.0" },
        unknown_prerelease = { "1.53.0-gamma" },
        prerelease_which_is_not_a_number = { "1.53.0-beta.x" },
    )]
    fn report_a_version_which_cannot_be_parsed(version: &str) {
        let error = parse(version, &Date::new(2016, 3, 8)).unwrap_err();

        assert!(matches!(
            error,
            ReleaseManifestError::Version { version: reported } if reported == version
        ));
    }
}
