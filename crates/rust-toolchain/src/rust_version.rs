/// A three component, `major.minor.patch` version number.
///
/// This version number is a subset of [semver](https://semver.org/spec/v2.0.0.html), except that
/// it only accepts the numeric MAJOR, MINOR and PATCH components, while pre-release and build
/// metadata, and other labels, are rejected.
#[derive(Clone, Debug, Eq, PartialEq)]
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

    #[test]
    fn create_rust_version() {
        let version = RustVersion::new(1, 2, 3);

        assert_eq!(version.major(), 1);
        assert_eq!(version.minor(), 2);
        assert_eq!(version.patch(), 3);
    }
}
