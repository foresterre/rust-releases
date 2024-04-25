use crate::comparable_distribution::ComparableDistribution;
use crate::{Distribution, DistributionSet};
use std::collections::{btree_set, BTreeSet, HashMap};
use std::iter;
use std::marker::PhantomData;

pub struct DistributionSetK {
    items: BTreeSet<ComparableDistribution>,
}

impl FromIterator<Distribution> for DistributionSetK {
    fn from_iter<T: IntoIterator<Item = Distribution>>(iter: T) -> Self {
        Self {
            items: iter.into_iter().map(ComparableDistribution).collect(),
        }
    }
}

pub type DistributionByPlatformMap = HashMap<rust_toolchain::Target, DistributionSet>;

pub type DistributionByVersionMap = HashMap<rust_toolchain::Target, DistributionSet>;

impl DistributionSetK {
    // advantage: set with all items -> sorted by one is a lot easier than sorted by one element to sorted by another
    pub fn as_platform_map(&self) -> DistributionByPlatformMap {
        // clones
        todo!()
    }

    pub fn as_version_map(&self) -> DistributionByVersionMap {
        // clones
        todo!()
    }

    pub fn into_platform_map(self) -> DistributionByPlatformMap {
        // moves
        todo!()
    }

    pub fn into_by_version_map(self) -> DistributionByVersionMap {
        // moves
        todo!()
    }
}

impl IntoIterator for DistributionSetK {
    type Item = Distribution;
    type IntoIter = DistributionsIterator;

    fn into_iter(self) -> Self::IntoIter {
        DistributionsIterator {
            distributions: self.items.into_iter(),
        }
    }
}

pub struct DistributionsIterator {
    distributions: btree_set::IntoIter<ComparableDistribution>,
}

impl Iterator for DistributionsIterator {
    type Item = Distribution;

    fn next(&mut self) -> Option<Self::Item> {
        self.distributions.next().map(|c| c.0)
    }
}
