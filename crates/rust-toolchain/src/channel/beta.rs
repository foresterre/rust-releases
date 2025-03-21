use crate::RustVersion;

/// The `Beta` release [`channel`]
///
/// [`channel`]: https://rust-lang.github.io/rustup/concepts/channels.html
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Beta {
    /// The three component Rust version
    pub version: RustVersion,
    /// The nth pre-release beta version, if any
    pub prerelease: Option<u32>,
}

#[cfg(test)]
mod tests {
    use crate::{channel::Beta, RustVersion};

    #[yare::parameterized(
        day1 = { RustVersion::new(0, 0, 0), RustVersion::new(0, 0, 1) },
        month1 = { RustVersion::new(0, 0, 0), RustVersion::new(0, 1, 0) },
        year1 = { RustVersion::new(0, 0, 0), RustVersion::new(1, 0, 0) },
        month_over_day = { RustVersion::new(0, 0, 999), RustVersion::new(0, 1, 0) },
        year_over_day = { RustVersion::new(0, 0, 999), RustVersion::new(1, 0, 0) },
        year_over_month = { RustVersion::new(0, 999, 0), RustVersion::new(1, 0, 0) },
    )]
    fn ord(left: RustVersion, right: RustVersion) {
        let left = Beta {
            version: left,
            prerelease: None,
        };
        let right = Beta {
            version: right,
            prerelease: None,
        };

        assert!(left < right);
    }
}
