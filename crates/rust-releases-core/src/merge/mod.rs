pub mod builder;
pub mod from_fn;
pub mod strategy;

use std::fmt::Debug;

use rust_release::date;
use rust_release::toolchain;
use rust_release::RustRelease;

/// Resolves how to combine the `release_date` field during a merge.
pub trait MergeReleaseDate {
    fn merge_release_date(
        &self,
        left: Option<date::Date>,
        right: Option<date::Date>,
    ) -> Option<date::Date>;
}

/// Resolves how to combine the `toolchains` field during a merge.
pub trait MergeToolchains {
    fn merge_toolchains(
        &self,
        left: Vec<toolchain::Toolchain>,
        right: Vec<toolchain::Toolchain>,
    ) -> Vec<toolchain::Toolchain>;
}

/// Resolves how to combine the `context` field during a merge.
///
/// Separate from the field traits because it carries type parameters
/// that vary per call site.
pub trait ContextMerge<CL, CR> {
    type Output;

    fn merge_context(&self, left: CL, right: CR) -> Self::Output;
}

/// Merges two releases that share the same version.
///
/// Each field is resolved by its corresponding trait implementation,
/// allowing full control over the merge strategy per field.
///
/// # Panics (debug)
///
/// Panics if `left` and `right` have different versions (when compiling with `debug` on).
pub fn merge<V, CL, CR, D, T, C>(
    left: RustRelease<V, CL>,
    right: RustRelease<V, CR>,
    date_merge: &D,
    toolchains_merge: &T,
    context_merge: &C,
) -> RustRelease<V, C::Output>
where
    V: Eq + Debug,
    D: MergeReleaseDate,
    T: MergeToolchains,
    C: ContextMerge<CL, CR>,
{
    debug_assert_eq!(
        left.version, right.version,
        "cannot merge releases with different versions"
    );

    RustRelease {
        version: left.version,
        release_date: date_merge.merge_release_date(left.release_date, right.release_date),
        toolchains: toolchains_merge.merge_toolchains(left.toolchains, right.toolchains),
        context: context_merge.merge_context(left.context, right.context),
    }
}

/// Merges two releases with sane defaults: prefer left date, union
/// toolchains, unit context.
pub fn merge_default<V>(left: RustRelease<V, ()>, right: RustRelease<V, ()>) -> RustRelease<V, ()>
where
    V: Eq + Debug,
{
    builder::MergeBuilder::new(left, right).finish()
}
