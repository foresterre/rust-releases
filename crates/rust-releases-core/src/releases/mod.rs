use crate::merge::{Merge, MergeCandidate};
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

    pub struct ReleasesImpl<V, C = ()> {
        releases: BTreeSet<RustRelease<V, C>>,
    }

    impl<V, C> Default for ReleasesImpl<V, C> {
        fn default() -> Self {
            Self {
                releases: BTreeSet::default(),
            }
        }
    }

    impl<V, C> ReleasesImpl<V, C>
    where
        V: Clone + Ord,
    {
        /// Merge two sets of releases.
        ///
        /// # Generic Context
        ///
        /// The generic parameters `C`, `C2` and `C3` can be used to attach arbitrary metadata,
        /// possibly relevant for merging, to a release.
        ///
        /// - `C` is the metadata of `self`
        /// - `C2` is the metadata of `other`
        /// - `C3` is the merged metadata.
        pub fn merge_with<C2, F, C3>(
            self,
            other: ReleasesImpl<V, C2>,
            resolver: F,
        ) -> ReleasesImpl<V, C3>
        where
            F: Fn(&V, MergeCandidate<C>, MergeCandidate<C2>) -> Merge<C3>,
        {
            let mut out = ReleasesImpl::<V, C3>::default();

            let mut map: BTreeMap<V, Merge<C>> = self
                .releases
                .into_iter()
                .map(|r| {
                    (
                        r.version,
                        Merge {
                            release_date: r.release_date,
                            toolchains: r.toolchains,
                            context: r.context,
                        },
                    )
                })
                .collect();

            for other_release in other.releases {
                let version = &other_release.version;

                if let Some(self_result) = map.remove(version) {
                    // Exists in both
                    let lhs = MergeCandidate::from(&self_result);
                    let rhs = MergeCandidate::from(&other_release);

                    Self::apply_merge(&mut out, version, lhs, rhs, &resolver);
                } else {
                    // Only exists in other
                    let lhs = MergeCandidate::default();
                    let rhs = MergeCandidate::from(&other_release);

                    Self::apply_merge(&mut out, version, lhs, rhs, &resolver);
                }
            }

            // Process remaining versions from self
            for (version, candidate) in map {
                let lhs = MergeCandidate::from(&candidate);
                let rhs = MergeCandidate::default();

                Self::apply_merge(&mut out, &version, lhs, rhs, &resolver);
            }

            out
        }

        /// Merges two merge candidates with a matching version into a single merged Release.
        fn apply_merge<C2, C3, F>(
            out: &mut ReleasesImpl<V, C3>,
            version: &V,
            lhs: MergeCandidate<C>,
            rhs: MergeCandidate<C2>,
            resolver: &F,
        ) where
            F: Fn(&V, MergeCandidate<C>, MergeCandidate<C2>) -> Merge<C3>,
        {
            let merged = resolver(version, lhs, rhs);
            let merged_release = RustRelease {
                version: version.clone(),
                release_date: merged.release_date,
                toolchains: merged.toolchains,
                context: merged.context,
            };

            out.releases.insert(merged_release);
        }
    }

    impl<V, C> ReleasesImpl<V, C>
    where
        V: Ord,
    {
        /// Add a release to the collection
        pub fn add(&mut self, release: RustRelease<V, C>) {
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
        pub fn iter(&self) -> impl Iterator<Item = &RustRelease<V, C>> {
            self.releases.iter()
        }
    }
}
