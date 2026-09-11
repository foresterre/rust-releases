use rust_releases_bundled::BundledReleases;
use rust_releases_core::rust_release::date::Date;

#[test]
fn the_bundled_data_states_when_it_was_generated() {
    assert!(BundledReleases::new().generated_on() >= Date::new(2026, 9, 9));
}

#[cfg(feature = "stable")]
mod stable {
    use super::*;
    use rust_releases_core::Stable;
    use rust_releases_core::rust_release::toolchain::{Channel, Target};
    use std::ptr;

    #[test]
    fn the_bundled_stable_releases() {
        let releases = BundledReleases::new().stable();

        assert!(releases.len() >= 142);
    }

    #[test]
    fn every_stable_release_states_its_release_date() {
        let releases = BundledReleases::new().stable();

        assert!(releases.iter().all(|r| r.release_date().is_some()));
    }

    #[test]
    fn the_oldest_bundled_stable_release() {
        let releases = BundledReleases::new().stable();
        let oldest = releases.iter().next().unwrap();

        assert_eq!(oldest.version(), &Stable::new(0, 11, 0));
        assert_eq!(oldest.release_date(), Some(&Date::new(2014, 7, 2)));
    }

    #[yare::parameterized(
        first = { Stable::new(1, 0, 0), Date::new(2015, 5, 15) },
        before_the_first_release_manifest = { Stable::new(1, 7, 0), Date::new(2016, 3, 3) },
        first_release_manifest = { Stable::new(1, 8, 0), Date::new(2016, 4, 12) },
        patch = { Stable::new(1, 31, 0), Date::new(2018, 12, 6) },
        recent = { Stable::new(1, 85, 0), Date::new(2025, 2, 20) },
    )]
    fn a_bundled_stable_release(version: Stable, date: Date) {
        let releases = BundledReleases::new().stable();

        let release = releases.iter().find(|r| r.version() == &version).unwrap();

        assert_eq!(release.release_date(), Some(&date));
    }

    #[test]
    fn a_stable_release_has_a_toolchain_per_host() {
        let releases = BundledReleases::new().stable();

        let release = releases
            .iter()
            .find(|r| r.version() == &Stable::new(1, 8, 0))
            .unwrap();

        let hosts = release
            .toolchains_iter()
            .map(|toolchain| toolchain.host().to_string())
            .collect::<Vec<_>>();

        assert_eq!(hosts.len(), 8);
        assert!(hosts.contains(&"x86_64-unknown-linux-gnu".to_string()));
    }

    #[test]
    fn the_toolchains_of_a_stable_release_carry_its_channel_and_release_date() {
        let releases = BundledReleases::new().stable();

        let release = releases
            .iter()
            .find(|r| r.version() == &Stable::new(1, 85, 0))
            .unwrap();

        assert!(release.toolchains_iter().all(|toolchain| {
            toolchain.channel() == &Channel::Stable(Stable::new(1, 85, 0))
                && toolchain.date() == Some(&Date::new(2025, 2, 20))
        }));
    }

    #[test]
    fn the_toolchains_of_a_stable_release_state_their_components_and_targets() {
        let releases = BundledReleases::new().stable();

        assert!(releases.iter().all(|release| {
            release.toolchains_iter().all(|toolchain| {
                !toolchain.components().is_empty() && !toolchain.targets().is_empty()
            })
        }));
    }

    #[test]
    fn a_toolchain_targets_its_own_host() {
        let releases = BundledReleases::new().stable();

        assert!(releases.iter().all(|release| {
            release
                .toolchains_iter()
                .all(|toolchain| toolchain.targets().contains(toolchain.host()))
        }));
    }

    #[test]
    fn the_components_and_targets_of_a_bundled_toolchain() {
        let releases = BundledReleases::new().stable();

        let release = releases
            .iter()
            .find(|r| r.version() == &Stable::new(1, 8, 0))
            .unwrap();

        let host = Target::from_target_triple_or_unknown("x86_64-unknown-linux-gnu");
        let toolchain = release
            .toolchains_iter()
            .find(|toolchain| toolchain.host() == &host)
            .unwrap();

        let components = toolchain
            .components()
            .iter()
            .map(|component| component.name())
            .collect::<Vec<_>>();

        assert_eq!(components, ["cargo", "rust-docs", "rust-std", "rustc"]);
        assert_eq!(toolchain.targets().len(), 31);
        assert!(toolchain.targets().contains(&host));
    }

    // What the bundled data is interned for: a set which is shipped by many toolchains is bundled,
    // and held in memory, once.
    #[test]
    fn toolchains_which_ship_the_same_targets_share_a_single_set() {
        let releases = BundledReleases::new().stable();

        let release = releases
            .iter()
            .find(|r| r.version() == &Stable::new(1, 85, 0))
            .unwrap();

        let mut toolchains = release.toolchains_iter();
        let first = toolchains.next().unwrap();
        let shared = toolchains
            .find(|toolchain| toolchain.targets() == first.targets())
            .expect("a release ships the same targets from more than one host");

        assert!(ptr::eq(
            first.targets().as_slice(),
            shared.targets().as_slice()
        ));
    }

    #[test]
    fn the_releases_which_predate_the_release_manifests_have_no_toolchains() {
        let releases = BundledReleases::new().stable();

        let release = releases
            .iter()
            .find(|r| r.version() == &Stable::new(1, 7, 0))
            .unwrap();

        assert!(release.toolchains().is_empty());
    }
}

#[cfg(feature = "beta")]
mod beta {
    use super::*;
    use rust_releases_core::Beta;

    #[test]
    fn the_bundled_beta_releases() {
        let releases = BundledReleases::new().beta();

        assert!(releases.len() >= 617);
    }

    #[test]
    fn every_beta_release_states_its_release_date() {
        let releases = BundledReleases::new().beta();

        assert!(releases.iter().all(|r| r.release_date().is_some()));
    }

    #[yare::parameterized(
        oldest = { Beta::new(1, 8, 0, Some(2)), Date::new(2016, 3, 23) },
        named_by_its_manifest = { Beta::new(1, 75, 0, Some(1)), Date::new(2023, 11, 13) },
        republished = { Beta::new(1, 15, 0, Some(1)), Date::new(2016, 12, 20) },
    )]
    fn a_bundled_beta_release(version: Beta, date: Date) {
        let releases = BundledReleases::new().beta();

        let release = releases.iter().find(|r| r.version() == &version).unwrap();

        assert_eq!(release.release_date(), Some(&date));
    }
}

#[cfg(feature = "nightly")]
mod nightly {
    use super::*;
    use rust_releases_core::Nightly;

    #[test]
    fn the_bundled_nightly_releases() {
        let releases = BundledReleases::new().nightly();

        assert!(releases.len() >= 3605);
    }

    #[test]
    fn the_oldest_bundled_nightly_release() {
        let releases = BundledReleases::new().nightly();
        let oldest = releases.iter().next().unwrap();

        assert_eq!(oldest.version(), &Nightly::new(2016, 3, 8));
    }

    #[test]
    fn every_nightly_release_is_dated_by_its_version() {
        let releases = BundledReleases::new().nightly();

        assert!(
            releases
                .iter()
                .all(|r| r.release_date() == Some(&r.version().date))
        );
    }
}
