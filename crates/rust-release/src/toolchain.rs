//! Types for working with Rust toolchains in so far they're relevant to a Rust release.

/// Type to model a Rust toolchain, with additional metadata relevant to a
/// release.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseToolchain {
    toolchain: rust_toolchain::Toolchain,
    tier: TargetTier,
}

impl ReleaseToolchain {
    /// Create an ExtendedToolchain from a rust_toolchain::Toolchain
    pub fn new(toolchain: rust_toolchain::Toolchain, tier: TargetTier) -> Self {
        Self { toolchain, tier }
    }

    /// Get the toolchain
    pub fn toolchain(&self) -> &rust_toolchain::Toolchain {
        &self.toolchain
    }

    /// Get the toolchain tier
    pub fn tier(&self) -> TargetTier {
        self.tier
    }
}

/// Support tier for a target
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum TargetTier {
    /// Tier 1 target
    T1,
    /// Tier 2 target
    T2,
    /// Tier 2.5 target
    T2_5,
    /// Tier 3 target
    T3,
    /// Tier is unknown
    Unknown,
}
