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
    /// Iterate over the fetched stable releases.
    pub fn stable(&self) -> impl IntoIterator<Item = &RustRelease<Stable>> {
        self.stable.iter()
    }

    /// TODO
    pub fn beta(&self) -> impl IntoIterator<Item = &RustRelease<Beta>> {
        self.beta.iter_releases()
    }

    /// TODO
    pub fn nightly(&self) -> impl IntoIterator<Item = &RustRelease<Nightly>> {
        self.nightly.iter_releases()
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
    use crate::merge::Merge;
    use crate::resolver::prefer_self;
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

        let out = left.merge_with(right, prefer_self);

        assert!(out.is_empty());
    }
}
