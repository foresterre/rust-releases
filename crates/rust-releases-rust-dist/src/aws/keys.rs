use crate::version;
use rust_releases_core::{Beta, Nightly, Stable};

pub const CHANNEL_MANIFEST_PREFIX: &str = "dist/channel-rust-";

// The artifacts of a stable release. Used for th releases which were released before the v2 release
// manifests
const ARTIFACT_PREFIX: &str = "dist/rustc-";

// Rust 1.8.0 is the release which introduced the v2 release manifests, so 1.0.0 up to 1.7.0 are the
// only releases which have to be recovered from the artifacts they published.
const FIRST_MANIFEST_V2_MINOR: u64 = 8;

// Index of (dated) release manifests
pub const MANIFEST_INDEX_KEY: &str = "manifests.txt";

const MANIFEST_SUFFIX: &str = ".toml";

const BETA_SUFFIX: &str = "-beta";

const NIGHTLY_NAME: &str = "nightly";

const BETA_NAME: &str = "beta";

const STABLE_NAME: &str = "stable";

pub fn stable_manifest(version: &Stable) -> String {
    format!(
        "{CHANNEL_MANIFEST_PREFIX}{version}{MANIFEST_SUFFIX}",
        version = version.version
    )
}

pub fn beta_manifest(version: &Beta) -> String {
    format!(
        "{CHANNEL_MANIFEST_PREFIX}{name}{MANIFEST_SUFFIX}",
        name = version::beta_name(version)
    )
}

pub fn nightly_manifest(version: &Nightly) -> String {
    format!(
        "dist/{date}/channel-rust-{NIGHTLY_NAME}{MANIFEST_SUFFIX}",
        date = version.date.ymd()
    )
}

// artifacts are used by stable versions, but only for 1.0.0 to (incl) 1.7.0
pub fn artifact_prefixes_up_to_1d8d0() -> impl Iterator<Item = String> {
    // trailing dot keeps '1.1.' from matching 1.10.0 and up
    (0..FIRST_MANIFEST_V2_MINOR).map(|minor| format!("{ARTIFACT_PREFIX}1.{minor}."))
}

pub fn artifact_version(key: &str) -> Option<Stable> {
    let (number, _) = key.strip_prefix(ARTIFACT_PREFIX)?.split_once('-')?;

    version::rust_version(number).map(Stable::from)
}

