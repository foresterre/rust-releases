use crate::error::GeneratorError;
use rust_releases_core::RustRelease;
use rust_releases_core::rust_release::toolchain::{Target, Toolchain};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

const UNPARSABLE_TRIPLE: &str = "?";

// The interned release data of a channel, as it is written out by the generator
//
// Every table is ordered by its own contents, so the same releases always output the same tables,
// and a regeneration which has no new data writes no diff.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct Tables {
    pub triples: Vec<String>,
    pub components: Vec<String>,
    pub target_sets: Vec<Vec<u16>>,
    pub component_sets: Vec<Vec<u16>>,
    pub toolchains: Vec<(u16, u32, u32)>,
    pub release_toolchains: Vec<Vec<u32>>,
}

// The toolchains a release published, as an index into `Tables::release_toolchains`
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ReleaseToolchains(pub u32);

/// Interns the toolchains of a set of releases into tables of distinct values.
///
/// Returns the tables, and the toolchains of each release, in the order the releases were given.
pub fn intern<'a, V>(
    releases: impl IntoIterator<Item = &'a RustRelease<V>>,
    version: impl Fn(&V) -> String,
) -> Result<(Tables, Vec<ReleaseToolchains>), GeneratorError>
where
    V: Debug + 'a,
{
    let releases = releases.into_iter().collect::<Vec<_>>();

    let unknown = Target::from_target_triple_or_unknown(UNPARSABLE_TRIPLE);
    let mut triples = BTreeSet::new();
    let mut components = BTreeSet::new();

    for release in &releases {
        for toolchain in release.toolchains_iter() {
            if toolchain.host() == &unknown {
                return Err(GeneratorError::UnknownHost {
                    version: version(release.version()),
                });
            }

            triples.insert(toolchain.host().to_string());
            triples.extend(target_triples(toolchain));
            components.extend(component_names(toolchain));
        }
    }

    let (triples, triple_ids) = table(triples);
    let (components, component_ids) = table(components);

    let target_set_of = |toolchain: &Toolchain| ids(target_triples(toolchain), &triple_ids);
    let component_set_of = |toolchain: &Toolchain| ids(component_names(toolchain), &component_ids);

    let mut target_sets = BTreeSet::new();
    let mut component_sets = BTreeSet::new();

    for release in &releases {
        for toolchain in release.toolchains_iter() {
            target_sets.insert(target_set_of(toolchain));
            component_sets.insert(component_set_of(toolchain));
        }
    }

    let (target_sets, target_set_ids) = table(target_sets);
    let (component_sets, component_set_ids) = table(component_sets);

    let toolchain_of = |toolchain: &Toolchain| {
        (
            triple_ids[&toolchain.host().to_string()] as u16,
            target_set_ids[&target_set_of(toolchain)] as u32,
            component_set_ids[&component_set_of(toolchain)] as u32,
        )
    };

    let mut toolchains = BTreeSet::new();

    for release in &releases {
        for toolchain in release.toolchains_iter() {
            toolchains.insert(toolchain_of(toolchain));
        }
    }

    let (toolchains, toolchain_ids) = table(toolchains);

    let of_release = releases
        .iter()
        .map(|release| {
            release
                .toolchains_iter()
                .map(|toolchain| toolchain_ids[&toolchain_of(toolchain)] as u32)
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (release_toolchains, release_toolchain_ids) =
        table(of_release.iter().cloned().collect::<BTreeSet<_>>());

    let of_release = of_release
        .iter()
        .map(|toolchains| ReleaseToolchains(release_toolchain_ids[toolchains] as u32))
        .collect();

    Ok((
        Tables {
            triples,
            components,
            target_sets,
            component_sets,
            toolchains,
            release_toolchains,
        },
        of_release,
    ))
}

fn target_triples(toolchain: &Toolchain) -> impl Iterator<Item = String> {
    toolchain.targets().iter().map(|target| target.to_string())
}

fn component_names(toolchain: &Toolchain) -> impl Iterator<Item = String> {
    toolchain
        .components()
        .iter()
        .map(|component| component.name().to_owned())
}

// Lays out the distinct values of a table, and the mapping between the value and the index into the vec
fn table<T: Ord + Clone>(values: BTreeSet<T>) -> (Vec<T>, BTreeMap<T, usize>) {
    let values = values.into_iter().collect::<Vec<_>>();
    let index = values
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, value)| (value, index))
        .collect();

    (values, index)
}

