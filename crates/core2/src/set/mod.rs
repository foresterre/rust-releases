//! An ordered set for Rust [`distributions`].
//!
//! [`distributions`]: Distribution

use crate::comparable_distribution::ComparableDistribution;
use crate::Distribution;
use std::collections::BTreeSet;

/// Defines how releases are to be compared and ordered within a release set.
#[cfg(test)]
mod tests;

/// The [`DistributionSet`] data structure defines a naturally sorted set, consisting of
/// Rust [`distributions`].
///
/// The ordering follows the following rules, in order of importance:
///
/// 1. `stable > beta > nightly`.
/// 2. When comparing any releases of the same channel, `a` and `b`, and `version(a) * version(b)`, then `a * b`.
/// 3. When comparing two releases of the same channel, `a` and `b`, and `date(a) * date(b)`, then `a * b`.
///
/// NB: `*` stands for the operations [`<`] and [`>`].
///
/// [`distributions`]: Distribution
/// [`<`]: PartialOrd::lt
/// [`>`]: PartialOrd::gt
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DistributionSet<R: Ord> {
    releases: BTreeSet<R>,
}

impl<R> DistributionSet<R> {
    /// Create a new set from anything that can be turned into an owned iterator over
    /// [`Distribution`].
    ///
    /// # Example
    ///
    /// ```
    /// # use std::iter;
    /// # use rust_releases_core2::{Distribution, DistributionSet};
    /// # use rust_toolchain::{Channel, Target, ReleaseDate, RustVersion, Toolchain};
    /// #
    /// let date = ReleaseDate::new(2023, 1, 1);
    /// let version = RustVersion::new(1, 31, 0);
    ///
    /// let toolchain = Toolchain::new(Channel::stable(version), Target::host());
    /// let distribution = Distribution::new_without_components(date, toolchain);
    ///
    /// let set = DistributionSet::from_iter(iter::once(distribution));
    ///
    /// assert_eq!(set.len(), 1);
    /// ```
    pub fn from_iter<I: IntoIterator<Item = Distribution>>(iterable: I) -> Self {
        Self {
            releases: iterable.into_iter().map(ComparableDistribution).collect(),
        }
    }

    /// Add a release to the register.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::iter;
    /// # use rust_releases_core2::{Distribution, DistributionSet};
    /// # use rust_toolchain::{Channel, Target, ReleaseDate, RustVersion, Toolchain};
    /// #
    /// # let date = ReleaseDate::new(2023, 1, 1);
    /// # let toolchain = Toolchain::new(Channel::stable(RustVersion::new(1, 31, 0)), Target::host());
    /// # let distribution = Distribution::new_without_components(date.clone(), toolchain);
    /// #
    /// # let different_toolchain = Toolchain::new(Channel::stable(RustVersion::new(1, 32, 0)), Target::host());
    /// # let different_distribution = Distribution::new_without_components(date.clone(), different_toolchain);
    /// #
    /// let mut set = DistributionSet::from_iter(iter::once(distribution.clone()));
    /// assert_eq!(set.len(), 1);
    ///
    /// // Insert the same exact Rust distribution.
    /// set.push(distribution);
    ///
    /// // It won't be added to the set, since the set already contains an equal element.
    /// assert_eq!(set.len(), 1);
    ///
    /// // Add a different Rust distribution.
    /// set.push(different_distribution);
    ///
    /// assert_eq!(set.len(), 2);
    /// ```
    pub fn push(&mut self, release: Distribution) {
        self.releases.insert(ComparableDistribution(release));
    }
}

impl<R: Ord> DistributionSet<R> {
    /// Find the least recently released Rust release for the current platform.
    ///
    /// Returns `None` if no release could be found.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::iter;
    /// # use rust_releases_core2::{Distribution, DistributionSet};
    /// # use rust_toolchain::{Channel, Target, ReleaseDate, RustVersion, Toolchain};
    /// #
    /// # let date = ReleaseDate::new(2023, 1, 1);
    /// # let toolchain31 = Toolchain::new(Channel::stable(RustVersion::new(1, 31, 0)), Target::host());
    /// # let toolchain32 = Toolchain::new(Channel::stable(RustVersion::new(1, 32, 0)), Target::host());
    /// # let distribution31 = Distribution::new_without_components(date.clone(), toolchain31);
    /// # let distribution32 = Distribution::new_without_components(date.clone(), toolchain32);
    /// #
    /// // A set of with stable releases 1.31.0 and 1.32.0
    /// let set = DistributionSet::from_iter(vec![distribution32, distribution31]);
    ///
    /// // Get the first element in the set, which is the least recent stable release in this case!
    /// let element = set.first().unwrap();
    ///
    /// let version = element.toolchain().version().unwrap();
    /// assert_eq!(version, &RustVersion::new(1, 31, 0));
    /// assert!(element.is_stable());
    ///
    /// // Check that the empty set will indeed return `None`
    /// let empty_set = DistributionSet::from_iter(iter::empty());
    /// assert!(empty_set.last().is_none());
    /// ```
    pub fn first(&self) -> Option<&Distribution> {
        self.releases.first().map(|c| &c.0)
    }

