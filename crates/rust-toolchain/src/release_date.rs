/// A release date of the form `YYYY-MM-DD`.
///
/// It is up to the implementer to ensure that a constructed date is valid.
/// E.g. this date may accept `2023-02-30`, while February only has 28 or 29 days in
/// the Gregorian calendar.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseDate {
    date: DateImpl,
}

impl ReleaseDate {
    /// Create a new `ReleaseDate` instance.
    ///
    /// It is up to the caller to make sure that the given date is valid.
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Self {
            date: DateImpl { year, month, day },
        }
    }
}

/// A compact date consisting of a four number year, and a two number month and day.
/// Up to the caller to ensure it matches with their reality of a 'valid date'.
///
/// Not intended to be compatible with common date standards
#[derive(Clone, Debug, Eq, PartialEq)]
struct DateImpl {
    year: u16,
    month: u8,
    day: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
}
