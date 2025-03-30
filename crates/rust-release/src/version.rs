//! Version information for a Rust release.

/// A combination of a channel and the version number.
///
/// For stable and beta releases, we have a three component MAJOR.MINOR.PATCH
/// version number. For nightly releases, we have a release date.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReleaseVersion {
    /// A stable channel release version
    Stable(rust_toolchain::channel::Stable),
    /// A beta channel release version
    Beta(rust_toolchain::channel::Beta),
    /// A nightly channel release version
    Nightly(rust_toolchain::channel::Nightly),
}
