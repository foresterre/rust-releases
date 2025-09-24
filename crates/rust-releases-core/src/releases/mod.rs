use crate::merge::PartialRustRelease;
use rust_release::RustRelease;
use std::collections::{BTreeMap, BTreeSet};

mod beta;
mod nightly;
mod stable;

pub use beta::BetaReleases;
pub use nightly::NightlyReleases;
pub use stable::StableReleases;

// shared implementation for StableReleases, BetaReleases and NightlyReleases (implementation detail)
pub(in crate::releases) mod impls {
    use super::*;
    use crate::merge::Merge;

    #[derive(Debug)]
    pub struct ReleasesImpl<V> {
        releases: BTreeSet<RustRelease<V>>,
    }

    impl<V> Default for ReleasesImpl<V> {
        fn default() -> Self {
            Self {
                releases: BTreeSet::default(),
            }
        }
    }

    impl<V: Clone + Ord> Merge for ReleasesImpl<V> {
        type Channel = V;

        /// Merge two sets of releases.
        ///
        // # Generic Context `TODO(foresterre): removed from current version`
        //
        // The generic parameters `C`, `C2` and `C3` can be used to attach arbitrary metadata,
        // possibly relevant for merging, to a release.
        //
        // - `C` is the metadata of `self`
        // - `C2` is the metadata of `other`
        // - `C3` is the merged metadata.
        fn merge_with<F>(self, other: Self, resolver: F) -> Self
        where
            F: Fn(
                Self::Channel,
                PartialRustRelease,
                PartialRustRelease,
            ) -> RustRelease<Self::Channel>,
        {
            let mut out = ReleasesImpl::<V>::default();

            let mut map: BTreeMap<V, PartialRustRelease> = self
                .releases
                .into_iter()
                .map(|r| {
                    (
                        r.version,
                        PartialRustRelease {
                            release_date: r.release_date,
                            toolchains: Some(r.toolchains),
                        },
                    )
                })
                .collect();

            for rhs in other.releases {
                let version = rhs.version.clone();

                if let Some(lhs) = map.remove(&version) {
                    // Exists in both
                    let rhs = PartialRustRelease::from(rhs);

                    Self::apply_merge(&mut out, version, lhs, rhs, &resolver);
                } else {
                    // Only exists in other
                    let lhs = PartialRustRelease::default();
                    let rhs = PartialRustRelease::from(rhs);

                    Self::apply_merge(&mut out, version, lhs, rhs, &resolver);
                }
            }

            // Process remaining versions from self
            for (version, lhs) in map {
                let rhs = PartialRustRelease::default();

                Self::apply_merge(&mut out, version, lhs, rhs, &resolver);
            }

            out
        }
    }

    impl<V> ReleasesImpl<V>
    where
        V: Clone + Ord,
    {
        /// Merges two merge candidates with a matching version into a single merged Release.
        fn apply_merge<F>(
            out: &mut ReleasesImpl<V>,
            version: V,
            lhs: PartialRustRelease,
            rhs: PartialRustRelease,
            resolver: &F,
        ) where
            F: Fn(V, PartialRustRelease, PartialRustRelease) -> RustRelease<V>,
        {
            let merged = resolver(version.clone(), lhs, rhs);
            let merged_release = RustRelease {
                version,
                release_date: merged.release_date,
                toolchains: merged.toolchains,
            };

            out.releases.insert(merged_release);
        }
    }

    impl<V> ReleasesImpl<V>
    where
        V: Ord,
    {
        /// Add a release to the collection
        pub fn add(&mut self, release: RustRelease<V>) {
            self.releases.insert(release);
        }

        /// Returns the amount of releases
        pub fn len(&self) -> usize {
            self.releases.len()
        }

        /// Returns true if there are no releases, and false otherwise.
        pub fn is_empty(&self) -> bool {
            self.releases.is_empty()
        }

        /// Iterate over the releases.
        pub fn iter(&self) -> impl Iterator<Item = &RustRelease<V>> {
            self.releases.iter()
        }
    }
}
