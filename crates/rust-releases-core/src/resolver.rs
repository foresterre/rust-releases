use crate::{MergeCandidate, MergeResult};
use rust_release::rust_toolchain::channel::Stable;

// TODO: some sensible default impl's of resolvers
pub fn prefer_self_resolver<C1: Clone, C2>(
    _version: &Stable,
    this: MergeCandidate<C1>,
    other: MergeCandidate<C2>,
) -> MergeResult<C1> {
    MergeResult {
        // Prefer self's release dates if available
        release_date: match this.release_date {
            Some(date) => date.clone(),
            None => other.release_date.map(|d| d.clone()).flatten(),
        },

        // Prefer self's toolchains if available
        toolchains: match this.toolchains {
            Some(toolchains) => toolchains.clone(),
            None => other.toolchains.map(|t| t.clone()).unwrap_or_default(),
        },

        // Prefer self's context if available
        context: match this.context {
            Some(context) => context.clone(),
            // TODO: panic if self's context is None (since C1 is required for return)
            None => other.context.unwrap(),
        },
    }
}
