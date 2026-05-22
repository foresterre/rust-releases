//! Defines the core routines required to implement a [`Source`].
//!
//! Please, see the [`rust-releases`] for additional documentation on how this crate can be used.
//!
//! [`Source`]: crate::Source
//! [`rust-releases`]: https://docs.rs/rust-releases
#![allow(missing_docs)]
#![deny(clippy::all)]
#![deny(unsafe_code)]

pub use crate::releases::{BetaReleases, NightlyReleases, StableReleases};
/// Defines release channels, such as the stable, beta and nightly release channels.
pub use rust_release::{self, Beta, Nightly, RustRelease, Stable};

pub mod channel;
pub mod merge;
pub mod releases;

#[derive(Debug, Default)]
pub struct RustReleases {
    stable: StableReleases,
    beta: BetaReleases,
    nightly: NightlyReleases,
}

impl RustReleases {
    /// Iterate over set of stable releases
    pub fn stable(&self) -> impl IntoIterator<Item = &RustRelease<Stable>> {
        self.stable.iter()
    }

    /// Iterate over set of beta releases
    pub fn beta(&self) -> impl IntoIterator<Item = &RustRelease<Beta>> {
        self.beta.iter()
    }

    /// Iterate over set of nightly releases
    pub fn nightly(&self) -> impl IntoIterator<Item = &RustRelease<Nightly>> {
        self.nightly.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::merge;

    #[test]
    fn empty_merge_is_empty() {
        let mut out = StableReleases::<()>::default();

        let left: Vec<RustRelease<Stable>> = vec![];
        let right: Vec<RustRelease<Stable>> = vec![];
        for (l, r) in left.into_iter().zip(right) {
            out.add(merge::merge_default(l, r));
        }

        assert!(out.is_empty());
    }

    #[test]
    fn base() {
        let left = RustRelease::new(Stable::new(1, 2, 0), None, []);
        let right = RustRelease::new(Stable::new(1, 2, 0), None, []);

        let mut out = StableReleases::default();
        out.add(merge::merge_default(left, right));

        assert_eq!(out.len(), 1);

        let first = out.iter().next().unwrap();

        assert_eq!(first.version(), &Stable::new(1, 2, 0));
        assert_eq!(first.release_date(), None);
        assert_eq!(first.toolchains_iter().count(), 0);
    }
}
