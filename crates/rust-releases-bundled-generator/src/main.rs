#![deny(clippy::all)]
#![deny(unsafe_code)]

mod error;
mod r#gen;

use crate::error::GeneratorError;
use rust_releases_bundled::BundledReleases;
use rust_releases_core::rust_release::date::Date;
use rust_releases_core::{BetaReleases, NightlyReleases, StableReleases};
use rust_releases_rust_dist::{BlockingAwsDistClient, CachedDistClient, Detail, RustDist};
use std::path::{Path, PathBuf};

type Dist = RustDist<CachedDistClient<BlockingAwsDistClient>>;

fn main() -> std::process::ExitCode {
    match run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");

            let mut source = std::error::Error::source(&error);
            while let Some(cause) = source {
                eprintln!("  caused by: {cause}");
                source = cause.source();
            }

            std::process::ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), GeneratorError> {
    let generated = generated_dir();
    let bundled = BundledReleases::new();

    eprintln!("listing the Rust distribution bucket");
    let dist = RustDist::new_aws_cached_client().map_err(GeneratorError::DistSetup)?;

    let stable = stable_releases(&dist, &bundled)?;
    let beta = beta_releases(&dist, &bundled)?;
    let nightly = nightly_releases(&dist, &bundled)?;

    let stable = r#gen::stable(&generated.join("stable.rs"), &stable)?;
    eprintln!("wrote {} stable releases", stable.entries);

    let beta = r#gen::beta(&generated.join("beta.rs"), &beta)?;
    eprintln!("wrote {} beta releases", beta.entries);

    let nightly = r#gen::nightly(&generated.join("nightly.rs"), &nightly)?;
    eprintln!("wrote {} nightly releases", nightly.entries);

    let changed = stable.changed || beta.changed || nightly.changed;

    if changed {
        r#gen::metadata(&generated.join("metadata.rs"), &today())?;
    }

    eprintln!(
        "the bundled release data is {}",
        if changed { "updated" } else { "unchanged" }
    );

    Ok(())
}

// The releases which predate the v2 release manifests are dated by the data which is already
// bundled, and have no manifest to read thgat from. Every other release is extended from its own
// manifest.
fn stable_releases(
    dist: &Dist,
    bundled: &BundledReleases,
) -> Result<StableReleases, GeneratorError> {
    let published = dist
        .stable()
        .fetch()
        .map_err(|source| dist_error("stable", source))?;

    let (unmanifested, rest): (Vec<_>, Vec<_>) = bundled
        .stable()
        .merge(published)
        .into_iter()
        .partition(|release| release.release_date().is_some() && release.toolchains().is_empty());

    let extended = dist
        .stable()
        .extend_all(rest.into_iter().collect(), Detail::all())
        .map_err(|source| dist_error("stable", source))?;

    Ok(extended.into_iter().chain(unmanifested).collect())
}

// The beta dist names them without a release date, so the date of a release which is not bundled
// yet is read from its release manifest. The dates which published a beta manifest without a
// version in its name are only known from the bundled data.
fn beta_releases(dist: &Dist, bundled: &BundledReleases) -> Result<BetaReleases, GeneratorError> {
    let published = dist
        .beta()
        .fetch()
        .map_err(|source| dist_error("beta", source))?;

    dist.beta()
        .extend_all(bundled.beta().merge(published), Detail::release_date())
        .map_err(|source| dist_error("beta", source))
}

fn nightly_releases(
    dist: &Dist,
    bundled: &BundledReleases,
) -> Result<NightlyReleases, GeneratorError> {
    let published = dist
        .nightly()
        .fetch()
        .map_err(|source| dist_error("nightly", source))?;

    Ok(bundled.nightly().merge(published))
}

fn dist_error(channel: &'static str, source: error::DistError) -> GeneratorError {
    GeneratorError::Dist {
        channel,
        source: Box::new(source),
    }
}

fn today() -> Date {
    let today = time::OffsetDateTime::now_utc().date();

    Date::new(today.year() as u16, today.month() as u8, today.day())
}

fn generated_dir() -> PathBuf {
    match std::env::args().nth(1) {
        Some(path) => PathBuf::from(path),
        None => {
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../rust-releases-bundled/src/generated")
        }
    }
}
