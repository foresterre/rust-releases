use crate::merge::{Merge, MergeCandidate};
use rust_release::rust_toolchain::channel::{Beta, Nightly, Stable};
use rust_release::{ReleaseVersion, RustRelease};
use std::collections::{BTreeMap, BTreeSet};

pub struct StableReleases<C = ()> {
    releases: BTreeSet<RustRelease<Stable, C>>,
}

impl<C> Default for StableReleases<C> {
    fn default() -> Self {
        Self {
            releases: BTreeSet::default(),
        }
    }
}

impl<C> StableReleases<C> {
    pub fn add(&mut self, release: RustRelease<Stable, C>) {
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
    pub fn merge_with<C2, F, C3>(self, other: StableReleases<C2>, resolver: F) -> StableReleases<C3>
    where
        F: Fn(&Stable, MergeCandidate<C>, MergeCandidate<C2>) -> Merge<C3>,
    {
        let mut out = StableReleases::<C3>::default();

        let mut map: BTreeMap<Stable, Merge<C>> = self
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
    pub fn iter_releases(&self) -> impl Iterator<Item = &RustRelease<Stable, C>> {
        self.releases.iter()
    }

    /// Merges two merge candidates with a matching version into a single merged
    /// Release.
    fn apply_merge<C2, C3, F>(
        out: &mut StableReleases<C3>,
        version: &Stable,
        lhs: MergeCandidate<C>,
        rhs: MergeCandidate<C2>,
        resolver: F,
    ) where
        F: Fn(&Stable, MergeCandidate<C>, MergeCandidate<C2>) -> Merge<C3>,
    {
        let merged = resolver(&version, lhs, rhs);
        let merged_release: RustRelease<Stable, C3> = merged.to_version(version);

        out.releases.insert(merged_release);
    }
}

#[derive(Default)]
pub struct BetaReleases<C = ()> {
    releases: BTreeSet<RustRelease<Beta, C>>,
}

impl<C> BetaReleases<C> {
    pub fn is_empty(&self) -> bool {
        self.releases.is_empty()
    }

    pub fn iter_releases(&self) -> impl Iterator<Item = &RustRelease<Beta, C>> {
        self.releases.iter()
    }
}

#[derive(Default)]
pub struct NightlyReleases<C = ()> {
    releases: BTreeSet<RustRelease<Nightly, C>>,
}

impl<C> NightlyReleases<C> {
    pub fn is_empty(&self) -> bool {
        self.releases.is_empty()
    }

    pub fn iter_releases(&self) -> impl Iterator<Item = &RustRelease<Nightly, C>> {
        self.releases.iter()
    }
}

struct ReleasesImpl<V, C = ()> {
    releases: BTreeSet<RustRelease<V, C>>,
}

impl<C> Default for ReleasesImpl<C> {
    fn default() -> Self {
        Self {
            releases: BTreeSet::default(),
        }
    }
}

impl<V: Clone + Ord, C> ReleasesImpl<V, C> {
    pub fn add(&mut self, release: RustRelease<V, C>) {
        self.releases.insert(release);
    }

    /// Merge two sets of stable releases.
    ///
    /// # Generic Context
    ///
    /// The generic parameters `C`, `C2` and `C3` can be used to attach arbitrary metadata,
    /// possibly relevant for merging, to a release.
    ///
    /// - `C` is the attached data of `self`
    /// - `C2` is the attached data of `rhs`
    /// - `C3` is the data merged from `C` and `C2`.
    pub fn merge_with<C2, F, C3>(
        &self,
        other: &ReleasesImpl<C2>,
        resolver: F,
    ) -> ReleasesImpl<V, C3>
    where
        F: Fn(&V, MergeCandidate<C>, MergeCandidate<C2>) -> RustRelease<V, C3>,
    {
        // TODO
        //      1. making it generic to be used by StableReleases<C>, BetaReleases<C> and NightlyReleases<C>
        //      2. I changed F: -> Merge<C3> to RustRelease<V, C3> and the self.releases.'into btree map' to simply use its own internal BTreeSet
        //      3. also experimenting with whether to use self, mut self, &self as the receiver type; with consideration that since we create a new merged (i.e. mutated) version anyways and are would move all versions by removing them from self, there is likely no need to take ownership of self.

        let mut out = ReleasesImpl::<C3>::default();

        for rhs in other.releases() {
            let version = rhs.version();
            let lhs = self.releases.remove(version);

            if let Some(self_result) = lhs {
                // Exists in both
                let lhs = Into::<MergeCandidate<C>>::into(&self_result);
                let rhs = Into::<MergeCandidate<C2>>::into(&rhs);

                Self::apply_merge(&mut out, version, lhs, rhs, &resolver);
            } else {
                // Only exists in other
                let lhs = MergeCandidate::default();
                let rhs = Into::<MergeCandidate<C2>>::into(&rhs);

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
    pub fn iter_releases(&self) -> impl Iterator<Item = &RustRelease<V, C>> {
        self.releases.iter()
    }

    /// Merges two merge candidates with a matching version into a single merged
    /// Release.
    fn apply_merge<C2, C3, F>(
        out: &mut ReleasesImpl<C3>,
        version: &V,
        lhs: MergeCandidate<C>,
        rhs: MergeCandidate<C2>,
        resolver: F,
    ) where
        F: Fn(&V, MergeCandidate<C>, MergeCandidate<C2>) -> Merge<C3>,
    {
        let merged = resolver(version, lhs, rhs);
        let merged_release: RustRelease<V, C3> = merged.to_version(version);

        out.releases.insert(merged_release);
    }
}

impl<V, C> ReleasesImpl<V, C> {
    pub fn releases(&self) -> &BTreeSet<RustRelease<V, C>> {
        &self.releases
    }
}
