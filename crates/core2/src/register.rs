use crate::{Distribution, DistributionSet};

// Contains the actual implementation of the register API defined here.
mod r#impl;

#[cfg(test)]
mod tests;

/// The [`Register`] data structure provides a way to store and query the available
/// Rust distributions, known to this library.
#[derive(Clone, Debug)]
pub struct Register {
    platform_register: r#impl::PlatformRegister,
}

// Instantiations
impl Register {
    /// Create a new [`Register`] from an iterable of `([`rust_toolchain::Platform`], [`Distribution`])`
    /// tuples.
    pub fn from_iter<I: IntoIterator<Item = (rust_toolchain::Platform, Distribution)>>(
        iterable: I,
    ) -> Self {
        let platform_register = r#impl::PlatformRegister::from_iter(iterable);

        Self { platform_register }
    }
}

// Modifications
impl Register {
    /// Add a [`Distribution`] to the [`Register`].
    pub fn add_distribution(&mut self, distribution: Distribution) {
        self.platform_register.add_distribution(distribution)
    }
}

// Getters
impl Register {
    /// Get all releases for a given platform.
    pub fn platform(&self, id: &rust_toolchain::Platform) -> Option<&DistributionSet> {
        self.platform_register.platform(id)
    }

    /// The amount of releases inked into this register, regardless of the platform.
    pub fn size(&self) -> usize {
        self.platform_register.size()
    }
}