    /// Find the most recently released Rust release for the current platform.
    ///
    /// Returns `None` if no release could be found.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::iter;
    /// # use rust_releases_core2::{Distribution, DistributionSet};
    /// # use rust_toolchain::{Channel, Target, ReleaseDate, RustVersion, Toolchain};
    /// #
    /// # let date = ReleaseDate::new(2023, 1, 1);
    /// # let toolchain31 = Toolchain::new(Channel::stable(RustVersion::new(1, 31, 0)), Target::host());
    /// # let toolchain32 = Toolchain::new(Channel::stable(RustVersion::new(1, 32, 0)), Target::host());
    /// # let distribution31 = Distribution::new_without_components(date.clone(), toolchain31);
    /// # let distribution32 = Distribution::new_without_components(date.clone(), toolchain32);
    /// #
    /// // A set of with stable releases 1.31.0 and 1.32.0
    /// let set = DistributionSet::from_iter(vec![distribution32, distribution31]);
    ///
    /// // Get the last element in the set, which is the most recent stable release in this case!
    /// let element = set.last().unwrap();
    ///
    /// let version = element.toolchain().version().unwrap();
    /// assert_eq!(version, &RustVersion::new(1, 32, 0));
    /// assert!(element.is_stable());
    ///
    /// // Check that the empty set will indeed return `None`
    /// let empty_set = DistributionSet::from_iter(iter::empty());
    /// assert!(empty_set.last().is_none());
    /// ```
    pub fn last(&self) -> Option<&Distribution> {
        self.releases.last().map(|c| &c.0)
    }

    /// All held Rust releases in ascending order.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::iter;
    /// # use rust_releases_core2::{Distribution, DistributionSet};
    /// # use rust_toolchain::{Channel, Target, ReleaseDate, RustVersion, Toolchain};
    /// #
    /// # let date = ReleaseDate::new(2023, 1, 1);
    /// # let toolchain31 = Toolchain::new(Channel::stable(RustVersion::new(1, 31, 0)), Target::host());
    /// # let toolchain32 = Toolchain::new(Channel::stable(RustVersion::new(1, 32, 0)), Target::host());
    /// # let distribution31 = Distribution::new_without_components(date.clone(), toolchain31);
    /// # let distribution32 = Distribution::new_without_components(date.clone(), toolchain32);
    /// #
    /// // A set of with stable releases 1.31.0 and 1.32.0
    /// let set = DistributionSet::from_iter(vec![distribution31, distribution32]);
    ///
    /// let mut ascending_elements = set.ascending();
    ///
    /// let first = ascending_elements.next().unwrap().toolchain().version().unwrap();
    /// let second = ascending_elements.next().unwrap().toolchain().version().unwrap();
    /// let third = ascending_elements.next();
    ///
    /// assert_eq!(first, &RustVersion::new(1, 31, 0));
    /// assert_eq!(second, &RustVersion::new(1, 32, 0));
    /// assert!(third.is_none());
    /// ```
    pub fn ascending(&self) -> impl Iterator<Item = &Distribution> {
        self.releases.iter().map(|c| &c.0)
    }

    /// All held Rust distributions in descending order.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::iter;
    /// # use rust_releases_core2::{Distribution, DistributionSet};
    /// # use rust_toolchain::{Channel, Target, ReleaseDate, RustVersion, Toolchain};
    /// #
    /// # let date = ReleaseDate::new(2023, 1, 1);
    /// # let toolchain31 = Toolchain::new(Channel::stable(RustVersion::new(1, 31, 0)), Target::host());
    /// # let toolchain32 = Toolchain::new(Channel::stable(RustVersion::new(1, 32, 0)), Target::host());
    /// # let distribution31 = Distribution::new_without_components(date.clone(), toolchain31);
    /// # let distribution32 = Distribution::new_without_components(date.clone(), toolchain32);
    /// #
    /// // A set of with stable releases 1.31.0 and 1.32.0
    /// let set = DistributionSet::from_iter(vec![distribution31, distribution32]);
    ///
    /// let mut descending_elements = set.descending();
    ///
    /// let first = descending_elements.next().unwrap().toolchain().version().unwrap();
    /// let second = descending_elements.next().unwrap().toolchain().version().unwrap();
    /// let third = descending_elements.next();
    ///
    /// assert_eq!(first, &RustVersion::new(1, 32, 0));
    /// assert_eq!(second, &RustVersion::new(1, 31, 0));
    /// assert!(third.is_none());
    /// ```
    pub fn descending(&self) -> impl Iterator<Item = &Distribution> {
        self.releases.iter().rev().map(|c| &c.0)
    }

    /// Amount of releases held by the set.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::iter;
    /// # use rust_releases_core2::{Distribution, DistributionSet};
    /// # use rust_toolchain::{Channel, Target, ReleaseDate, RustVersion, Toolchain};
    /// #
    /// # let date = ReleaseDate::new(2023, 1, 1);
    /// # let toolchain31 = Toolchain::new(Channel::stable(RustVersion::new(1, 31, 0)), Target::host());
    /// # let toolchain32 = Toolchain::new(Channel::stable(RustVersion::new(1, 32, 0)), Target::host());
    /// # let distribution31 = Distribution::new_without_components(date.clone(), toolchain31);
    /// # let distribution32 = Distribution::new_without_components(date.clone(), toolchain32);
    /// #
    /// // A set of with stable releases 1.31.0 and 1.32.0
    /// let set = DistributionSet::from_iter(vec![distribution31, distribution32]);
    ///
    /// assert_eq!(set.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.releases.len()
    }
}
