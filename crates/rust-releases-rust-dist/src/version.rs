use rust_releases_core::Beta;
use rust_releases_core::rust_release::toolchain::RustVersion;

const BETA_SUFFIX: &str = "-beta";

// A release is versioned by three components. A two component version, such as '1.48', names a
// rolling alias of a release rather than a release of its own.
pub fn rust_version(value: &str) -> Option<RustVersion> {
    if components(value) != 3 {
        return None;
    }

    value.parse::<RustVersion>().ok()
}

pub fn beta(value: &str) -> Option<Beta> {
    let (version, prerelease) = value.split_once(BETA_SUFFIX)?;

    Some(Beta {
        version: rust_version(version)?,
        prerelease: prerelease_number(prerelease)?,
    })
}

pub fn beta_name(version: &Beta) -> String {
    match version.prerelease {
        Some(prerelease) => format!(
            "{version}{BETA_SUFFIX}.{prerelease}",
            version = version.version
        ),
        None => format!("{version}{BETA_SUFFIX}", version = version.version),
    }
}

// The number of a beta prerelease: absent for the bare '-beta' of a release which was published
// once. The outer option reports a suffix which is not a prerelease number at all.
fn prerelease_number(suffix: &str) -> Option<Option<u32>> {
    if suffix.is_empty() {
        return Some(None);
    }

    suffix
        .strip_prefix('.')
        .and_then(|number| number.parse::<u32>().ok())
        .map(Some)
}

pub fn components(value: &str) -> usize {
    let is_number = |part: &str| !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit());

    if value.split('.').all(is_number) {
        value.split('.').count()
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_a_version() {
        assert_eq!(rust_version("1.53.0"), Some(RustVersion::new(1, 53, 0)));
    }

    #[yare::parameterized(
        empty = { "" },
        two_components = { "1.53" },
        four_components = { "1.2.3.4" },
        not_a_number = { "1.x.0" },
        trailing_dot = { "1.53." },
        prerelease = { "1.53.0-beta.1" },
    )]
    fn reject_a_version_which_does_not_name_a_release(value: &str) {
        assert_eq!(rust_version(value), None);
    }

    #[test]
    fn parse_a_beta_version() {
        assert_eq!(beta("1.75.0-beta.1"), Some(Beta::new(1, 75, 0, Some(1))));
        assert_eq!(beta("1.75.0-beta"), Some(Beta::new(1, 75, 0, None)));
    }

    #[yare::parameterized(
        empty = { "" },
        stable = { "1.75.0" },
        two_components = { "1.75-beta" },
        nightly = { "1.75.0-nightly" },
        prerelease_which_is_not_a_number = { "1.75.0-beta.x" },
        prerelease_without_a_separator = { "1.75.0-beta1" },
    )]
    fn reject_a_version_which_is_not_a_beta_release(value: &str) {
        assert_eq!(beta(value), None);
    }

    #[test]
    fn name_a_beta_version() {
        assert_eq!(beta_name(&Beta::new(1, 75, 0, Some(1))), "1.75.0-beta.1");
        assert_eq!(beta_name(&Beta::new(1, 75, 0, None)), "1.75.0-beta");
    }

    #[test]
    fn a_beta_version_round_trips_through_its_name() {
        for version in [Beta::new(1, 75, 0, Some(1)), Beta::new(1, 75, 0, None)] {
            assert_eq!(beta(&beta_name(&version)), Some(version));
        }
    }

    #[test]
    fn count_the_components_of_a_version() {
        assert_eq!(components("1.53.0"), 3);
        assert_eq!(components("1.53"), 2);
        assert_eq!(components("1.x.0"), 0);
    }
}
