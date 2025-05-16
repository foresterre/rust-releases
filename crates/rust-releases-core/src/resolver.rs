use crate::merge::{Merge, MergeCandidate};
use rust_release::rust_toolchain::channel::Stable;

// TODO: some sensible default impl's of resolvers
pub fn prefer_self<C1: Clone + Default, C2>(
    _version: &Stable,
    current: MergeCandidate<C1>,
    incoming: MergeCandidate<C2>,
) -> Merge<C1> {
    Merge {
        // Prefer self's release dates if available
        release_date: match current.release_date {
            Some(date) => date.map(Clone::clone),
            None => incoming.release_date.flatten().map(Clone::clone),
        },

        // Prefer self's toolchains if available
        // Use other
        toolchains: match current.toolchains {
            Some(toolchains) => toolchains.clone(),
            None => incoming.toolchains.map(Clone::clone).unwrap_or_default(),
        },

        // Prefer self's context if available
        context: match current.context {
            Some(context) => context.clone(),
            // TODO: ... or panic if self's context is None (since C1 is required for return)
            None => Default::default(),
        },
    }
}
