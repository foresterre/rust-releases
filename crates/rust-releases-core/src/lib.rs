//! Defines the core routines required to implement a [`Source`].
//!
//! Please, see the [`rust-releases`] for additional documentation on how this crate can be used.
//!
//! [`Source`]: crate::Source
//! [`rust-releases`]: https://docs.rs/rust-releases
#![allow(missing_docs)]
#![deny(clippy::all)]
#![deny(unsafe_code)]

/// Defines release channels, such as the stable, beta and nightly release channels.
mod channel;

pub use crate::channel::Channel;

pub use rust_release;

use rust_release::rust_toolchain::{channel::Beta, channel::Nightly, channel::Stable};
use rust_release::toolchain::ReleaseToolchain;
use rust_release::{rust_toolchain, RustRelease};
use std::collections::{BTreeMap, BTreeSet};

pub mod resolver;

pub struct StableReleases<C = ()> {
    releases: BTreeSet<RustRelease<Stable, C>>,
}

impl Default for StableReleases {
    fn default() -> Self {
        Self {
            releases: BTreeSet::default(),
        }
    }
}

impl<C> StableReleases<C> {
    pub fn merge<C2, F, C3>(self, other: StableReleases<C2>, resolver: F) -> StableReleases<C3>
    where
        F: Fn(&Stable, MergeCandidate<C>, MergeCandidate<C2>) -> MergeResult<C3>,
    {
        struct InternalValue {
            date: Option<rust_toolchain::Date>,
        }

        let mut result = StableReleases::default();

        let mut map: BTreeMap<Stable, MergeResult<C>> = self
            .releases
            .into_iter()
            .map(|r| {
                (
                    r.version,
                    MergeResult {
                        release_date: r.release_date,
                        toolchains: r.toolchains,
                        context: r.context,
                    },
                )
            })
            .collect();

        for other_release in other.releases {
            let version = other_release.version;
            let other_date = other_release.release_date;
            let other_toolchains = other_release.toolchains;
            let other_context = other_release.context;

            if let Some((self_date, self_toolchains, self_context)) = map.remove(&version) {
                let self_candidate = MergeCandidate {
                    release_date: Some(&self_date),
                    toolchains: Some(&self_toolchains),
                    context: Some(&self_context),
                };

                let other_candidate = MergeCandidate {
                    release_date: Some(&other_date),
                    toolchains: Some(&other_toolchains),
                    context: Some(&other_context),
                };

                // Let resolver handle all the merging
                let merge_result = resolver(&version, self_candidate, other_candidate);

                result.releases.insert(RustRelease {
                    version,
                    release_date: merge_result.release_date,
                    toolchains: merge_result.toolchains,
                    context: merge_result.context,
                });
            } else {
                // Only exists in other
                let self_candidate = MergeCandidate {
                    release_date: None,
                    toolchains: None,
                    context: None,
                };

                let other_candidate = MergeCandidate {
                    release_date: Some(&other_date),
                    toolchains: Some(&other_toolchains),
                    context: Some(&other_context),
                };

                let merge_result = resolver(&version, self_candidate, other_candidate);

                result.releases.insert(RustRelease {
                    version,
                    release_date: merge_result.release_date,
                    toolchains: merge_result.toolchains,
                    context: merge_result.context,
                });
            }
        }

        // Process remaining versions from self
        for (version, (self_date, self_toolchains, self_context)) in map {
            let self_candidate = MergeCandidate {
                release_date: Some(&self_date),
                toolchains: Some(&self_toolchains),
                context: Some(&self_context),
            };

            let other_candidate = MergeCandidate {
                release_date: None,
                toolchains: None,
                context: None,
            };

            let merge_result = resolver(&version, self_candidate, other_candidate);

            result.releases.insert(RustRelease {
                version,
                release_date: merge_result.release_date,
                toolchains: merge_result.toolchains,
                context: merge_result.context,
            });
        }

        result
    }

    pub fn is_empty(&self) -> bool {
        self.releases.is_empty()
    }
}

pub struct MergeCandidate<'a, C> {
    pub release_date: Option<&'a Option<rust_toolchain::Date>>,
    pub toolchains: Option<&'a Vec<ReleaseToolchain>>,
    pub context: Option<&'a C>,
}

pub struct MergeResult<C> {
    pub release_date: Option<rust_toolchain::Date>,
    pub toolchains: Vec<ReleaseToolchain>,
    pub context: C,
}

#[derive(Default)]
pub struct BetaReleases<C = ()> {
    releases: BTreeSet<RustRelease<Beta, C>>,
}

impl<C> BetaReleases<C> {
    pub fn is_empty(&self) -> bool {
        self.releases.is_empty()
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
}

#[derive(Default)]
pub struct RustReleases<Cs = (), Cb = (), Cn = ()> {
    stable: StableReleases<Cs>,
    beta: BetaReleases<Cb>,
    nightly: NightlyReleases<Cn>,
}

impl RustReleases {
    /// Iterate over the fetched stable releases.
    pub fn stable(&self) -> impl IntoIterator<Item = &RustRelease<Stable>> {
        self.stable.releases.iter()
    }

    /// TODO
    pub fn beta(&self) -> impl IntoIterator<Item = &RustRelease<Beta>> {
        self.beta.releases.iter()
    }

    /// TODO
    pub fn nightly(
        &self,
    ) -> impl IntoIterator<Item = &rust_release::RustRelease<rust_release::Nightly>> {
        self.nightly.releases.iter()
    }
}

// pub trait MergeMut {
//     /// TODO
//     ///
//     /// Probably <V: Into<rust_release::ReleaseVersion>> or similar, for now simpler variant
//     fn merge_mut<V: Into<rust_release::ReleaseVersion>>(&mut self, f: impl Fn(V));
// }

// impl<Cs, Cb, Cn> RustReleases<Cs, Cb, Cn> {
//     pub fn merge_mut(&mut self, other: RustReleases<Cs, Cb, Cn>, f: impl Fn(ReleaseVersion)) {
//         todo!()
//     }
// }

#[cfg(test)]
mod tests {
    use crate::resolver::prefer_self_resolver;
    use crate::{RustReleases, StableReleases};
    use rust_release::rust_toolchain::channel::Stable;
    use rust_release::rust_toolchain::RustVersion;
    use rust_release::RustRelease;

    #[test]
    fn empty_merge_is_empty() {
        let left = RustReleases::default();
        let right = RustReleases::default();

        // todo!

        assert!(left.stable.is_empty());
        assert!(left.beta.is_empty());
        assert!(left.nightly.is_empty());
    }

    #[test]
    fn base() {
        let rr = RustRelease::new(
            Stable {
                version: RustVersion::new(0, 0, 0),
            },
            None,
            [],
        );

        let left = StableReleases::default();
        let right = StableReleases::default();

        let out = left.merge(right, prefer_self_resolver);

        assert!(out.is_empty());
    }
}
