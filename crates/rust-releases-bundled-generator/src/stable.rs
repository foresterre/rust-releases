use crate::error::{DistError, GeneratorError};
use rust_releases_core::rust_release::date::Date;
use rust_releases_core::rust_release::toolchain::{RustVersion, Target};
use rust_releases_core::{Stable, StableReleases};
use rust_releases_rust_dist::{
    AwsError, BlockingAwsDistClient, CachedDistClient, CachedDistError, ReleaseManifest, RustDist,
};
use std::collections::{BTreeMap, BTreeSet};

pub type Dist = RustDist<CachedDistClient<BlockingAwsDistClient>>;

const UNPARSABLE_TRIPLE: &str = "?";

#[derive(Clone, Debug)]
pub struct StableRelease {
    pub date: Date,
    pub hosts: BTreeSet<String>,
}

pub fn releases(
    index: &StableReleases,
    changelog: &StableReleases, // TODO replace completelt with rust-dist
    bundled: &StableReleases,
    dist: &Dist,
) -> Result<BTreeMap<RustVersion, StableRelease>, GeneratorError> {
    let known = bundled_releases(bundled);
    let announced = dates(changelog);

    let versions = index
        .iter()
        .chain(changelog.iter())
        .map(|release| release.version().version)
        .collect::<BTreeSet<_>>();

    let mut releases = BTreeMap::new();

    for version in versions {
        if let Some(release) = known.get(&version) {
            releases.insert(version, release.clone());
            continue;
        }

        eprintln!("  fetching the release manifest of Rust {version}");

        let release = match manifest(dist, &version)? {
            Some(manifest) => StableRelease {
                date: manifest.release_date().clone(),
                hosts: hosts(&manifest, &version)?,
            },
            None => StableRelease {
                date: announced.get(&version).cloned().ok_or_else(|| {
                    GeneratorError::UnknownReleaseDate {
                        version: version.to_string(),
                    }
                })?,
                hosts: BTreeSet::new(),
            },
        };

        releases.insert(version, release);
    }

    Ok(releases)
}

fn manifest(dist: &Dist, version: &RustVersion) -> Result<Option<ReleaseManifest>, GeneratorError> {
    let version = Stable::new(version.major(), version.minor(), version.patch());

    match dist.stable().manifest(&version) {
        Ok(manifest) => Ok(Some(manifest)),
        Err(error) if is_missing(&error) => Ok(None),
        Err(error) => Err(GeneratorError::DistManifest(Box::new(error))),
    }
}

fn is_missing(error: &DistError) -> bool {
    match error {
        CachedDistError::Client(AwsError::GetObjectError { source, .. }) => source.is_no_such_key(),
        _ => false,
    }
}

fn hosts(
    manifest: &ReleaseManifest,
    version: &RustVersion,
) -> Result<BTreeSet<String>, GeneratorError> {
    let unknown = Target::from_target_triple_or_unknown(UNPARSABLE_TRIPLE);

    manifest
        .toolchains()
        .iter()
        .map(|toolchain| {
            let host = toolchain.host();

            if host == &unknown {
                return Err(GeneratorError::UnknownHost {
                    version: version.to_string(),
                });
            }

            Ok(host.to_string())
        })
        .collect()
}

fn bundled_releases(releases: &StableReleases) -> BTreeMap<RustVersion, StableRelease> {
    releases
        .iter()
        .filter(|release| !release.toolchains().is_empty())
        .filter_map(|release| {
            let date = release.release_date()?.clone();
            let hosts = release
                .toolchains_iter()
                .map(|toolchain| toolchain.host().to_string())
                .collect();

            Some((release.version().version, StableRelease { date, hosts }))
        })
        .collect()
}

fn dates(releases: &StableReleases) -> BTreeMap<RustVersion, Date> {
    releases
        .iter()
        .filter_map(|release| {
            release
                .release_date()
                .map(|date| (release.version().version, date.clone()))
        })
        .collect()
}
