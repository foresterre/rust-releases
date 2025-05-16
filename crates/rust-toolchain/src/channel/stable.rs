use crate::RustVersion;

/// The `Stable` release [`channel`]
///
/// [`channel`]: https://rust-lang.github.io/rustup/concepts/channels.html
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Stable {
    /// The three component Rust version
    pub version: RustVersion,
}

impl Stable {
    /// Instantiate a new `Stable` struct, representing the version of a release channel.
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            version: RustVersion::new(major, minor, patch),
        }
    }
}

impl From<RustVersion> for Stable {
    fn from(version: RustVersion) -> Self {
        Self { version }
    }
}

impl From<(u64, u64, u64)> for Stable {
    fn from((major, minor, patch): (u64, u64, u64)) -> Self {
        Self::new(major, minor, patch)
    }
}

#[cfg(test)]
mod tests {
    use crate::{channel::Stable, RustVersion};

    #[yare::parameterized(
        patch1 = { RustVersion::new(0, 0, 0), RustVersion::new(0, 0, 1) },
        minor1 = { RustVersion::new(0, 0, 0), RustVersion::new(0, 1, 0) },
        major1 = { RustVersion::new(0, 0, 0), RustVersion::new(1, 0, 0) },
        minor_over_patch = { RustVersion::new(0, 0, 999), RustVersion::new(0, 1, 0) },
        major_over_patch = { RustVersion::new(0, 0, 999), RustVersion::new(1, 0, 0) },
        major_over_minor = { RustVersion::new(0, 999, 0), RustVersion::new(1, 0, 0) },
    )]
    fn ord(left: RustVersion, right: RustVersion) {
        let left = Stable { version: left };
        let right = Stable { version: right };

        assert!(left < right);
    }
}
