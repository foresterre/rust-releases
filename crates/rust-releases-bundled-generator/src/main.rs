#![deny(clippy::all)]
#![deny(unsafe_code)]

mod error;
mod r#gen;
mod stable;

use crate::error::GeneratorError;
use crate::stable::Dist;
use rust_releases_bundled::BundledReleases;
use rust_releases_core::rust_release::date::Date;
use rust_releases_core::{BetaReleases, NightlyReleases};
use rust_releases_io::{HttpClient, UreqTransport};
use rust_releases_rust_changelog::{RUST_CHANGELOG_URL, RustChangelog};
use rust_releases_rust_dist::{Detail, RustDist};
use std::path::{Path, PathBuf};
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(120);

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
    let client = HttpClient::new(UreqTransport::new()).with_timeout(TIMEOUT);

    eprintln!("listing the Rust distribution bucket");
    let dist = RustDist::new_aws_cached_client().map_err(GeneratorError::DistSetup)?;
    let index = dist
        .stable()
        .fetch()
        .map_err(|source| dist_error("stable", source))?;

    eprintln!("fetching {RUST_CHANGELOG_URL}");
    let changelog =
        RustChangelog::new(client)
            .fetch()
            .map_err(|source| GeneratorError::Changelog {
                url: RUST_CHANGELOG_URL.to_string(),
                source,
            })?;

    let stable = stable::releases(&index, &changelog, &bundled.stable(), &dist)?;

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

// The beta releases which name their version. The distribution names them without a release date,
// so the date of a release which is not bundled yet is read from its release manifest. The dates
// which published a beta manifest without a version in its name are only known from the bundled
// data.
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