fn ids(values: impl Iterator<Item = String>, index: &BTreeMap<String, usize>) -> Vec<u16> {
    let mut ids = values.map(|value| index[&value] as u16).collect::<Vec<_>>();
    ids.sort_unstable();
    ids.dedup();

    ids
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_releases_core::rust_release::date::Date;
    use rust_releases_core::rust_release::toolchain::{Channel, Component, Target};
    use rust_releases_core::{RustRelease, Stable};

    fn toolchain(version: Stable, host: &str, targets: &[&str], components: &[&str]) -> Toolchain {
        Toolchain::new(
            Channel::Stable(version),
            Some(Date::new(2016, 4, 12)),
            Target::from_target_triple_or_unknown(host),
            components
                .iter()
                .map(|name| Component::new(name.to_string()))
                .collect(),
            targets
                .iter()
                .map(|triple| Target::from_target_triple_or_unknown(triple))
                .collect(),
        )
    }

    fn describe(version: &Stable) -> String {
        version.version.to_string()
    }

    fn release(version: Stable, toolchains: Vec<Toolchain>) -> RustRelease<Stable> {
        RustRelease::new(version, Some(Date::new(2016, 4, 12)), toolchains)
    }

    #[test]
    fn a_release_without_toolchains() {
        let releases = [release(Stable::new(1, 7, 0), vec![])];

        let (tables, of_release) = intern(releases.iter(), describe).unwrap();

        assert!(tables.triples.is_empty());
        assert_eq!(tables.release_toolchains, vec![Vec::<u32>::new()]);
        assert_eq!(of_release, vec![ReleaseToolchains(0)]);
    }

    #[test]
    fn a_triple_is_interned_once_per_channel() {
        let host = "x86_64-unknown-linux-gnu";
        let releases = [
            release(
                Stable::new(1, 8, 0),
                vec![toolchain(Stable::new(1, 8, 0), host, &[host], &["rustc"])],
            ),
            release(
                Stable::new(1, 9, 0),
                vec![toolchain(Stable::new(1, 9, 0), host, &[host], &["rustc"])],
            ),
        ];

        let (tables, _) = intern(releases.iter(), describe).unwrap();

        assert_eq!(tables.triples, vec![host.to_string()]);
        assert_eq!(tables.components, vec!["rustc".to_string()]);
    }

    #[test]
    fn toolchains_which_ship_the_same_targets_share_a_set() {
        let releases = [release(
            Stable::new(1, 8, 0),
            vec![
                toolchain(
                    Stable::new(1, 8, 0),
                    "x86_64-unknown-linux-gnu",
                    &["wasm32-unknown-unknown"],
                    &["rustc"],
                ),
                toolchain(
                    Stable::new(1, 8, 0),
                    "x86_64-apple-darwin",
                    &["wasm32-unknown-unknown"],
                    &["rustc"],
                ),
            ],
        )];

        let (tables, _) = intern(releases.iter(), describe).unwrap();

        assert_eq!(tables.toolchains.len(), 2);
        assert_eq!(tables.target_sets.len(), 1);
        assert_eq!(tables.component_sets.len(), 1);
    }

    #[test]
    fn releases_which_published_the_same_toolchains_share_an_entry() {
        let toolchains = |version| {
            vec![toolchain(
                version,
                "x86_64-unknown-linux-gnu",
                &["x86_64-unknown-linux-gnu"],
                &["rustc"],
            )]
        };

        // A toolchain states its own channel, so the two releases only share an entry when the
        // channel is disregarded, which is what makes the entry worth sharing at all.
        let releases = [
            release(Stable::new(1, 8, 0), toolchains(Stable::new(1, 8, 0))),
            release(Stable::new(1, 9, 0), toolchains(Stable::new(1, 8, 0))),
        ];

        let (tables, of_release) = intern(releases.iter(), describe).unwrap();

        assert_eq!(tables.release_toolchains.len(), 1);
        assert_eq!(of_release, vec![ReleaseToolchains(0), ReleaseToolchains(0)]);
    }

    #[test]
    fn report_a_toolchain_whose_host_could_not_be_parsed() {
        let releases = [release(
            Stable::new(1, 8, 0),
            vec![toolchain(Stable::new(1, 8, 0), "?", &[], &[])],
        )];

        let error = intern(releases.iter(), describe).unwrap_err();

        assert!(matches!(error, GeneratorError::UnknownHost { .. }));
    }
}
