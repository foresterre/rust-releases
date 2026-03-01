use rust_release::{date, toolchain, RustRelease};
use std::cmp::Ordering;
use std::collections::BTreeSet;

/// Trait for merging two pieces of data of the same type.
pub trait Merge {
    type Data;

    fn merge(&self, a: Self::Data, b: Self::Data) -> Self::Data;
}

// Merge strategy for RustRelease that prefers earlier dates
pub struct PreferEarlierDate;

impl<V> Merge for PreferEarlierDate {
    type Data = RustRelease<V>;

    fn merge(&self, a: Self::Data, b: Self::Data) -> Self::Data {
        let release_date = match (a.release_date, b.release_date) {
            (Some(d1), Some(d2)) => Some(if d1 <= d2 { d1 } else { d2 }),
            (Some(d), None) | (None, Some(d)) => Some(d),
            (None, None) => None,
        };

        let mut toolchains = a.toolchains;
        toolchains.extend(b.toolchains);

        RustRelease {
            version: a.version,
            release_date,
            toolchains,
        }
    }
}

// Fully customizable merge strategy
pub struct CustomMerge<FVersion, FDate, FToolchains> {
    pub version_merge: FVersion,
    pub date_merge: FDate,
    pub toolchains_merge: FToolchains,
}

impl<V, FVersion, FDate, FToolchains> Merge for CustomMerge<FVersion, FDate, FToolchains>
where
    FVersion: Fn(V, V) -> V,
    FDate: Fn(Option<date::Date>, Option<date::Date>) -> Option<date::Date>,
    FToolchains:
        Fn(Vec<toolchain::Toolchain>, Vec<toolchain::Toolchain>) -> Vec<toolchain::Toolchain>,
{
    type Data = RustRelease<V>;

    fn merge(&self, first: Self::Data, second: Self::Data) -> Self::Data {
        RustRelease {
            version: (self.version_merge)(first.version, second.version),
            release_date: (self.date_merge)(first.release_date, second.release_date),
            toolchains: (self.toolchains_merge)(first.toolchains, second.toolchains),
        }
    }
}

// Merge strategy for ReleasesImpl that delegates to a RustRelease merger
pub struct ReleaseMerger<M>(std::marker::PhantomData<M>);

impl<M> ReleaseMerger<M> {
    pub fn new() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<V, M> Merge for ReleaseMerger<M>
where
    V: Ord + Clone,
    M: Merge<Data = RustRelease<V>>,
{
    type Data = ReleasesImpl<V>;

    fn merge(&self, a: Self::Data, b: Self::Data) -> Self::Data {
        let mut result = BTreeSet::new();
        let mut other_releases = b.releases;

        for release in a.releases {
            if let Some(other_release) = other_releases.take(&release) {
                // Both have this version - merge them using M
                result.insert(M::merge(release, other_release));
            } else {
                // Only in first
                result.insert(release);
            }
        }

        // Add remaining releases from second that weren't matched
        result.extend(other_releases);

        ReleasesImpl { releases: result }
    }
}

// Convenience methods on the types themselves
impl<V> RustRelease<V> {
    /// Merges this release with another using a merge strategy.
    pub fn merge<M: Merge<Data = Self>>(self, other: Self) -> Self {
        M::merge(self, other)
    }
}

impl<V> ReleasesImpl<V> {
    /// Merges this releases collection with another using a merge strategy.
    pub fn merge<M: Merge<Data = Self>>(self, other: Self) -> Self {
        M::merge(self, other)
    }
}

// Usage examples:
//
// For RustRelease:
// let merged = release1.merge::<PreferEarlierDate>(release2);
//
// For ReleasesImpl:
// let merged = releases1.merge::<ReleaseMerger<PreferEarlierDate>>(releases2);
//
// Or using the trait directly:
// let merged = PreferEarlierDate::merge(release1, release2);
// let merged = ReleaseMerger::<PreferEarlierDate>::merge(releases1, releases2);
