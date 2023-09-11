use crate::{Distribution, DistributionSet};
use rust_toolchain::Channel;
use std::collections::HashMap;
use std::iter;

/// The default implementation for a `Register`.
#[derive(Clone, Debug)]
pub struct PlatformRegister {
    register: HashMap<rust_toolchain::Platform, DistributionSet>,
}

impl PlatformRegister {
    /// Instantiate a register from a iterable (Platform, Release) tuple
    pub fn from_iter<I: IntoIterator<Item = (rust_toolchain::Platform, Distribution)>>(
        iterable: I,
    ) -> Self {
        let register = iterable.into_iter().fold(
            HashMap::<rust_toolchain::Platform, DistributionSet>::new(),
            |mut map, (platform, release)| {
                map.entry(platform).or_default().push(release);
                map
            },
        );

        Self { register }
    }

    /// Add a release to the register.
    pub fn add_distribution(&mut self, release: Distribution) {
        let platform = release.toolchain().platform().clone();

        self.register.entry(platform).or_default().push(release)
    }

    /// Get the releases for a given set of releases.
    pub fn platform(&self, id: &rust_toolchain::Platform) -> Option<&DistributionSet> {
        self.register.get(id)
    }

    /// Get the amount of releases held by the register.
    pub fn size(&self) -> usize {
        self.register.values().map(|reg| reg.len()).sum()
    }

    pub fn distributions_by_channel(
        &self,
        channel: &Channel,
    ) -> impl Iterator<Item = &Distribution> {
        // self.register.iter().filter_map(|(platform, set)| {
        //     set.ascending()
        //         .filter(|dist| dist.toolchain().channel() == channel)
        //         .next()
        // })

        iter::once(todo!())
    }

    /// List all releases, regardless of platform, ordered by least- to most recent.
    pub fn ascending(&self) -> impl Iterator<Item = &Distribution> {
        //BTreeSet::from_iter(self.register.values().map(|reg| reg.ascending()).flatten())
        iter::once(todo!())
    }

    /// List all releases, regardless of platform, ordered by most- to least recent.
    pub fn descending(&self) -> impl IntoIterator<Item = &Distribution> {
        // BTreeSet::from_iter(self.register.values().map(|reg| reg.descending()).flatten())
        iter::once(todo!())
    }

    /// List the releases, published on the given date.
    pub fn by_date(
        &self,
        date: &rust_toolchain::ReleaseDate,
    ) -> impl IntoIterator<Item = &Distribution> {
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
