mod fakes;

use fakes::{DistError, FakeDist, Request};
use rust_releases_core::rust_release::date::Date;
use rust_releases_core::rust_release::toolchain::{Channel, Target};
use rust_releases_core::{
    Beta, BetaReleases, Nightly, NightlyReleases, RustRelease, Stable, StableReleases,
};
use rust_releases_rust_dist::{Detail, RustDist};

#[test]
fn fetch_the_release_manifest_of_a_stable_release() {
    let dist = FakeDist::new();
    dist.serve_manifest("stable_2016-04-12.toml");

    let source = RustDist::new(dist.clone());

    let manifest = source.stable().manifest(&Stable::new(1, 8, 0)).unwrap();

    assert_eq!(manifest.channel(), &Channel::Stable(Stable::new(1, 8, 0)));
    assert_eq!(manifest.release_date(), &Date::new(2016, 4, 12));
    assert_eq!(manifest.toolchains().len(), 8);
    assert_eq!(
        dist.requests(),
        vec![Request::StableManifest(Stable::new(1, 8, 0))]
    );
}

#[test]
fn fetch_the_release_manifest_of_a_beta_release() {
    let dist = FakeDist::new();
    dist.serve_manifest("beta_2016-03-23.toml");

    let source = RustDist::new(dist.clone());

    let version = Beta::new(1, 8, 0, Some(2));
    let manifest = source.beta().manifest(&version).unwrap();

    assert_eq!(manifest.channel(), &Channel::Beta(version.clone()));
    assert_eq!(manifest.release_date(), &Date::new(2016, 3, 23));
    assert_eq!(dist.requests(), vec![Request::BetaManifest(version)]);
}

#[test]
fn fetch_the_release_manifest_of_a_nightly_release() {
    let dist = FakeDist::new();
    dist.serve_manifest("nightly_2016-03-08.toml");

    let source = RustDist::new(dist.clone());

    let version = Nightly::new(2016, 3, 8);
    let manifest = source.nightly().manifest(&version).unwrap();

    assert_eq!(manifest.channel(), &Channel::Nightly(version.clone()));
    assert_eq!(manifest.release_date(), &Date::new(2016, 3, 8));
    assert_eq!(dist.requests(), vec![Request::NightlyManifest(version)]);
}

#[test]
fn the_toolchains_of_a_release_manifest_describe_a_host() {
    let dist = FakeDist::new();
    dist.serve_manifest("stable_2016-04-12.toml");

    let source = RustDist::new(dist);

    let manifest = source.stable().manifest(&Stable::new(1, 8, 0)).unwrap();

    let host = Target::from_target_triple_or_unknown("x86_64-unknown-linux-gnu");
    let toolchain = manifest
        .toolchains()
        .iter()
        .find(|toolchain| toolchain.host() == &host)
        .unwrap();

    assert_eq!(toolchain.channel(), &Channel::Stable(Stable::new(1, 8, 0)));
    assert_eq!(toolchain.date(), Some(&Date::new(2016, 4, 12)));

    let mut components = toolchain
        .components()
        .iter()
        .map(|component| component.name())
        .collect::<Vec<_>>();
    components.sort_unstable();

    assert_eq!(components, ["cargo", "rust-docs", "rust-std", "rustc"]);
    assert_eq!(toolchain.targets().len(), 31);
    assert!(toolchain.targets().contains(&host));
}

#[test]
fn extend_a_release_with_its_release_manifest() {
    let dist = FakeDist::new();
    dist.serve_manifest("stable_2016-04-12.toml");

    let source = RustDist::new(dist);

    let release = RustRelease::new(Stable::new(1, 8, 0), None, []);
    let extended = source.stable().extend(release, Detail::all()).unwrap();

    assert_eq!(extended.release_date(), Some(&Date::new(2016, 4, 12)));
    assert_eq!(extended.toolchains().len(), 8);
}

