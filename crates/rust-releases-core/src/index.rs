use crate::release::Release;
use std::iter::FromIterator;

/// A release index is a data structure holding known Rust releases.
///
/// # Ordering contract
///
/// Releases must be ordered from the newest to the oldest known release.
#[derive(Debug)]
pub struct ReleaseIndex {
    index: Vec<Release>,
}

impl ReleaseIndex {
    /// Returns a slice of releases.
    pub fn releases(&self) -> &[Release] {
        &self.index
    }

    /// Returns the most recent release.
    ///
    /// Returns `None` if the index has not registered any release.
    pub fn most_recent(&self) -> Option<&Release> {
        self.index.first()
    }

    /// Returns the least recent (oldest) registered release.
    ///
    /// Returns `None` if the index has not registered any release.
    pub fn least_recent(&self) -> Option<&Release> {
        self.index.last()
    }
}

impl FromIterator<Release> for ReleaseIndex {
    /// Create a new `ReleaseIndex` from a given iterable.
    ///
    /// NB: Releases should already be sorted from the most recent to the least recent release.
    fn from_iter<T: IntoIterator<Item = Release>>(iter: T) -> Self {
        Self {
            index: iter.into_iter().collect(),
        }
    }
}
