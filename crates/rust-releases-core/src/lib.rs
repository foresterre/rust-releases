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
pub use rust_release;

use crate::releases::{BetaReleases, NightlyReleases, StableReleases};
use rust_release::RustRelease;

pub use rust_release::rust_toolchain::{channel::Beta, channel::Nightly, channel::Stable};

pub mod channel;
pub mod merge;
pub mod releases;
pub mod resolver;

pub struct RustReleases<Cs = (), Cb = (), Cn = ()> {
    stable: StableReleases<Cs>,
    beta: BetaReleases<Cb>,
    nightly: NightlyReleases<Cn>,
}

impl<Cs: Default, Cb: Default, Cn: Default> Default for RustReleases<Cs, Cb, Cn> {
    fn default() -> Self {
        Self {
            stable: StableReleases::default(),
            beta: BetaReleases::default(),
            nightly: NightlyReleases::default(),
        }
    }
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
    use crate::merge::Merge;
    use crate::resolver::combine;
    use crate::StableReleases;
    use rust_release::RustRelease;
    use rust_release::Stable;

    #[test]
    fn empty_merge_is_empty() {
        let left = StableReleases::<()>::default();
        let right = StableReleases::<()>::default();

        let merge = left.merge_with(right, |_version, _lhs, _rhs| Merge {
            release_date: None,
            toolchains: vec![],
            context: (),
        });

        assert!(merge.is_empty());
        assert!(merge.is_empty());
        assert!(merge.is_empty());
    }

    #[test]
    fn base() {
        let mut left = StableReleases::default();
        left.add(RustRelease::new(Stable::new(1, 2, 0), None, []));
        let right = StableReleases::<()>::default();

        let out = left.merge_with(right, combine);

        assert_eq!(out.len(), 1);

        let first = out.iter().next().unwrap();

        assert_eq!(first.version(), &Stable::new(1, 2, 0));
        assert_eq!(first.release_date(), None);
        assert_eq!(first.toolchains.len(), 0);
        assert_eq!(first.context, ());
    }
}
