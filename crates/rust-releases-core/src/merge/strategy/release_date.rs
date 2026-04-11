use crate::merge::MergeReleaseDate;
use rust_release::date;

/// Prefers the left date if present, otherwise the right.
pub struct PreferLeftDate;

impl MergeReleaseDate for PreferLeftDate {
    fn merge_release_date(
        &self,
        left: Option<date::Date>,
        right: Option<date::Date>,
    ) -> Option<date::Date> {
        left.or(right)
    }
}

/// Picks the later date when both are present, otherwise whichever is defined.
pub struct LatestDate;

impl MergeReleaseDate for LatestDate {
    fn merge_release_date(
        &self,
        left: Option<date::Date>,
        right: Option<date::Date>,
    ) -> Option<date::Date> {
        match (left, right) {
            (Some(l), Some(r)) => Some(l.max(r)),
            (l, r) => l.or(r),
        }
    }
}
