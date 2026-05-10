use crate::merge::strategy::context::UnitContext;
use crate::merge::strategy::release_date::PreferLeftDate;
use crate::merge::strategy::toolchains::UnionToolchains;
use crate::merge::{merge, MergeContext, MergeReleaseDate, MergeToolchains};
use rust_release::RustRelease;
use std::fmt::Debug;

/// Builder for configuring a merge with selective overrides.
///
/// Starts with sane defaults (`PreferLeftDate`, `UnionToolchains`,
/// `UnitContext`) and allows overriding individual field strategies
/// by name.
pub struct MergeBuilder<V, CL, CR, D, T, C>
where
    V: Debug,
{
    left: RustRelease<V, CL>,
    right: RustRelease<V, CR>,
    date: D,
    toolchains: T,
    context: C,
}

impl<V: Debug> MergeBuilder<V, (), (), PreferLeftDate, UnionToolchains, UnitContext> {
    pub fn new(left: RustRelease<V, ()>, right: RustRelease<V, ()>) -> Self {
        Self {
            left,
            right,
            date: PreferLeftDate,
            toolchains: UnionToolchains,
            context: UnitContext,
        }
    }
}

impl<V, CL, CR, D, T, C> MergeBuilder<V, CL, CR, D, T, C>
where
    V: Eq + Debug,
    D: MergeReleaseDate,
    T: MergeToolchains,
    C: MergeContext<CL, CR>,
{
    pub fn date_merge<D2: MergeReleaseDate>(
        self,
        date_merge: D2,
    ) -> MergeBuilder<V, CL, CR, D2, T, C> {
        MergeBuilder {
            left: self.left,
            right: self.right,
            date: date_merge,
            toolchains: self.toolchains,
            context: self.context,
        }
    }

    pub fn toolchains_merge<T2: MergeToolchains>(
        self,
        toolchains: T2,
    ) -> MergeBuilder<V, CL, CR, D, T2, C> {
        MergeBuilder {
            left: self.left,
            right: self.right,
            date: self.date,
            toolchains,
            context: self.context,
        }
    }

    pub fn context_merge<C2: MergeContext<CL, CR>>(
        self,
        context_merge: C2,
    ) -> MergeBuilder<V, CL, CR, D, T, C2> {
        MergeBuilder {
            left: self.left,
            right: self.right,
            date: self.date,
            toolchains: self.toolchains,
            context: context_merge,
        }
    }

    pub fn finish(self) -> RustRelease<V, C::Output> {
        merge(
            self.left,
            self.right,
            &self.date,
            &self.toolchains,
            &self.context,
        )
    }
}
