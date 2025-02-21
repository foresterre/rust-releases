/// A three component, `major.minor.patch` version number.
///
/// This version number is a subset of [semver](https://semver.org/spec/v2.0.0.html), except that
/// it only accepts the numeric MAJOR, MINOR and PATCH components, while pre-release and build
/// metadata, and other labels, are rejected.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct RustVersion {
    version: version_number::FullVersion,
}

impl RustVersion {
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
    pub fn major(&self) -> u64 {
        self.version.major
    }

    pub fn minor(&self) -> u64 {
        self.version.minor
    }

    pub fn patch(&self) -> u64 {
        self.version.patch
    }
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
