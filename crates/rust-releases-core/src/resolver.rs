use crate::merge::PartialRustRelease;
use rust_release::rust_toolchain::Date;
use rust_release::toolchain::TargetToolchain;
use rust_release::RustRelease;
use std::cmp;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::marker::PhantomData;
// TODO: provide a builder to build suitable resolvers?

pub struct ConflictResolutionBuilder<V> {
    version: PhantomData<V>,
    release_date_resolver: ReleaseDateResolver<V>,
    toolchains_resolver: ToolchainsResolver<V>,
}

impl<V> Default for ConflictResolutionBuilder<V> {
    fn default() -> Self {
        Self {
            version: PhantomData,
            release_date_resolver: ReleaseDateResolver::default(),
            toolchains_resolver: ToolchainsResolver::default(),
        }
    }
}

impl<V> ConflictResolutionBuilder<V> {
    pub fn with_release_date_resolver(
        self,
        resolver: ReleaseDateResolver<V>,
    ) -> ConflictResolutionBuilder<V> {
        ConflictResolutionBuilder {
            version: PhantomData,
            release_date_resolver: resolver,
            toolchains_resolver: self.toolchains_resolver,
        }
    }

    pub fn with_release_date_fn(
        self,
        f: fn(V, Option<Date>, Option<Date>) -> Option<Date>,
    ) -> ConflictResolutionBuilder<V> {
        self.with_release_date_resolver(ReleaseDateResolver::new(f))
    }

    pub fn with_toolchains_resolver(
        self,
        resolver: ToolchainsResolver<V>,
    ) -> ConflictResolutionBuilder<V> {
        ConflictResolutionBuilder {
            version: PhantomData,
            release_date_resolver: self.release_date_resolver,
            toolchains_resolver: resolver,
        }
    }

    pub fn with_toolchains_fn(
        self,
        f: fn(
            V,
            Option<Vec<TargetToolchain>>,
            Option<Vec<TargetToolchain>>,
        ) -> Vec<TargetToolchain>,
    ) -> ConflictResolutionBuilder<V> {
        self.with_toolchains_resolver(ToolchainsResolver::new(f))
    }
}

impl<V: Clone> ConflictResolutionBuilder<V> {
    pub fn build_or_default(
        self,
    ) -> impl Fn(V, PartialRustRelease, PartialRustRelease) -> RustRelease<V> {
        move |version: V, lhs: PartialRustRelease, rhs: PartialRustRelease| RustRelease {
            version: version.clone(),
            release_date: self.release_date_resolver.call(
                version.clone(),
                lhs.release_date,
                rhs.release_date,
            ),
            toolchains: self
                .toolchains_resolver
                .call(version, lhs.toolchains, rhs.toolchains),
        }
    }
}

pub struct ReleaseDateResolver<V> {
    v: PhantomData<V>,
    f: fn(V, Option<Date>, Option<Date>) -> Option<Date>,
}

impl<V> Default for ReleaseDateResolver<V> {
    fn default() -> Self {
        Self::most_recent()
    }
}

impl<V> ReleaseDateResolver<V> {
    pub fn new(f: fn(V, Option<Date>, Option<Date>) -> Option<Date>) -> Self {
        Self { v: PhantomData, f }
    }

    pub fn call(&self, version: V, lhs: Option<Date>, rhs: Option<Date>) -> Option<Date> {
        (self.f)(version, lhs, rhs)
    }

    pub fn most_recent() -> Self {
        Self::new(|_v, lhs, rhs| {
            match (lhs, rhs) {
                (Some(l), Some(r)) => Some(cmp::max(l, r)), // Use most recent
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                (None, None) => None,
            }
        })
    }
}

pub struct ToolchainsResolver<V> {
    f: fn(V, Option<Vec<TargetToolchain>>, Option<Vec<TargetToolchain>>) -> Vec<TargetToolchain>,
}

impl<V> Default for ToolchainsResolver<V> {
    fn default() -> Self {
        Self::deduped()
    }
}

impl<V> ToolchainsResolver<V> {
    pub fn new(
        f: fn(
            V,
            Option<Vec<TargetToolchain>>,
            Option<Vec<TargetToolchain>>,
        ) -> Vec<TargetToolchain>,
    ) -> Self {
        Self { f }
    }
    pub fn call(
        &self,
        version: V,
        lhs: Option<Vec<TargetToolchain>>,
        rhs: Option<Vec<TargetToolchain>>,
    ) -> Vec<TargetToolchain> {
        (self.f)(version, lhs, rhs)
    }

    pub fn chain() -> Self {
        Self::new(|_v, lhs, rhs| {
            match (lhs, rhs) {
                (Some(l), Some(r)) => l.into_iter().chain(r).collect(), // Maximalist combination of known toolchains
                (Some(l), None) => l,
                (None, Some(r)) => r,
                (None, None) => vec![],
            }
        })
    }

    pub fn deduped() -> Self {
        Self::new(|_v, lhs, rhs| {
            match (lhs, rhs) {
                (Some(l), Some(r)) => {
                    fn hash_toolchain(rt: &TargetToolchain) -> u64 {
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
        })
    }
}
