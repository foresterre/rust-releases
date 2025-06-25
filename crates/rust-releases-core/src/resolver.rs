use crate::merge::{Merge, MergeCandidate};
use rust_release::rust_toolchain::{Date, Toolchain};
use rust_release::toolchain::ReleaseToolchain;
use std::cmp;
use std::hash::{DefaultHasher, Hash, Hasher};
// TODO: provide a builder to build suitable resolvers?

#[derive(Default)]
pub struct ConflictResolutionBuilder<V, C, C2, C3> {
    release_date: Option<ReleaseDateResolver<V>>,
    toolchains: Option<ToolchainsResolver<V>>,
    context: Option<ContextResolver<V, C, C2, C3>>,
}

impl<V, C, C2, C3> ConflictResolutionBuilder<V, C, C2, C3> {
    pub fn release_date_resolver(mut self, f: ReleaseDateResolver<V>) -> Self {
        self.release_date = Some(f);
        self
    }

    pub fn toolchain_resolver(mut self, f: ToolchainsResolver<V>) -> Self {
        self.toolchains = Some(f);
        self
    }

    pub fn context_resolver(mut self, f: ContextResolver<V, C, C2, C3>) -> Self {
        self.context = Some(f);
        self
    }
}

impl<V, C, C2, C3: Default> ConflictResolutionBuilder<V, C, C2, C3> {
    pub fn build_or_default(
        self,
    ) -> impl Fn(&V, MergeCandidate<C>, MergeCandidate<C2>) -> Merge<C3> {
        move |version: &V, lhs: MergeCandidate<C>, rhs: MergeCandidate<C2>| {
            let Self {
                release_date,
                toolchains,
                context,
            } = self; // TODO this won't disjointly capture grrrr

            Merge {
                release_date: release_date.unwrap_or_default().f(
                    version,
                    lhs.release_date.flatten(),
                    rhs.release_date.flatten(),
                ),
                toolchains: toolchains.unwrap_or_default().f(
                    version,
                    lhs.toolchains,
                    rhs.toolchains,
                ),
                context: context
                    .unwrap_or_default()
                    .f(version, lhs.context, rhs.context),
            }
        }
    }
}

pub struct ReleaseDateResolver<V> {
    f: Box<dyn Fn(&V, Option<Date>, Option<Date>) -> Option<Date>>,
}

impl<V> Default for ReleaseDateResolver<V> {
    fn default() -> Self {
        Self::most_recent()
    }
}

impl<V> ReleaseDateResolver<V> {
    pub fn new(f: Box<dyn Fn(&V, Option<Date>, Option<Date>) -> Option<Date>>) -> Self {
        Self { f }
    }

    pub fn f(&self, version: &V, lhs: Option<Date>, rhs: Option<Date>) -> Option<Date> {
        (self.f)(version, lhs, rhs)
    }

    pub fn most_recent() -> Self {
        Self {
            f: Box::new(|_v, lhs, rhs| {
                match (lhs, rhs) {
                    (Some(l), Some(r)) => Some(cmp::max(l, r)), // Use most recent
                    (Some(l), None) => Some(l),
                    (None, Some(r)) => Some(r),
                    (None, None) => None,
                }
            }),
        }
    }
}

pub struct ToolchainsResolver<V> {
    f: Box<
        dyn Fn(
            &V,
            Option<Vec<ReleaseToolchain>>,
            Option<Vec<ReleaseToolchain>>,
        ) -> Vec<ReleaseToolchain>,
    >,
}

impl<V> Default for ToolchainsResolver<V> {
    fn default() -> Self {
        Self::deduped()
    }
}

impl<V> ToolchainsResolver<V> {
    pub fn new(
        f: Box<
            dyn Fn(
                &V,
                Option<Vec<ReleaseToolchain>>,
                Option<Vec<ReleaseToolchain>>,
            ) -> Vec<ReleaseToolchain>,
        >,
    ) -> Self {
        Self { f }
    }

    pub fn f(
        &self,
        version: &V,
        lhs: Option<Vec<ReleaseToolchain>>,
        rhs: Option<Vec<ReleaseToolchain>>,
    ) -> Vec<ReleaseToolchain> {
        (self.f)(version, lhs, rhs)
    }

    pub fn chain() -> Self {
        Self {
            f: Box::new(|_v, lhs, rhs| {
                match (lhs, rhs) {
                    (Some(l), Some(r)) => l.into_iter().chain(r).collect(), // Maximalist combination of known toolchains
                    (Some(l), None) => l,
                    (None, Some(r)) => r,
                    (None, None) => vec![],
                }
            }),
        }
    }

    pub fn deduped() -> Self {
        Self {
            f: Box::new(|_v, lhs, rhs| {
                match (lhs, rhs) {
                    (Some(l), Some(r)) => {
                        fn hash_toolchain(rt: &ReleaseToolchain) -> u64 {
                            let mut hasher = DefaultHasher::new();
                            rt.toolchain().host().hash(&mut hasher);
                            rt.toolchain().date().hash(&mut hasher);
                            rt.toolchain().channel().hash(&mut hasher);
                            hasher.finish()
                        }

                        let mut vec = l.into_iter().chain(r).collect::<Vec<_>>();

                        // This is perhaps poor man's uniqueness.
                        // Can we do better? For example, mem::discriminant won't work because only channel is an enum.
                        //
                        // NB: We don't actually care about the ordering (which is unstable), we only care that unique values are placed
                        //     next to each other. We also do not to rely on the PartialEq of Toolchain, since we violate its
                        //     contract here by only using channel and host.
                        vec.sort_unstable_by(|a, b| {
                            let hash_a = hash_toolchain(a);
                            let hash_b = hash_toolchain(b);

                            hash_a.cmp(&hash_b)
                        });
                        // Only keep one of the unique values, better hope we aren't unlucky in the hash collision department
                        vec.dedup_by_key(|k| hash_toolchain(k));
                        vec
                    }
                    (Some(l), None) => l,
                    (None, Some(r)) => r,
                    (None, None) => vec![],
                }
            }),
        }
    }
}

pub struct ContextResolver<V, C, C2, C3> {
    f: Box<dyn Fn(&V, Option<C>, Option<C2>) -> C3>,
}

impl<V, C, C2, C3> ContextResolver<V, C, C2, C3> {
    pub fn new(f: Box<dyn Fn(&V, Option<C>, Option<C2>) -> C3>) -> Self {
        Self { f }
    }

    pub fn f(&self, version: &V, lhs: Option<C>, rhs: Option<C2>) -> C3 {
        (self.f)(version, lhs, rhs)
    }
}

impl<V, C, C2, C3: Default> Default for ContextResolver<V, C, C2, C3> {
    fn default() -> Self {
        Self::overwrite_with_default()
    }
}

impl<V, C, C2, C3: Default> ContextResolver<V, C, C2, C3> {
    /// Overwrites the context value with C3::default()
    pub fn overwrite_with_default() -> Self {
        Self {
            f: Box::new(|_v, _c, _c2| C3::default()),
        }
    }
}

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
        toolchain.date().hash(&mut hasher);
        toolchain.channel().hash(&mut hasher);
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
                //     next to each other. We also do not rely on the PartialEq of Toolchain, since we violate its
                //     contract here by only using channel, date and host (components and targets are HashSet's and
                //     cannot be hashed themselves (... without extracting their values)).
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
