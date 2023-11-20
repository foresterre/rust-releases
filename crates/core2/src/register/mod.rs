//! See [`Register`].

use crate::{Distribution, DistributionSet};
use rust_toolchain::{Beta, Nightly, Stable, Target};
use std::collections::HashMap;
use std::iter;
use std::sync::Arc;

// Contains the actual implementation of the register API defined here.
mod r#impl;

#[cfg(test)]
mod tests;

/// The [`Register`] data structure provides a way to store and query the available
/// Rust distributions, known to this library.
#[derive(Clone, Debug)]
pub struct Register {
    stable: HashMap<rust_toolchain::Target, DistributionSet>,
    beta: Arc<[Beta]>,
    nightly: Arc<[Nightly]>,
    // TODO:
    //  - impl::Register with
    //    trait Registry<Version>
    //      fn ...
    //
    //  - stable: impl::Register<Index=RustVersion>
    //  - beta: impl::Register<Index=RustVersion>
    //  - nightly: impl::Register<Index=RustDate>
    //
    // map version | nightly version -> Platform
    // see https://github.com/foresterre/cargo-msrv/issues/587#issuecomment-1793153675
    platform_register: r#impl::PlatformRegister,
}

// Instantiations
impl Register {
    /// Create a new, empty [`Register`].
    ///
    /// # Example
    ///
    /// ```
    /// use rust_releases_core2::Register;
    ///
    /// let _ = Register::new();
    /// ```
    pub fn new() -> Self {
        let platform_register = r#impl::PlatformRegister::from_iter(iter::empty());

        Self { platform_register }
    }

    /// Create a new [`Register`] from an iterable of `([`rust_toolchain::Platform`], [`Distribution`])`
    /// tuples.
    pub fn from_iter<I: IntoIterator<Item = (rust_toolchain::Target, Distribution)>>(
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
    /// All releases for the given channel description.
    // pub fn releases_of(&self, channel: &Channel) -> impl IntoIterator<Item = Release> {
    //     self.platform_register
    //         .distributions_by_channel(channel)
    //         .map(|dist| {
    //              dist.to_release()
    //          })
    //          .collect::<Vec<_>>
    // }

    /// Get all releases for a given platform.
    pub fn platform(&self, id: &rust_toolchain::Target) -> Option<&DistributionSet> {
        self.platform_register.platform(id)
    }

    /// The amount of releases inserted into this register, regardless of platform.
    pub fn size(&self) -> usize {
        self.platform_register.size()
    }
}
