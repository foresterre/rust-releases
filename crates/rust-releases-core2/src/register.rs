use crate::Release;
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::iter;

/// A data structure consisting of the known Rust releases.
///
/// Whether a release is known, and how much information is known about a release,
/// depends on the source used to build up this information.
pub struct Register {
    releases: HashMap<rust_toolchain::Platform, PlatformRegister>,
}

impl Register {
    pub fn add_release(&mut self, release: Release) {
        let platform = release.toolchain().platform().clone();

        self.releases.entry(platform).or_default().add(release)
    }
}

impl Register {
    pub fn find(&self) -> Option<&Release> {
        todo!()
    }

    // /// Least recent to most recent
    // pub fn all_ascending(&self) -> impl Iterator<Item = Release> {
    //     todo!();
    // }
    //
    // /// Most recent to least recent
    // pub fn all_descending(&self) -> impl Iterator<Item = Release> {
    //     todo!();
    // }

    pub fn last(&self) -> &Release {
        todo!();
    }

    pub fn from_iter<'i, I: Iterator<Item = &'i (rust_toolchain::Platform, Release)>>(
        iter: I,
    ) -> Self {
        todo!()
    }

    /// Find the release distributions for the given platform
    pub fn find_by_platform(
        &self,
        platform: rust_toolchain::Platform,
    ) -> impl IntoIterator<Item = Release> {
        iter::once(todo!())
    }

    /// Find the release distribution(s) which where published on the given date.
    pub fn find_by_date(&self, date: rust_toolchain::ReleaseDate) -> impl Iterator<Item = Release> {
        iter::once(todo!())
    }
}

/// A data structure consisting of the known Rust releases for a specific platform.
#[derive(Clone, Debug, Default, PartialEq)]
struct PlatformRegister {
    releases: BTreeSet<Release>,
}

impl PlatformRegister {
    pub fn add(&mut self, release: Release) {
        self.releases.insert(release);
    }

    /// Find the most recently released Rust release for the current platform.
    ///
    /// Returns `None` if no release could be found.
    pub fn most_recent(&self) -> Option<&Release> {
        self.releases.last()
    }

    /// Find the least recently released Rust release for the current platform.
    ///
    /// Returns `None` if no release could be found.
    pub fn least_recent(&self) -> Option<&Release> {
        self.releases.first()
    }
}