#[test]
fn extend_a_release_with_the_fields_which_are_asked_for() {
    let dist = FakeDist::new();
    dist.serve_manifest("stable_2016-04-12.toml");

    let source = RustDist::new(dist);

    let release = RustRelease::new(Stable::new(1, 8, 0), None, []);
    let extended = source
        .stable()
        .extend(release, Detail::release_date())
        .unwrap();

    assert_eq!(extended.release_date(), Some(&Date::new(2016, 4, 12)));
    assert!(extended.toolchains().is_empty());

    let release = RustRelease::new(Stable::new(1, 8, 0), None, []);
    let extended = source
        .stable()
        .extend(release, Detail::toolchains())
        .unwrap();

    assert_eq!(extended.release_date(), None);
    assert_eq!(extended.toolchains().len(), 8);
}

#[test]
fn extending_a_release_with_nothing_takes_no_request() {
    let dist = FakeDist::new();
    dist.serve_manifest("stable_2016-04-12.toml");

    let source = RustDist::new(dist.clone());

    let release = RustRelease::new(Stable::new(1, 8, 0), None, []);
    let extended = source.stable().extend(release, Detail::default()).unwrap();

    assert_eq!(extended.release_date(), None);
    assert!(dist.requests().is_empty());
}

#[test]
fn extend_a_batch_of_releases_with_their_release_manifests() {
    let dist = FakeDist::new();
    dist.serve_manifest("beta_2016-03-23.toml");

    let source = RustDist::new(dist.clone());

    let releases = BetaReleases::new([RustRelease::new(Beta::new(1, 8, 0, Some(2)), None, [])]);

    let extended = source.beta().extend_all(releases, Detail::all()).unwrap();

    assert_eq!(
        extended.iter().next().unwrap().release_date(),
        Some(&Date::new(2016, 3, 23))
    );
    assert_eq!(
        dist.requests(),
        vec![Request::BetaManifest(Beta::new(1, 8, 0, Some(2)))]
    );
}

#[test]
fn a_release_which_already_carries_what_is_asked_for_takes_no_request() {
    let dist = FakeDist::new();
    dist.serve_manifest("nightly_2016-03-08.toml");

    let source = RustDist::new(dist.clone());

    let releases = NightlyReleases::new([RustRelease::new(
        Nightly::new(2016, 3, 8),
        Some(Date::new(2016, 3, 8)),
        [],
    )]);

    let extended = source
        .nightly()
        .extend_all(releases, Detail::release_date())
        .unwrap();

    assert_eq!(extended.len(), 1);
    assert!(dist.requests().is_empty());
}

#[tokio::test]
async fn extend_a_batch_of_releases_asynchronously() {
    let dist = FakeDist::new();
    dist.serve_manifest("stable_2016-04-12.toml");

    let source = RustDist::new(dist.clone());

    let releases = StableReleases::new([RustRelease::new(Stable::new(1, 8, 0), None, [])]);

    let extended = source
        .stable()
        .extend_all_async(releases, Detail::all())
        .await
        .unwrap();

    assert_eq!(
        extended.iter().next().unwrap().release_date(),
        Some(&Date::new(2016, 4, 12))
    );
    assert_eq!(
        dist.requests(),
        vec![Request::StableManifest(Stable::new(1, 8, 0))]
    );
}

#[test]
fn fetch_the_releases_of_a_channel_with_their_detail() {
    let dist = FakeDist::new();
    dist.serve_stable(StableReleases::new([RustRelease::new(
        Stable::new(1, 8, 0),
        None,
        [],
    )]));
    dist.serve_manifest("stable_2016-04-12.toml");

    let source = RustDist::new(dist.clone());

    let releases = source.stable().fetch_detailed(Detail::all()).unwrap();

    assert_eq!(releases.len(), 1);

    let release = releases.iter().next().unwrap();

    assert_eq!(release.version(), &Stable::new(1, 8, 0));
    assert_eq!(release.release_date(), Some(&Date::new(2016, 4, 12)));
    assert_eq!(release.toolchains().len(), 8);
}

#[test]
fn download_faults_are_reported() {
    let dist = FakeDist::new();
    dist.serve_manifest("stable_2016-04-12.toml");
    dist.serve_fault();

    let source = RustDist::new(dist);

    let error = source.stable().manifest(&Stable::new(1, 8, 0)).unwrap_err();

    assert_eq!(error, DistError);
}
