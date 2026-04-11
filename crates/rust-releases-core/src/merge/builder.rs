use crate::merge::strategy::context::UnitContext;
use crate::merge::strategy::release_date::PreferLeftDate;
use crate::merge::strategy::toolchains::UnionToolchains;
use crate::merge::{merge, ContextMerge, MergeReleaseDate, MergeToolchains};
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
    date_merge: D,
    toolchains_merge: T,
    context_merge: C,
}

impl<V: Debug> MergeBuilder<V, (), (), PreferLeftDate, UnionToolchains, UnitContext> {
    pub fn new(left: RustRelease<V, ()>, right: RustRelease<V, ()>) -> Self {
        Self {
            left,
            right,
            date_merge: PreferLeftDate,
            toolchains_merge: UnionToolchains,
            context_merge: UnitContext,
        }
    }
}

impl<V, CL, CR, D, T, C> MergeBuilder<V, CL, CR, D, T, C>
where
    V: Eq + Debug,
    D: MergeReleaseDate,
    T: MergeToolchains,
    C: ContextMerge<CL, CR>,
{
    pub fn date_merge<D2: MergeReleaseDate>(
        self,
        date_merge: D2,
    ) -> MergeBuilder<V, CL, CR, D2, T, C> {
        MergeBuilder {
            left: self.left,
            right: self.right,
            date_merge,
            toolchains_merge: self.toolchains_merge,
            context_merge: self.context_merge,
        }
    }

    pub fn toolchains_merge<T2: MergeToolchains>(
        self,
        toolchains_merge: T2,
    ) -> MergeBuilder<V, CL, CR, D, T2, C> {
        MergeBuilder {
            left: self.left,
            right: self.right,
            date_merge: self.date_merge,
            toolchains_merge,
            context_merge: self.context_merge,
        }
    }

    pub fn context_merge<C2: ContextMerge<CL, CR>>(
        self,
        context_merge: C2,
    ) -> MergeBuilder<V, CL, CR, D, T, C2> {
        MergeBuilder {
            left: self.left,
            right: self.right,
            date_merge: self.date_merge,
            toolchains_merge: self.toolchains_merge,
            context_merge,
        }
    }

    pub fn finish(self) -> RustRelease<V, C::Output> {
        merge(
            self.left,
            self.right,
            &self.date_merge,
            &self.toolchains_merge,
            &self.context_merge,
        )
    }
}
