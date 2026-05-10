use crate::merge::{MergeContext, MergeReleaseDate, MergeToolchains};
use rust_release::{date, toolchain};

/// Wraps a closure into a merge trait implementation.
///
/// The compiler selects the correct trait impl based on the closure's signature and the trait bound
/// required at the call site.
///
/// You might prefer the dedicated [`date_fn`], [`toolchains_fn`], and [`context_fn`] functions which
/// each implement a single trait, so the compiler can infer the closure parameter types without
/// annotations <3.
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
        self.0(left, right)
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
        self.0(left, right)
    }
}

impl<CL, CR, CO, F> MergeContext<CL, CR> for FromFn<F>
where
    F: Fn(CL, CR) -> CO,
{
    type Output = CO;

    fn merge_context(&self, left: CL, right: CR) -> CO {
        self.0(left, right)
    }
}

/// Creates a [`MergeReleaseDate`] strategy from a closure.
///
/// # Example
///
/// ```
/// use rust_releases_core::merge::from_fn::date_fn;
/// use rust_releases_core::merge::builder::MergeBuilder;
/// use rust_release::{RustRelease, Stable};
///
/// let left = RustRelease::new(Stable::new(1, 2, 3), None, []);
/// let right = RustRelease::new(Stable::new(1, 2, 3), None, []);
///
/// let _merged = MergeBuilder::new(left, right)
///     .date_merge(date_fn(|l, r| r.or(l)))
///     .finish();
/// ```
pub fn date_fn<F>(f: F) -> impl MergeReleaseDate
where
    F: Fn(Option<date::Date>, Option<date::Date>) -> Option<date::Date>,
{
    FromFn(f)
}

/// Creates a [`MergeToolchains`] strategy from a closure.
///
/// # Example
///
/// ```
/// use rust_releases_core::merge::from_fn::toolchains_fn;
/// use rust_releases_core::merge::builder::MergeBuilder;
/// use rust_release::{RustRelease, Stable};
///
/// let left = RustRelease::new(Stable::new(1, 2, 3), None, []);
/// let right = RustRelease::new(Stable::new(1, 2, 3), None, []);
///
/// let _merged = MergeBuilder::new(left, right)
///     .toolchains_merge(toolchains_fn(|mut l, r| { l.extend(r); l }))
///     .finish();
/// ```
pub fn toolchains_fn<F>(f: F) -> impl MergeToolchains
where
    F: Fn(Vec<toolchain::Toolchain>, Vec<toolchain::Toolchain>) -> Vec<toolchain::Toolchain>,
{
    FromFn(f)
}

/// Creates a [`MergeContext`] strategy from a closure.
pub fn context_fn<CL, CR, CO, F>(f: F) -> impl MergeContext<CL, CR, Output = CO>
where
    F: Fn(CL, CR) -> CO,
{
    FromFn(f)
}
