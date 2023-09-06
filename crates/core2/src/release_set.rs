use crate::Release;
use comparator::VersionComparator;
use std::collections::BTreeSet;

mod comparator;
#[cfg(test)]
mod tests;

/// A set data structure for Rust releases.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ReleaseSet {
    releases: BTreeSet<VersionComparator>,
}

impl ReleaseSet {
    pub fn from_iter<I: IntoIterator<Item = Release>>(iterable: I) -> Self {
        Self {
            releases: iterable.into_iter().map(VersionComparator).collect(),
        }
    }

    /// Add a release to the register.
    pub fn push(&mut self, release: Release) {
        self.releases.insert(VersionComparator(release));
    }
}

impl ReleaseSet {
    /// Find the least recently released Rust release for the current platform.
    ///
    /// Returns `None` if no release could be found.
    pub fn first(&self) -> Option<&Release> {
        self.releases.first().map(|c| &c.0)
    }

    /// Find the most recently released Rust release for the current platform.
    ///
    /// Returns `None` if no release could be found.
    pub fn last(&self) -> Option<&Release> {
        self.releases.last().map(|c| &c.0)
    }

    /// All releases of the given platform, in ascending order.
    pub fn ascending(&self) -> impl IntoIterator<Item = &Release> {
        self.releases.iter().map(|c| &c.0)
    }

    /// All releases of the given platform, in descending order.
    pub fn descending(&self) -> impl IntoIterator<Item = &Release> {
        self.releases.iter().rev().map(|c| &c.0)
    }

    /// Amount of releases held by this platform register.
    pub fn len(&self) -> usize {
        self.releases.len()
    }
}