pub fn manifest_name(key: &str) -> Option<&str> {
    key.strip_prefix(CHANNEL_MANIFEST_PREFIX)?
        .strip_suffix(MANIFEST_SUFFIX)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChannelName {
    Stable(Stable),
    Beta(Beta),
    Nightly,
    // E.g. the rolling 'stable', 'beta' and two component aliases
    Other,
}

pub fn channel_name(name: &str) -> Option<ChannelName> {
    match name {
        NIGHTLY_NAME => return Some(ChannelName::Nightly),
        BETA_NAME | STABLE_NAME => return Some(ChannelName::Other),
        _ => {}
    }

    // A beta manifest names a release only when its name has a prerelease number
    if let Some(version) = version::beta(name) {
        return Some(match version.prerelease {
            Some(_) => ChannelName::Beta(version),
            None => ChannelName::Other,
        });
    }

    if let Some(version) = version::rust_version(name) {
        return Some(ChannelName::Stable(Stable::from(version)));
    }

    // there are also two component aliasses...
    match version::components(name.strip_suffix(BETA_SUFFIX).unwrap_or(name)) {
        2 => Some(ChannelName::Other),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_object_key_of_a_stable_release_manifest() {
        assert_eq!(
            stable_manifest(&Stable::new(1, 53, 0)),
            "dist/channel-rust-1.53.0.toml"
        );
    }

    #[test]
    fn the_object_key_of_a_beta_release_manifest() {
        assert_eq!(
            beta_manifest(&Beta::new(1, 75, 0, Some(1))),
            "dist/channel-rust-1.75.0-beta.1.toml"
        );
        assert_eq!(
            beta_manifest(&Beta::new(1, 75, 0, None)),
            "dist/channel-rust-1.75.0-beta.toml"
        );
    }

    #[test]
    fn the_object_key_of_a_nightly_release_manifest() {
        assert_eq!(
            nightly_manifest(&Nightly::new(2016, 3, 8)),
            "dist/2016-03-08/channel-rust-nightly.toml"
        );
    }

    #[test]
    fn the_prefixes_of_the_artifacts_which_predate_the_release_manifests() {
        let prefixes = artifact_prefixes_up_to_1d8d0().collect::<Vec<_>>();

        assert_eq!(prefixes.len(), 8);
        assert_eq!(prefixes.first().unwrap(), "dist/rustc-1.0.");
        assert_eq!(prefixes.last().unwrap(), "dist/rustc-1.7.");
    }

    #[test]
    fn an_artifact_prefix_does_not_match_a_later_release() {
        let prefixes = artifact_prefixes_up_to_1d8d0().collect::<Vec<_>>();

        assert!(
            !prefixes
                .iter()
                .any(|prefix| "dist/rustc-1.10.0-x86_64-apple-darwin.tar.gz".starts_with(prefix))
        );
        assert!(
            prefixes
                .iter()
                .any(|prefix| "dist/rustc-1.1.0-x86_64-apple-darwin.tar.gz".starts_with(prefix))
        );
    }

    #[test]
    fn the_version_of_an_artifact() {
        assert_eq!(
            artifact_version("dist/rustc-1.53.0-x86_64-apple-darwin.tar.gz"),
            Some(Stable::new(1, 53, 0))
        );
    }

    #[yare::parameterized(
        another_package = { "dist/rust-1.53.0-x86_64-apple-darwin.tar.gz" },
        dated = { "dist/2021-06-17/rustc-1.53.0-x86_64-apple-darwin.tar.gz" },
        two_components = { "dist/rustc-1.53-x86_64-apple-darwin.tar.gz" },
        no_target = { "dist/rustc-1.53.0.tar.gz" },
    )]
    fn skip_an_object_which_is_not_the_artifact_of_a_stable_release(key: &str) {
        assert_eq!(artifact_version(key), None);
    }

    #[test]
    fn the_name_of_a_release_manifest() {
        assert_eq!(
            manifest_name("dist/channel-rust-1.53.0.toml"),
            Some("1.53.0")
        );
        assert_eq!(manifest_name("dist/channel-rust-1.53.0.toml.asc"), None);
        assert_eq!(manifest_name("dist/channel-rust-1.53.0-date.txt"), None);
    }

    #[test]
    fn the_channel_of_a_stable_manifest_name() {
        assert_eq!(
            channel_name("1.53.0"),
            Some(ChannelName::Stable(Stable::new(1, 53, 0)))
        );
    }

    #[test]
    fn the_channel_of_a_beta_manifest_name() {
        assert_eq!(
            channel_name("1.75.0-beta.1"),
            Some(ChannelName::Beta(Beta::new(1, 75, 0, Some(1))))
        );
    }

    #[test]
    fn the_channel_of_a_nightly_manifest_name() {
        assert_eq!(channel_name("nightly"), Some(ChannelName::Nightly));
    }

    #[yare::parameterized(
        rolling_stable = { "stable" },
        rolling_beta = { "beta" },
        two_component_stable = { "1.48" },
        two_component_beta = { "1.75-beta" },
        unnumbered_beta = { "1.75.0-beta" },
    )]
    fn a_manifest_name_which_does_not_name_a_release(name: &str) {
        assert_eq!(channel_name(name), Some(ChannelName::Other));
    }

    #[yare::parameterized(
        empty = { "" },
        unknown_channel = { "gamma" },
        four_components = { "1.2.3.4" },
        not_a_number = { "1.x.0" },
        prerelease_which_is_not_a_number = { "1.75.0-beta.x" },
    )]
    fn an_unrecognized_manifest_name(name: &str) {
        assert_eq!(channel_name(name), None);
    }
}
