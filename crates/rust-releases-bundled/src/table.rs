use rust_releases_core::rust_release::date::Date;
use rust_releases_core::rust_release::toolchain::{
    Channel, Component, ComponentSet, Target, TargetSet, Toolchain,
};

pub struct Bundle {
    tables: &'static Tables,
    triples: Vec<Target>,
    target_sets: Vec<TargetSet>,
    component_sets: Vec<ComponentSet>,
}

impl Bundle {
    pub fn new(tables: &'static Tables) -> Self {
        let triples = tables
            .triples
            .iter()
            .map(|triple| Target::from_target_triple_or_unknown(triple))
            .collect::<Vec<_>>();

        // A component name is bundled as a `&'static str`, so a component does not own its name
        let components = tables
            .components
            .iter()
            .map(|name| Component::new(*name))
            .collect::<Vec<_>>();

        let target_sets = tables
            .target_sets
            .iter()
            .map(|set| set.iter().map(|&id| triples[id as usize].clone()).collect())
            .collect();

        let component_sets = tables
            .component_sets
            .iter()
            .map(|set| {
                set.iter()
                    .map(|&id| components[id as usize].clone())
                    .collect()
            })
            .collect();

        Self {
            tables,
            triples,
            target_sets,
            component_sets,
        }
    }

    pub fn toolchains(
        &self,
        channel: &Channel,
        release_date: &Date,
        release_toolchains: u32,
    ) -> Vec<Toolchain> {
        self.tables.release_toolchains[release_toolchains as usize]
            .iter()
            .map(|&id| {
                let (host, targets, components) = self.tables.toolchains[id as usize];

                Toolchain::new(
                    channel.clone(),
                    Some(release_date.clone()),
                    self.triples[host as usize].clone(),
                    self.component_sets[components as usize].clone(),
                    self.target_sets[targets as usize].clone(),
                )
            })
            .collect()
    }
}

