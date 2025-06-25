use crate::merge::{Merge, MergeCandidate};
use rust_release::rust_toolchain::Toolchain;
use std::cmp;
use std::hash::{DefaultHasher, Hash, Hasher};

// TODO: provide a builder to build suitable resolvers?

/// This resolver tries to combine (meta) data from both `lhs` and `rhs`.
/// The C3 type must be the unit type.
///
/// - **release_date**: take the most recent, or the one present
/// - **toolchains**: combine the toolchains: does not filter any duplicate entries
/// - **context**: Overwrite with the unit type ()
pub fn combine<V, C, C2>(
    _version: &V,
    lhs: MergeCandidate<C>,
    rhs: MergeCandidate<C2>,
) -> Merge<()> {
    Merge {
        release_date: match (lhs.release_date.flatten(), rhs.release_date.flatten()) {
            (Some(l), Some(r)) => Some(cmp::max(l, r)), // Use most recent
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (None, None) => None,
        },
        toolchains: match (lhs.toolchains, rhs.toolchains) {
            (Some(l), Some(r)) => l.into_iter().chain(r).collect(), // Maximalist combination of known toolchains
            (Some(l), None) => l,
            (None, Some(r)) => r,
            (None, None) => vec![],
        },
        context: (),
    }
}

/// This resolver tries to combine (meta) data from both `lhs` and `rhs`.
/// The C3 type must be the unit type.
///
/// - **release_date**: take the most recent, or the one present
/// - **toolchains**: combine the toolchains: filters duplicate entries
/// - **context**: Overwrite with the unit type ()
pub fn dedup_toolchains<V, C, C2>(
    _version: &V,
    lhs: MergeCandidate<C>,
    rhs: MergeCandidate<C2>,
) -> Merge<()> {
    fn hash_toolchain(toolchain: &Toolchain) -> u64 {
        let mut hasher = DefaultHasher::new();
        toolchain.host().hash(&mut hasher);
        // toolchain.channel().hash(&mut hasher);
        hasher.finish()
    }

    Merge {
        release_date: match (lhs.release_date.flatten(), rhs.release_date.flatten()) {
            (Some(l), Some(r)) => Some(cmp::max(l, r)), // Use most recent
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (None, None) => None,
        },
        toolchains: match (lhs.toolchains, rhs.toolchains) {
            (Some(l), Some(r)) => {
                let mut vec = l.into_iter().chain(r).collect::<Vec<_>>();

                // This is perhaps poor man's uniqueness. Can we do better?
                //
                // NB: We don't actually care about the ordering (which is unstable), we only care that unique values are placed
                //     next to each other. We also do not to rely on the PartialEq of Toolchain, since we violate its
                //     contract here by only using channel and host.
                vec.sort_unstable_by(|a, b| {
                    let hash_a = hash_toolchain(a.toolchain());
                    let hash_b = hash_toolchain(b.toolchain());

                    hash_a.cmp(&hash_b)
                });
                // Only keep one of the unique values, better hope we aren't unlucky in the hash collision department
                vec.dedup_by_key(|k| hash_toolchain(k.toolchain()));
                vec
            }
            (Some(l), None) => l,
            (None, Some(r)) => r,
            (None, None) => vec![],
        },
        context: (),
    }
}
