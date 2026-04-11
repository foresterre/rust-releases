use crate::merge::MergeToolchains;
use rust_release::toolchain;

/// Prefers the left toolchains, ignoring the right entirely.
pub struct PreferLeftToolchains;

impl MergeToolchains for PreferLeftToolchains {
    fn merge_toolchains(
        &self,
        left: Vec<toolchain::Toolchain>,
        _right: Vec<toolchain::Toolchain>,
    ) -> Vec<toolchain::Toolchain> {
        left
    }
}

/// Deduplicates and concatenates toolchains from both sides.
///
/// Toolchains from the right side are appended only if they are not
/// already present on the left side.
pub struct UnionToolchains;

impl MergeToolchains for UnionToolchains {
    fn merge_toolchains(
        &self,
        mut left: Vec<toolchain::Toolchain>,
        right: Vec<toolchain::Toolchain>,
    ) -> Vec<toolchain::Toolchain> {
        for t in right {
            if !left.contains(&t) {
                left.push(t);
            }
        }
        left
    }
}
