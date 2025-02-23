use std::fmt;

/// A release date for a Rust toolchain.
///
/// Nightly toolchains use a date as version instead of a semver number.
/// This Date should be regarded as a form of a version number, just like semver.
///
/// For full-featured dates, it is recommended to use a dedicated library
/// like [`time`], [`chrono`] or [`jiff`].
///
/// [`time`]: https://docs.rs/time/latest/time/
/// [`chrono`]: https://docs.rs/chrono/latest/chrono/
/// [`jiff`]:https://docs.rs/jiff/latest/jiff/
#[derive(Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Date {
    date: DateImpl,
}

impl Date {
    /// Create a new `Date` instance.
    ///
    /// While it is called a `date`, it is merely a shallow representation of
    /// a day in time where a new release was cut. This release date
    /// should not be used to perform operations on, and may not even be
    /// valid in the Gregorian calendar. For example, a date `0-0-0` will
    /// be accepted, but is not a valid month nor year for most parsers
    /// which parse a Gregorian date.
    ///
    /// It is up to the caller to make sure that the given date is valid.
    /// This library just takes a date representation "as-is".
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Self {
            date: DateImpl { year, month, day },
        }
    }

    /// The year
    pub fn year(&self) -> u16 {
        self.date.year
    }

    /// The month
    pub fn month(&self) -> u8 {
        self.date.month
    }

    /// The day
    pub fn day(&self) -> u8 {
        self.date.day
    }

    /// Prints a yyyy-mm-dd representation of a release date.
    ///
    /// This representation may, just like [`Date`], be not a valid date
    /// in the Gregorian calendar. The date is merely a representation.
    ///
    /// Year, month, and day will all be pre-filled with 0's.
    /// For year, at least four numbers are shown. For
    /// month and day, two.
    ///
    /// Note that, a representation of `9999-200-200` is still possible, while
    /// not valid as a Gregorian date.
    pub fn ymd(&self) -> impl fmt::Display {
        let y = self.date.year;
        let m = self.date.month;
        let d = self.date.day;

        format!("{y:04}-{m:02}-{d:02}")
    }
}

/// A compact date consisting of a four number year, and a two number month and day.
/// Up to the caller to ensure it matches with their reality of a 'valid date'.
///
/// Not intended to be compatible with common date standards
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct DateImpl {
    year: u16,
    month: u8,
    day: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[yare::parameterized(
        zeroes = { Date::new(0, 0, 0), 0, 0, 0  },
        one_two_three = { Date::new(1, 2, 3), 1, 2, 3 },
        max = { Date::new(u16::MAX, u8::MAX, u8::MAX), u16::MAX, u8::MAX, u8::MAX },
    )]
    fn create_release_date(date: Date, year: u16, month: u8, day: u8) {
        let expected = Date {
            date: DateImpl { year, month, day },
        };

        assert_eq!(date, expected);
    }

    #[test]
    fn compare_year() {
        let smaller = Date::new(2000, 1, 1);
        let bigger = Date::new(2001, 1, 1);

        assert!(smaller < bigger);
        assert!(smaller <= bigger);
    }

    #[test]
    fn compare_month() {
        let smaller = Date::new(2000, 1, 1);
        let bigger = Date::new(2000, 2, 1);

        assert!(smaller < bigger);
        assert!(smaller <= bigger);
    }

    #[test]
    fn compare_day() {
        let smaller = Date::new(2000, 1, 1);
        let bigger = Date::new(2000, 1, 2);

        assert!(smaller < bigger);
        assert!(smaller <= bigger);
    }

    #[yare::parameterized(
        zeroes = { Date::new(0, 0, 0), "0000-00-00"  },
        fill_y = { Date::new(1, 10, 10), "0001-10-10"  },
        fill_m = { Date::new(1000, 1, 10), "1000-01-10"  },
        fill_d = { Date::new(1000, 10, 1), "1000-10-01"  },
        invalid_month_is_not_rejected = { Date::new(1000, 100, 1), "1000-100-01"  },
        invalid_day_is_not_rejected = { Date::new(1000, 1, 100), "1000-01-100"  },
    )]
    fn to_string(date: Date, expected: &str) {
        assert_eq!(date.ymd().to_string(), expected.to_string());
    }

    #[test]
    fn newer_date() {
        let newer = Date::new(2000, 1, 1);
        let older = Date::new(1999, 1, 1);

        assert_eq!(newer.cmp(&older), Ordering::Greater);
    }
}
