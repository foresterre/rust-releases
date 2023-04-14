/// A three component, `major.minor.patch` version number.
///
/// This version number is a subset of [semver](https://semver.org/spec/v2.0.0.html), except that
/// it only accepts the numeric MAJOR, MINOR and PATCH components, while pre-release and build
/// metadata, and other labels, are rejected.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RustVersion {
    version: version_number::MajorMinorPatch,
}

impl RustVersion {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        todo!()
        // Self {
        //     version: version_number::MajorMinorPatch {
        //         major, minor, patch
        //     }
        // }
    }
}

impl RustVersion {
    pub fn major(&self) -> u64 {
        todo!()
    }

    pub fn minor(&self) -> u64 {
        todo!()
    }

    pub fn patch(&self) -> u64 {
        todo!()
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
