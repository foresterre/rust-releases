use crate::RustVersion;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Stable {
    pub version: RustVersion,
}

#[cfg(test)]
mod tests {
    use crate::{RustVersion, Stable};

    #[yare::parameterized(
        patch1 = { RustVersion::new(0, 0, 0), RustVersion::new(0, 0, 1) },
        minor1 = { RustVersion::new(0, 0, 0), RustVersion::new(0, 1, 0) },
        major1 = { RustVersion::new(0, 0, 0), RustVersion::new(1, 0, 0) },
        minor_trumps_patch = { RustVersion::new(0, 0, 999), RustVersion::new(0, 1, 0) },
        major_trumps_patch = { RustVersion::new(0, 0, 999), RustVersion::new(1, 0, 0) },
        major_trumps_minor = { RustVersion::new(0, 999, 0), RustVersion::new(1, 0, 0) },
    )]
    fn ord(left: RustVersion, right: RustVersion) {
        let left = Stable { version: left };
        let right = Stable { version: right };

        assert!(left < right);
    }
}
