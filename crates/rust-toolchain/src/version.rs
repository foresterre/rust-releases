use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

/// A three component, `major.minor.patch` version number.
///
/// This version number is a subset of [semver](https://semver.org/spec/v2.0.0.html), except that
/// it only accepts the numeric MAJOR, MINOR and PATCH components, while pre-release and build
/// metadata, and other labels, are rejected.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct RustVersion {
    version: version_number::FullVersion,
}

impl RustVersion {
    /// Instantiate a semver compatible three component version number.
    ///
    /// This version is a subset of semver. It does not support the extensions
    /// to the MAJOR.MINOR.PATCH format, i.e. the additional labels for
    /// pre-releases and build metadata.
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            version: version_number::FullVersion {
                major,
                minor,
                patch,
            },
        }
    }
}

impl RustVersion {
    /// The major version of a semver three component version number
    pub fn major(&self) -> u64 {
        self.version.major
    }

    /// The minor version of a semver three component version number
    pub fn minor(&self) -> u64 {
        self.version.minor
    }

    /// The patch version of a semver three component version number
    pub fn patch(&self) -> u64 {
        self.version.patch
    }
}

impl FromStr for RustVersion {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use version_number::parsers::error::ExpectedError;
        use version_number::parsers::error::NumericError;
        use version_number::ParserError;

        version_number::FullVersion::parse(s)
            .map(|version| Self { version })
            .map_err(|e| match e {
                ParserError::Expected(inner) => match inner {
                    ExpectedError::Numeric { got, .. } => ParseError::Expected("0-9", got),
                    ExpectedError::Separator { got, .. } => ParseError::Expected(".", got),
                    ExpectedError::EndOfInput { got, .. } => ParseError::Expected("EOI", Some(got)),
                },
                ParserError::Numeric(inner) => match inner {
                    NumericError::LeadingZero => ParseError::LeadingZero,
                    NumericError::Overflow => ParseError::NumberOverflow,
                },
            })
    }
}

impl fmt::Display for RustVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.version)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum ParseError {
    #[error("Expected '{0}' but got '{got}'", got = .1.map(|c| c.to_string()).unwrap_or_default())]
    Expected(&'static str, Option<char>),

    #[error("expected token 1-9, but got '0' (leading zero is not permitted)")]
    LeadingZero,

    #[error("unable to parse number (overflow occurred)")]
    NumberOverflow,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn create_rust_version() {
        let version = RustVersion::new(1, 2, 3);

        assert_eq!(version.major(), 1);
        assert_eq!(version.minor(), 2);
        assert_eq!(version.patch(), 3);
    }

    #[test]
    fn display() {
        let version = RustVersion::new(12, 2, 24);

        assert_eq!(&format!("{version}"), "12.2.24");
    }

    #[test]
    fn partial_eq() {
        let left = RustVersion::new(1, 2, 3);
        let right = RustVersion::new(1, 2, 3);

        assert_eq!(left, right);
    }

    #[test]
    fn eq() {
        let left = RustVersion::new(1, 2, 3);
        let right = RustVersion::new(1, 2, 3);

        assert!(left.eq(&right));
    }

    #[yare::parameterized(
        on_major = { RustVersion::new(1, 0, 0), RustVersion::new(0, 0, 0), Ordering::Greater },
        on_minor = { RustVersion::new(1, 1, 0), RustVersion::new(1, 0, 0), Ordering::Greater },
        on_patch = { RustVersion::new(1, 1, 1), RustVersion::new(1, 1, 0), Ordering::Greater },
        eq = { RustVersion::new(1, 1, 1), RustVersion::new(1, 1, 1), Ordering::Equal },
    )]
    fn ordering(left: RustVersion, right: RustVersion, expected_ord: Ordering) {
        assert_eq!(left.partial_cmp(&right), Some(expected_ord));
        assert_eq!(left.cmp(&right), expected_ord);
    }

    mod partial_eq {
        use super::*;

        #[test]
        fn symmetric() {
            let left = RustVersion::new(1, 2, 3);
            let right = RustVersion::new(1, 2, 3);

            assert_eq!(
                left, right,
                "PartialEq should be symmetric: 'left == right' must hold"
            );
            assert_eq!(
                right, left,
                "PartialEq should be symmetric: 'right == left' must hold"
            );
        }

        #[test]
        fn transitive() {
            let a = RustVersion::new(1, 2, 3);
            let b = RustVersion::new(1, 2, 3);
            let c = RustVersion::new(1, 2, 3);

            assert_eq!(
                a, b,
                "PartialEq should be transitive: 'a == b' must hold, by symmetric property"
            );
            assert_eq!(
                b, c,
                "PartialEq should be transitive: 'b == c' must hold, by symmetric property"
            );

            assert_eq!(a, c, "PartialEq should be transitive: 'a == c' must hold, given a == b (prior) and b == c (prior)");
        }
    }

    mod partial_ord {
        use super::*;

        #[test]
        fn equality() {
            let a = RustVersion::new(1, 2, 3);
            let b = RustVersion::new(1, 2, 3);

            assert_eq!(
                a, b,
                "PartialOrd should hold for equality: 'a == b' must hold"
            );
            assert_eq!(a.partial_cmp(&b), Some(Ordering::Equal), "PartialOrd should hold for equality: 'a.partial_cmp(&b) == Ordering::Equal' must hold");
        }

        #[test]
        fn transitive_lt() {
            let a = RustVersion::new(1, 2, 1);
            let b = RustVersion::new(1, 2, 2);
            let c = RustVersion::new(1, 2, 3);

            assert!(a < b, "PartialOrd should be transitive: 'a < b' must hold");
            assert!(b < c, "PartialOrd should be transitive: 'b < c' must hold");
            assert!(a < c, "PartialOrd should be transitive: 'a < c' must hold, given a < b (prior) and b < c (prior)");
        }

        #[test]
        fn transitive_gt() {
            let a = RustVersion::new(1, 2, 3);
            let b = RustVersion::new(1, 2, 2);
            let c = RustVersion::new(1, 2, 1);

            assert!(a > b, "PartialOrd should be transitive: 'a > b' must hold");
            assert!(b > c, "PartialOrd should be transitive: 'b > c' must hold");
            assert!(a > c, "PartialOrd should be transitive: 'a > c' must hold, given a > b (prior) and b > c (prior)");
        }
    }
}
