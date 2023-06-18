use std::cmp::Ordering;

#[cfg(test)]
mod tests;

/// Comparator which compares a toolchain using the following parameters (in order):
/// 1. By Rust Version
/// 2. By Release date,
///
/// The `platform` and `channel` are ignored completely.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct RustToolchainComparator<'toolchain> {
    // NB: order matters here for auto-derive!
    version: Option<&'toolchain rust_toolchain::RustVersion>,
    date: &'toolchain rust_toolchain::ReleaseDate,
}

impl<'toolchain> From<&'toolchain rust_toolchain::Toolchain>
    for RustToolchainComparator<'toolchain>
{
    fn from(value: &'toolchain rust_toolchain::Toolchain) -> Self {
        Self {
            version: value.version(),
            date: value.release_date(),
        }
    }
}

impl<'toolchain> Ord for RustToolchainComparator<'toolchain> {
    fn cmp(&self, other: &Self) -> Ordering {
        let version_left = self.version;
        let version_right = other.version;

        match (version_left, version_right) {
            (Some(l), Some(r)) => l.cmp(r),
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (None, None) => self.date.cmp(other.date),
        }
    }
}
