use crate::{Release, ReleaseSet};
use std::collections::HashMap;
use std::iter;

/// The default implementation for a `Register`.
#[derive(Clone, Debug)]
pub struct PlatformRegister {
    register: HashMap<rust_toolchain::Platform, ReleaseSet>,
}

impl PlatformRegister {
    /// Instantiate a register from a iterable (Platform, Release) tuple
    pub fn from_iter<I: IntoIterator<Item = (rust_toolchain::Platform, Release)>>(
        iterable: I,
    ) -> Self {
        let register = iterable.into_iter().fold(
            HashMap::<rust_toolchain::Platform, ReleaseSet>::new(),
            |mut map, (platform, release)| {
                map.entry(platform).or_default().push(release);
                map
            },
        );

        Self { register }
    }

    /// Add a release to the register.
    pub fn add_release(&mut self, release: Release) {
        let platform = release.toolchain().platform().clone();

        self.register.entry(platform).or_default().push(release)
    }

    /// Get the releases for a given set of releases.
    pub fn platform(&self, id: &rust_toolchain::Platform) -> Option<&ReleaseSet> {
        self.register.get(id)
    }

    /// Get the amount of releases held by the register.
    pub fn size(&self) -> usize {
        self.register.values().map(|reg| reg.len()).sum()
    }

    /// List all releases, regardless of platform, ordered by least- to most recent.
    pub fn ascending(&self) -> impl IntoIterator<Item = &Release> {
        //BTreeSet::from_iter(self.register.values().map(|reg| reg.ascending()).flatten())
        iter::once(todo!())
    }

    /// List all releases, regardless of platform, ordered by most- to least recent.
    pub fn descending(&self) -> impl IntoIterator<Item = &Release> {
        // BTreeSet::from_iter(self.register.values().map(|reg| reg.descending()).flatten())
        iter::once(todo!())
    }

    /// List the releases, published on the given date.
    pub fn by_date(
        &self,
        date: &rust_toolchain::ReleaseDate,
    ) -> impl IntoIterator<Item = &Release> {
        // self.platform_register
        //     .values()
        //     .map(|reg| reg.ascending().into_iter().filter(|rel| rel.date() == date))
        //     .fold(BTreeSet::new(), |mut acc: BTreeSet<&Release>, next| {
        //         acc.extend(next);
        //         acc
        //     })

        iter::once(todo!())
    }
}
