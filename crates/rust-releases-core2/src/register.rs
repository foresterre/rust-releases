use crate::Release;
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::iter;

/// A data structure consisting of the known Rust releases.
///
/// Whether a release is known, and how much information is known about a release,
/// depends on the source used to build up this information.
pub struct Register {
    platform_register: HashMap<rust_toolchain::Platform, PlatformRegister>,
}

impl Register {
    pub fn add_release(&mut self, release: Release) {
        let platform = release.toolchain().platform().clone();

        self.platform_register
            .entry(platform)
            .or_default()
            .add(release)
    }
}

impl Register {
    pub fn from_iter<I: IntoIterator<Item = (rust_toolchain::Platform, Release)>>(
        iterable: I,
    ) -> Self {
        let platform_register = iterable.into_iter().fold(
            HashMap::<rust_toolchain::Platform, PlatformRegister>::new(),
            |mut map, (platform, release)| {
                map.entry(platform).or_default().add(release);
                map
            },
        );

        Self { platform_register }
    }
}

impl Register {
    /// Get a subset of the register, where the subset contains just the releases of the given platform.
    pub fn platform(&self, id: &rust_toolchain::Platform) -> Option<&PlatformRegister> {
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
    pub fn by_date(&self, date: rust_toolchain::ReleaseDate) -> impl IntoIterator<Item = &Release> {
        self.platform_register
            .values()
            .map(|reg| {
                reg.ascending()
                    .into_iter()
                    .filter(|rel| rel.release_date() == date)
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

/// A data structure consisting of the known Rust releases for a specific platform.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlatformRegister {
    releases: BTreeSet<Release>,
}

impl PlatformRegister {
    pub fn from_iter<I: IntoIterator<Item = Release>>(iterable: I) -> Self {
        Self {
            releases: iterable.into_iter().collect(),
        }
    }

    /// Add a release to the register.
    pub fn add(&mut self, release: Release) {
        self.releases.insert(release);
    }
}

impl PlatformRegister {
    /// Find the least recently released Rust release for the current platform.
    ///
    /// Returns `None` if no release could be found.
    pub fn first(&self) -> Option<&Release> {
        self.releases.first()
    }

    /// Find the most recently released Rust release for the current platform.
    ///
    /// Returns `None` if no release could be found.
    pub fn last(&self) -> Option<&Release> {
        self.releases.last()
    }

    /// All releases of the given platform, in ascending order.
    pub fn ascending(&self) -> impl IntoIterator<Item = &Release> {
        self.releases.iter()
    }

    /// All releases of the given platform, in descending order.
    pub fn descending(&self) -> impl IntoIterator<Item = &Release> {
        self.releases.iter().rev()
    }

    /// Amount of releases held by this platform register.
    pub fn len(&self) -> usize {
        self.releases.len()
    }
}

#[cfg(test)]
mod tests_register {
    use crate::{Register, Release};
    use rust_toolchain::Channel;

    #[test]
    fn from_iter() {
        let platform = rust_toolchain::Platform::host();
        let date = rust_toolchain::ReleaseDate::new(2023, 1, 1);
        let version = rust_toolchain::RustVersion::new(1, 0, 0);

        let toolchain = rust_toolchain::Toolchain::new(
            Channel::Stable,
            date.clone(),
            platform.clone(),
            Some(version.clone()),
        );

        let releases = vec![
            (
                rust_toolchain::Platform::host(),
                Release::new(toolchain.clone(), vec![]),
            ),
            (
                rust_toolchain::Platform::host(),
                Release::new(toolchain, vec![]),
            ),
        ];

        let register = Register::from_iter(releases);
        assert_eq!(register.count_releases(), 2);
    }
}
