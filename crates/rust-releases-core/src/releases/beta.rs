use crate::merge::{Merge, MergeCandidate};
use crate::releases::StableReleases;
use rust_release::rust_toolchain::channel::Beta;
use rust_release::{ReleaseVersion, RustRelease};
use std::collections::{BTreeMap, BTreeSet};

pub struct BetaReleases<C = ()> {
    releases: BTreeSet<RustRelease<Beta, C>>,
}

impl<C> Default for BetaReleases<C> {
    fn default() -> Self {
        Self {
            releases: BTreeSet::default(),
        }
    }
}

impl<C> BetaReleases<C> {
    pub fn add(&mut self, release: RustRelease<Beta, C>) {
        self.releases.insert(release);
    }

    /// Merge two sets of stable releases.
    ///
    /// # Generic Context
    ///
    /// The generic parameters `C`, `C2` and `C3` can be used to attach arbitrary metadata,
    /// possibly relevant for merging, to a release.
    ///
    /// - `C` is the metadata of `self`
    /// - `C2` is the metadata of `other`
    /// - `C3` is the merged metadata.
    pub fn merge_with<C2, F, C3>(self, other: BetaReleases<C2>, resolver: F) -> BetaReleases<C3>
    where
        F: Fn(&Beta, MergeCandidate<C>, MergeCandidate<C2>) -> Merge<C3>,
    {
        let mut out = BetaReleases::<C3>::default();

        let mut map: BTreeMap<Beta, Merge<C>> = self
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

            if let Some(self_result) = map.remove(&version) {
                // Exists in both
                let lhs = Into::<MergeCandidate<C>>::into(&self_result);
                let rhs = Into::<MergeCandidate<C2>>::into(&other_release);

                Self::apply_merge(&mut out, version, lhs, rhs, &resolver);
            } else {
                // Only exists in other
                let lhs = MergeCandidate::default();
                let rhs = Into::<MergeCandidate<C2>>::into(&other_release);

                Self::apply_merge(&mut out, version, lhs, rhs, &resolver);
            }
        }

        // Process remaining versions from self
        for (version, candidate) in map {
            let lhs = Into::<MergeCandidate<C>>::into(&candidate);
            let rhs = MergeCandidate::default();

            Self::apply_merge(&mut out, &version, lhs, rhs, &resolver);
        }

        out
    }

    /// Returns true if there are no releases, and false otherwise.
    pub fn is_empty(&self) -> bool {
        self.releases.is_empty()
    }

    /// Iterate over the releases.
    pub fn iter_releases(&self) -> impl Iterator<Item = &RustRelease<Beta, C>> {
        self.releases.iter()
    }

    /// Merges two merge candidates with a matching version into a single merged
    /// Release.
    fn apply_merge<C2, C3, F>(
        out: &mut BetaReleases<C3>,
        version: &Beta,
        lhs: MergeCandidate<C>,
        rhs: MergeCandidate<C2>,
        resolver: F,
    ) where
        F: Fn(&Beta, MergeCandidate<C>, MergeCandidate<C2>) -> Merge<C3>,
    {
        let merged = resolver(&version, lhs, rhs);
        let merged_release: RustRelease<Beta, C3> = merged.to_version(version);

        out.releases.insert(merged_release);
    }
}
