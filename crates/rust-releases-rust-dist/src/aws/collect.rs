use crate::aws::keys::{self, ChannelName};
use rust_releases_core::{BetaReleases, Nightly, NightlyReleases, RustRelease, StableReleases};

// The release manifests of the rust dist bucket start at 1.8.0, so we need the actual artifacts for
// `<1.8.0`.
pub fn create_stable_releases<'a>(
    manifests: impl IntoIterator<Item = &'a str>,
    artifacts: impl IntoIterator<Item = &'a str>,
) -> StableReleases {
    let mut releases = StableReleases::empty();

    for key in manifests {
        if let Some(ChannelName::Stable(version)) =
            keys::manifest_name(key).and_then(keys::channel_name)
        {
            releases.add(RustRelease::new(version, None, []));
        }
    }

    for key in artifacts {
        if let Some(version) = keys::artifact_version(key) {
            releases.add(RustRelease::new(version, None, []));
        }
    }

    releases
}

// Only the beta manifests which name their version are in scope. The dates which publish a
// 'channel-rust-beta.toml' alone name no version, and resolving them would take a request per date.
pub fn create_beta_releases<'a>(manifests: impl IntoIterator<Item = &'a str>) -> BetaReleases {
    let mut releases = BetaReleases::empty();

    for key in manifests {
        if let Some(ChannelName::Beta(version)) =
            keys::manifest_name(key).and_then(keys::channel_name)
        {
            releases.add(RustRelease::new(version, None, []));
        }
    }

    releases
}

// A nightly is versioned by the date it was published on, so its release date is what enumerating
// the channel already found out.
pub fn create_nightly_releases(dates: impl IntoIterator<Item = Nightly>) -> NightlyReleases {
    dates
        .into_iter()
        .map(|version| {
            let release_date = version.date.clone();

            RustRelease::new(version, Some(release_date), [])
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_releases_core::rust_release::date::Date;
    use rust_releases_core::{Beta, Stable};

    fn artifact_index() -> String {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../resources/rust_dist/dist_static-rust-lang-org.txt");

        std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("unable to read '{}': {error}", path.display()))
    }

    #[test]
    fn collect_the_stable_releases_of_the_artifacts_of_the_bucket() {
        let index = artifact_index();
        let releases = create_stable_releases([], index.lines());

        // 74 releases including minor releases from 1.0.0 to 1.53.0
        assert_eq!(releases.len(), 74);
        assert_eq!(
            releases.iter().next().unwrap().version(),
            &Stable::new(1, 0, 0)
        );
        assert_eq!(
            releases.iter().last().unwrap().version(),
            &Stable::new(1, 53, 0)
        );
    }

    #[test]
    fn collect_the_stable_releases_of_the_release_manifests_of_the_bucket() {
        let manifests = [
            "dist/channel-rust-1.8.0.toml",
            "dist/channel-rust-1.8.0.toml.asc",
            "dist/channel-rust-1.8.toml",
            "dist/channel-rust-1.9.0.toml",
            "dist/channel-rust-1.9.0-date.txt",
            "dist/channel-rust-stable.toml",
            "dist/channel-rust-1.75.0-beta.1.toml",
        ];

        let releases = create_stable_releases(manifests, []);

        assert_eq!(
            releases
                .iter()
                .map(|release| release.version().clone())
                .collect::<Vec<_>>(),
            vec![Stable::new(1, 8, 0), Stable::new(1, 9, 0)]
        );
    }

    #[test]
    fn every_artifact_of_a_release_yields_a_single_release() {
        let artifacts = [
            "dist/rustc-1.53.0-x86_64-apple-darwin.tar.gz",
            "dist/rustc-1.53.0-x86_64-apple-darwin.tar.gz.asc",
            "dist/rustc-1.53.0-x86_64-unknown-linux-gnu.tar.xz",
        ];

        assert_eq!(create_stable_releases([], artifacts).len(), 1);
    }

    #[test]
    fn collected_stable_release_has_no_release_date_and_no_toolchains() {
        let index = artifact_index();
        let releases = create_stable_releases([], index.lines());

        assert!(
            releases
                .iter()
                .all(|release| release.release_date().is_none() && release.toolchains().is_empty())
        );
    }

    #[test]
    fn collect_the_beta_releases_which_name_their_version() {
        let manifests = [
            "dist/channel-rust-1.75.0-beta.1.toml",
            "dist/channel-rust-1.75.0-beta.2.toml",
            "dist/channel-rust-1.75.0-beta.toml",
            "dist/channel-rust-1.75-beta.toml",
            "dist/channel-rust-beta.toml",
            "dist/channel-rust-1.75.0.toml",
        ];

        let releases = create_beta_releases(manifests);

        assert_eq!(
            releases
                .iter()
                .map(|release| release.version().clone())
                .collect::<Vec<_>>(),
            vec![Beta::new(1, 75, 0, Some(1)), Beta::new(1, 75, 0, Some(2))]
        );
    }

    #[test]
    fn a_collected_nightly_release_carries_its_date_as_its_release_date() {
        let releases = create_nightly_releases([Nightly::new(2016, 3, 8)]);
        let release = releases.iter().next().unwrap();

        assert_eq!(release.version(), &Nightly::new(2016, 3, 8));
        assert_eq!(release.release_date(), Some(&Date::new(2016, 3, 8)));
    }
}
