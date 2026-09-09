mod fakes;

use fakes::{DistError, FakeDist, Request, scratch_cache_folder};
use rust_releases_core::rust_release::date::Date;
use rust_releases_core::{
    Beta, BetaReleases, Nightly, NightlyReleases, RustRelease, Stable, StableReleases,
};
use rust_releases_rust_dist::{CachedDistClient, CachedDistError, RustDist};
use std::time::Duration;

fn stable() -> StableReleases {
    StableReleases::new([
        RustRelease::new(Stable::new(1, 8, 0), None, []),
        RustRelease::new(Stable::new(1, 53, 0), None, []),
    ])
}

fn beta() -> BetaReleases {
    BetaReleases::new([RustRelease::new(Beta::new(1, 75, 0, Some(1)), None, [])])
}

fn nightly() -> NightlyReleases {
    NightlyReleases::new([RustRelease::new(
        Nightly::new(2016, 3, 8),
        Some(Date::new(2016, 3, 8)),
        [],
    )])
}

#[test]
fn fetch_the_releases_of_every_channel() {
    let dist = FakeDist::new();
    dist.serve_stable(stable());
    dist.serve_beta(beta());
    dist.serve_nightly(nightly());

    let source = RustDist::new(dist.clone());

    assert_eq!(source.stable().fetch().unwrap(), stable());
    assert_eq!(source.beta().fetch().unwrap(), beta());
    assert_eq!(source.nightly().fetch().unwrap(), nightly());

    assert_eq!(
        dist.requests(),
        vec![
            Request::StableReleases,
            Request::BetaReleases,
            Request::NightlyReleases,
        ]
    );
}

#[tokio::test]
async fn fetch_the_releases_of_every_channel_asynchronously() {
    let dist = FakeDist::new();
    dist.serve_stable(stable());
    dist.serve_beta(beta());
    dist.serve_nightly(nightly());

    let source = RustDist::new(dist.clone());

    assert_eq!(source.stable().fetch_async().await.unwrap(), stable());
    assert_eq!(source.beta().fetch_async().await.unwrap(), beta());
    assert_eq!(source.nightly().fetch_async().await.unwrap(), nightly());
}

#[test]
fn fetch_the_releases_of_a_channel_through_a_cache() {
    let cache_folder = scratch_cache_folder("cached-fetch");

    let dist = FakeDist::new();
    dist.serve_stable(stable());

    let client = CachedDistClient::new(dist.clone(), cache_folder.clone());
    let source = RustDist::new(client);

    let first = source.stable().fetch().unwrap();
    let second = source.stable().fetch().unwrap();

    assert_eq!(first, stable());
    assert_eq!(first, second);
    assert_eq!(dist.requests(), vec![Request::StableReleases]);
    assert!(cache_folder.join("stable-releases.txt").is_file());

    std::fs::remove_dir_all(&cache_folder).unwrap();
}

#[test]
fn a_cached_channel_keeps_the_release_dates_it_carried() {
    let cache_folder = scratch_cache_folder("cached-dates");

    let dist = FakeDist::new();
    dist.serve_nightly(nightly());

    let client = CachedDistClient::new(dist.clone(), cache_folder.clone());
    let source = RustDist::new(client);

    source.nightly().fetch().unwrap();
    let cached = source.nightly().fetch().unwrap();

    assert_eq!(cached, nightly());
    assert_eq!(
        cached.iter().next().unwrap().release_date(),
        Some(&Date::new(2016, 3, 8))
    );
    assert_eq!(dist.requests(), vec![Request::NightlyReleases]);

    std::fs::remove_dir_all(&cache_folder).unwrap();
}

#[test]
fn every_channel_is_cached_on_its_own() {
    let cache_folder = scratch_cache_folder("cached-channels");

    let dist = FakeDist::new();
    dist.serve_stable(stable());
    dist.serve_beta(beta());

    let client = CachedDistClient::new(dist.clone(), cache_folder.clone());
    let source = RustDist::new(client);

    assert_eq!(source.stable().fetch().unwrap(), stable());
    assert_eq!(source.beta().fetch().unwrap(), beta());

    assert!(cache_folder.join("stable-releases.txt").is_file());
    assert!(cache_folder.join("beta-releases.txt").is_file());

    std::fs::remove_dir_all(&cache_folder).unwrap();
}

#[test]
fn a_stale_cache_is_read_again() {
    let cache_folder = scratch_cache_folder("stale-cache");

    let dist = FakeDist::new();
    dist.serve_stable(stable());

    let client = CachedDistClient::new(dist.clone(), cache_folder.clone())
        .with_releases_timeout(Duration::from_secs(0));
    let source = RustDist::new(client);

    source.stable().fetch().unwrap();
    source.stable().fetch().unwrap();

    assert_eq!(
        dist.requests(),
        vec![Request::StableReleases, Request::StableReleases]
    );

    std::fs::remove_dir_all(&cache_folder).unwrap();
}

#[test]
fn a_release_manifest_is_cached_as_it_was_received() {
    let cache_folder = scratch_cache_folder("cached-manifest");

    let dist = FakeDist::new();
    dist.serve_manifest("stable_2016-04-12.toml");

    let client = CachedDistClient::new(dist.clone(), cache_folder.clone());
    let source = RustDist::new(client);

    let version = Stable::new(1, 8, 0);
    let first = source.stable().manifest(&version).unwrap();
    let second = source.stable().manifest(&version).unwrap();

    assert_eq!(first, second);
    assert_eq!(dist.requests(), vec![Request::StableManifest(version)]);

    let cache_file = cache_folder.join("manifests/channel-rust-1.8.0.toml");

    assert_eq!(std::fs::read(&cache_file).unwrap(), first.buffer());

    std::fs::remove_dir_all(&cache_folder).unwrap();
}

#[test]
fn faults_are_reported() {
    let dist = FakeDist::new();
    dist.serve_fault();

    let source = RustDist::new(dist.clone());

    let error = source.stable().fetch().unwrap_err();

    assert_eq!(error, DistError);
    assert_eq!(error.to_string(), "the distribution is unavailable");
}

#[test]
fn faults_of_a_cached_client_are_reported() {
    let cache_folder = scratch_cache_folder("cached-fault");

    let dist = FakeDist::new();
    dist.serve_fault();

    let client = CachedDistClient::new(dist.clone(), cache_folder);
    let source = RustDist::new(client);

    let error = source.beta().fetch().unwrap_err();

    assert!(matches!(&error, CachedDistError::Client(DistError)));
    assert_eq!(
        error.to_string(),
        "Failed to read the Rust distribution: the distribution is unavailable"
    );
}