/// # Bundled release data (approximate generated file sizes)
///
/// Crates.io has a 10MB limit [1] per crate. This isn't a problem yet, because we currently don't
/// bundle all toolchain and component data for beta and nightly releases. For stable we do bundle
/// the host, the targets and the components of every release which published a release manifest
/// (1.8.0 and up); the releases before it only have a release date, because their artifacts are all
/// the distribution names them by. Bundling the beta and nightly toolchains is something I want to
/// do. To make it possible, without splitting this crate into multiple crates, I have decided to intern
/// certain repeating values (compression would probably also have worked, but I wasn't willing to
/// include a compressor and decompressor inside the crate :P).
///
/// As a reference, these are the current file sizes generated, as of 2026-09-09 (and remember, they
/// will only grow over time):
///
///   ┌────────────┬─────────────────┬─────────────────────────────┬──────────────────────────────┐
///   │    File    │ Inline literals │ Interned tables, tuple rows │ Interned tables, struct rows │
///   ├────────────┼─────────────────┼─────────────────────────────┼──────────────────────────────┤
///   │ stable.rs  │        128.2 kB │                     72.5 kB │                      74.5 kB │
///   ├────────────┼─────────────────┼─────────────────────────────┼──────────────────────────────┤
///   │ beta.rs    │         48.6 kB │                     26.9 kB │                      34.8 kB │
///   ├────────────┼─────────────────┼─────────────────────────────┼──────────────────────────────┤
///   │ nightly.rs │        173.4 kB │                     80.7 kB │                     124.1 kB │
///   ├────────────┼─────────────────┼─────────────────────────────┼──────────────────────────────┤
///   │ total      │        350.1 kB │                    180.1 kB │                     233.4 kB │
///   └────────────┴─────────────────┴─────────────────────────────┴──────────────────────────────┘
///
/// If we would provide all toolchain data (for stable incl. beta and nightly), we would have the following
/// file sizes (approximatedly anyways, I extrapolated from a subset of release manifests), and again
/// they will only grow in the future with new releases:
///
///   ┌────────────┬─────────────────┬─────────────────────────────┬──────────────────────────────┐
///   │    File    │ Inline literals │ Interned tables, tuple rows │ Interned tables, struct rows │
///   ├────────────┼─────────────────┼─────────────────────────────┼──────────────────────────────┤
///   │ stable.rs  │      9,106.8 kB │                     72.5 kB │                      74.5 kB │
///   ├────────────┼─────────────────┼─────────────────────────────┼──────────────────────────────┤
///   │ beta.rs    │     44,268.3 kB │                    155.5 kB │                     163.3 kB │
///   ├────────────┼─────────────────┼─────────────────────────────┼──────────────────────────────┤
///   │ nightly.rs │    258,482.7 kB │                    242.0 kB │                     285.4 kB │
///   ├────────────┼─────────────────┼─────────────────────────────┼──────────────────────────────┤
///   │ total      │    311,857.8 kB │                    469.9 kB │                     523.2 kB │
///   └────────────┴─────────────────┴─────────────────────────────┴──────────────────────────────┘
///
/// As you can see, the generated files get reasonably large when you use a lot of inline literals,
/// and the total would be above the 10MB limit (even for stable only, we would already today be close
/// to the limits).
///
/// This is why I decided to intern the str's, to reduce duplication which I hypothesized would
/// work very well for this crate, since target triples and components are duplicated so many times
/// if you take the complete set of Rust releases available now.
///
/// I've chosen, for now, to use a simple struct (internal to this crate) instead of tuples, because
/// to construct the table's interned values (iso tuples which would produce slightly smaller
/// file sizes) for the better readability. I might switch to tuples in the future if I consider
/// it worth it, but the big win is in the interning itself, which has the con that it makes the
/// readability of the internal code (i.e. for me) less readable, and thus harder to debug (although
/// this matters less because the mapping is fully generated of course).
///
///
/// # The mapping
///
/// ```text
/// releases -> release toolchains -> toolchains -> target sets -> triples
///                                              -> component sets -> components
/// ```
///
/// [1]: https://doc.rust-lang.org/cargo/reference/publishing.html#packaging-a-crate
pub struct Tables {
    pub triples: &'static [&'static str],
    pub components: &'static [&'static str],
    pub target_sets: &'static [&'static [u16]],
    pub component_sets: &'static [&'static [u16]],
    pub toolchains: &'static [(u16, u32, u32)],
    pub release_toolchains: &'static [&'static [u32]],
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_releases_core::Stable;

    static TABLES: Tables = Tables {
        triples: &["aarch64-apple-darwin", "wasm32-unknown-unknown"],
        components: &["cargo", "rustc"],
        target_sets: &[&[], &[0], &[0, 1]],
        component_sets: &[&[], &[0, 1]],
        toolchains: &[(0, 1, 1), (0, 2, 1)],
        release_toolchains: &[&[], &[0, 1]],
    };

    fn bundle() -> Bundle {
        Bundle::new(&TABLES)
    }

    #[test]
    fn a_release_without_toolchains() {
        let channel = Channel::Stable(Stable::new(1, 2, 3));

        let toolchains = bundle().toolchains(&channel, &Date::new(2016, 4, 12), 0);

        assert!(toolchains.is_empty());
    }

    #[test]
    fn a_toolchain_states_the_channel_and_release_date_of_its_release() {
        let channel = Channel::Stable(Stable::new(1, 2, 3));
        let date = Date::new(2016, 4, 12);

        let toolchains = bundle().toolchains(&channel, &date, 1);

        assert_eq!(toolchains.len(), 2);
        assert!(
            toolchains
                .iter()
                .all(|toolchain| toolchain.channel() == &channel
                    && toolchain.date() == Some(&date))
        );
    }

    #[test]
    fn a_toolchain_decodes_its_host_targets_and_components() {
        let channel = Channel::Stable(Stable::new(1, 2, 3));

        let toolchains = bundle().toolchains(&channel, &Date::new(2016, 4, 12), 1);
        let toolchain = &toolchains[1];

        assert_eq!(
            toolchain.host(),
            &Target::from_target_triple_or_unknown("aarch64-apple-darwin")
        );
        assert_eq!(
            toolchain
                .targets()
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<_>>(),
            ["aarch64-apple-darwin", "wasm32-unknown-unknown"]
        );
        assert_eq!(
            toolchain
                .components()
                .iter()
                .map(|c| c.name())
                .collect::<Vec<_>>(),
            ["cargo", "rustc"]
        );
    }

    #[test]
    fn toolchains_which_refer_to_the_same_set_share_it() {
        let channel = Channel::Stable(Stable::new(1, 2, 3));

        let toolchains = bundle().toolchains(&channel, &Date::new(2016, 4, 12), 1);

        assert!(std::ptr::eq(
            toolchains[0].components().as_slice(),
            toolchains[1].components().as_slice()
        ));
    }
}
