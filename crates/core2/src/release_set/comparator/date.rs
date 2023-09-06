use crate::Release;
use std::cmp::Ordering;

/// A comparator for Releases `a` and `b` and given `date(a) cmp date(b)`, then `a cmp b` holds the
/// same ordering as `date(a) cmp date(b)`.
#[derive(Clone, Debug)]
pub struct DateComparator(pub Release);

impl PartialEq for DateComparator {
    fn eq(&self, other: &Self) -> bool {
        self.0.date().eq(other.0.date())
    }
}

impl Eq for DateComparator {}

impl PartialOrd for DateComparator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.date().partial_cmp(other.0.date())
    }
}

impl Ord for DateComparator {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.date().cmp(other.0.date())
    }
}

// TODO: test!
