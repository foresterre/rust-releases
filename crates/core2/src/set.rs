use crate::Distribution;
use std::collections::BTreeSet;

/// Defines how releases are to be compared and ordered within a release set.
mod compare;
#[cfg(test)]
mod tests;

/// The [`DistributionSet`] data structure defines a sorted set, which is sorted
/// naturally.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DistributionSet {
    releases: BTreeSet<compare::CompareRelease>,
}

impl DistributionSet {
    pub fn from_iter<I: IntoIterator<Item = Distribution>>(iterable: I) -> Self {
        Self {
            releases: iterable.into_iter().map(compare::CompareRelease).collect(),
        }
    }

    /// Add a release to the register.
    pub fn push(&mut self, release: Distribution) {
        self.releases.insert(compare::CompareRelease(release));
    }
}

impl DistributionSet {
    /// Find the least recently released Rust release for the current platform.
    ///
    /// Returns `None` if no release could be found.
    pub fn first(&self) -> Option<&Distribution> {
        self.releases.first().map(|c| &c.0)
    }

    /// Find the most recently released Rust release for the current platform.
    ///
    /// Returns `None` if no release could be found.
    pub fn last(&self) -> Option<&Distribution> {
        self.releases.last().map(|c| &c.0)
    }

    /// All releases of the given platform, in ascending order.
    pub fn ascending(&self) -> impl IntoIterator<Item = &Distribution> {
        self.releases.iter().map(|c| &c.0)
    }

    /// All releases of the given platform, in descending order.
    pub fn descending(&self) -> impl IntoIterator<Item = &Distribution> {
        self.releases.iter().rev().map(|c| &c.0)
    }

    /// Amount of releases held by the set.
    pub fn len(&self) -> usize {
        self.releases.len()
    }
}
