use crate::{Release, ReleaseSet};
use std::collections::{BTreeSet, HashMap};

#[cfg(test)]
mod tests;

/// A data structure consisting of the known Rust releases.
///
/// Whether a release is known, and how much information is known about a release,
/// depends on the source used to build up this information.
#[derive(Clone, Debug)]
pub struct Register {
    platform_register: HashMap<rust_toolchain::Platform, ReleaseSet>,
}

impl Register {
    pub fn add_release(&mut self, release: Release) {
        let platform = release.toolchain().platform().clone();

        self.platform_register
            .entry(platform)
            .or_default()
            .push(release)
    }
}

impl Register {
    pub fn from_iter<I: IntoIterator<Item = (rust_toolchain::Platform, Release)>>(
        iterable: I,
    ) -> Self {
        let platform_register = iterable.into_iter().fold(
            HashMap::<rust_toolchain::Platform, ReleaseSet>::new(),
            |mut map, (platform, release)| {
                map.entry(platform).or_default().push(release);
                map
            },
        );

        Self { platform_register }
    }
}

impl Register {
    /// Get a subset of the register, where the subset contains just the releases of the given platform.
    pub fn platform(&self, id: &rust_toolchain::Platform) -> Option<&ReleaseSet> {
        self.platform_register.get(id)
    }

    /// List all releases, regardless of platform, ordered by least- to most recent.
    ///
    /// Returns `None` if the platform does not exist in the register.
    pub fn ascending(&self) -> impl IntoIterator<Item = &Release> {
        BTreeSet::from_iter(
            self.platform_register
                .values()
                .map(|reg| reg.ascending())
                .flatten(),
        )
    }

    /// List all releases, regardless of platform, ordered by most- to least recent.
    ///
    /// Returns `None` if the platform does not exist in the register.
    pub fn descending(&self) -> impl IntoIterator<Item = &Release> {
        BTreeSet::from_iter(
            self.platform_register
                .values()
                .map(|reg| reg.descending())
                .flatten(),
        )
    }

    /// List the releases, published on the given date.
    pub fn by_date(
        &self,
        date: &rust_toolchain::ReleaseDate,
    ) -> impl IntoIterator<Item = &Release> {
        self.platform_register
            .values()
            .map(|reg| {
                reg.ascending()
                    .into_iter()
                    .filter(|rel| rel.toolchain().release_date() == date)
            })
            .fold(BTreeSet::new(), |mut acc: BTreeSet<&Release>, next| {
                acc.extend(next);
                acc
            })
    }

    /// The amount of releases inked into this register, regardless of the platform.
    pub fn count_releases(&self) -> usize {
        self.platform_register.values().map(|reg| reg.len()).sum()
    }
}
