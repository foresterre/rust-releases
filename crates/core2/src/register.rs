use crate::{Release, ReleaseSet};

// Contains the actual implementation of the register API defined here.
mod r#impl;

#[cfg(test)]
mod tests;

/// A data structure consisting of the known Rust releases.
///
/// Whether a release is known, and how much information is known about a release,
/// depends on the source used to build up this information.
#[derive(Clone, Debug)]
pub struct Register {
    platform_register: r#impl::PlatformRegister,
}

// Instantiations
impl Register {
    pub fn from_iter<I: IntoIterator<Item = (rust_toolchain::Platform, Release)>>(
        iterable: I,
    ) -> Self {
        let platform_register = r#impl::PlatformRegister::from_iter(iterable);

        Self { platform_register }
    }
}

// Modifications
impl Register {
    pub fn add_release(&mut self, release: Release) {
        self.platform_register.add_release(release)
    }
}

// Getters
impl Register {
    /// Get a subset of the register, where the subset contains just the releases of the given platform.
    pub fn platform(&self, id: &rust_toolchain::Platform) -> Option<&ReleaseSet> {
        self.platform_register.platform(id)
    }

    /// The amount of releases inked into this register, regardless of the platform.
    pub fn size(&self) -> usize {
        self.platform_register.size()
    }
}
