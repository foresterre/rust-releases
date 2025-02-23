use crate::RustVersion;

/// The `Stable` release [`channel`]
///
/// [`channel`]: https://rust-lang.github.io/rustup/concepts/channels.html
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Stable {
    /// The three component Rust version
    pub version: RustVersion,
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
