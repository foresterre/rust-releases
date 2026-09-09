use rust_releases_core::{
    Beta, BetaReleases, Nightly, NightlyReleases, RustRelease, Stable, StableReleases,
};
use std::fmt::Debug;

pub trait ReleaseCollection: Sized + IntoIterator<Item = RustRelease<Self::Version>> {
    type Version: Debug;

    fn empty() -> Self;

    fn add(&mut self, release: RustRelease<Self::Version>);

    fn releases(&self) -> impl Iterator<Item = &RustRelease<Self::Version>>;
}

impl ReleaseCollection for StableReleases {
    type Version = Stable;

    fn empty() -> Self {
        StableReleases::empty()
    }

    fn add(&mut self, release: RustRelease<Stable>) {
        StableReleases::add(self, release);
    }

    fn releases(&self) -> impl Iterator<Item = &RustRelease<Stable>> {
        self.iter()
    }
}

impl ReleaseCollection for BetaReleases {
    type Version = Beta;

    fn empty() -> Self {
        BetaReleases::empty()
    }

    fn add(&mut self, release: RustRelease<Beta>) {
        BetaReleases::add(self, release);
    }

    fn releases(&self) -> impl Iterator<Item = &RustRelease<Beta>> {
        self.iter()
    }
}

impl ReleaseCollection for NightlyReleases {
    type Version = Nightly;

    fn empty() -> Self {
        NightlyReleases::empty()
    }

    fn add(&mut self, release: RustRelease<Nightly>) {
        NightlyReleases::add(self, release);
    }

    fn releases(&self) -> impl Iterator<Item = &RustRelease<Nightly>> {
        self.iter()
    }
}
