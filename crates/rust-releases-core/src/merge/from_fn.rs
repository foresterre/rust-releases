use crate::merge::{ContextMerge, MergeReleaseDate, MergeToolchains};
use rust_release::{date, toolchain};

/// Wraps a closure into a merge trait implementation.
///
/// The compiler selects the correct trait impl based on the closure's
/// signature and the trait bound required at the call site.
///
/// # Example
///
/// ```
/// use rust_releases_core::merge::from_fn::FromFn;
/// use rust_releases_core::merge::builder::MergeBuilder;
/// use rust_release::{RustRelease, Stable};
/// use rust_release::date::Date;
/// use rust_release::toolchain::Toolchain;
///
/// let left = RustRelease::new(Stable::new(1, 2, 3), None, []);
/// let right = RustRelease::new(Stable::new(1, 2, 3), None, []);
///
/// let _merged = MergeBuilder::new(left, right)
///     .date_merge(FromFn(|l: Option<Date>, r: Option<Date>| r.or(l)))
///     .toolchains_merge(FromFn(|mut l: Vec<Toolchain>, r| {
///         l.extend(r);
///         l
///     }))
///     .finish();
/// ```
pub struct FromFn<F>(pub F);

impl<F> MergeReleaseDate for FromFn<F>
where
    F: Fn(Option<date::Date>, Option<date::Date>) -> Option<date::Date>,
{
    fn merge_release_date(
        &self,
        left: Option<date::Date>,
        right: Option<date::Date>,
    ) -> Option<date::Date> {
        (self.0)(left, right)
    }
}

impl<F> MergeToolchains for FromFn<F>
where
    F: Fn(Vec<toolchain::Toolchain>, Vec<toolchain::Toolchain>) -> Vec<toolchain::Toolchain>,
{
    fn merge_toolchains(
        &self,
        left: Vec<toolchain::Toolchain>,
        right: Vec<toolchain::Toolchain>,
    ) -> Vec<toolchain::Toolchain> {
        (self.0)(left, right)
    }
}

impl<CL, CR, CO, F> ContextMerge<CL, CR> for FromFn<F>
where
    F: Fn(CL, CR) -> CO,
{
    type Output = CO;

    fn merge_context(&self, left: CL, right: CR) -> CO {
        (self.0)(left, right)
    }
}
