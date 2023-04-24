//! # rust-toolchain
//!
//! The `rust-toolchain` crate defines a set of types which describe a Rust toolchain.
//! While there is no definitive spec which defines what a "Rust toolchain" is,
//! we try to follow the official Rust release process as closely as possible.
//! The [`rustup`] project has written down a rough specification for [`toolchains`] used
//! by the Rust project. In the initial version, we will follow this spec, but disregard custom
//! toolchains altogether, in the name of simplicity.
//!
//! This project is part of the [`rust-releases`] and [`cargo-msrv`] projects.
//! In case you have a feature request, question, bug, or have another reason to contact the developers,
//! please, create a new issue at the `rust-releases` [`repository`].
//!
//! [`rustup`]: https://github.com/rust-lang/rustup
//! [`toolchains`]: https://rust-lang.github.io/rustup/concepts/toolchains.html
//! [`rust-releases`]: https://github.com/foresterre/rust-releases
//! [`cargo-msrv`]: https://github.com/foresterre/cargo-msrv
//! [`repository`]: https://github.com/foresterre/rust-releases/issues
// #![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(unsafe_code)]

mod channel;
mod component;
mod platform;
mod release_date;
mod rust_version;
mod toolchain;

pub use channel::Channel;
pub use component::Component;
pub use platform::Platform;
pub use release_date::ReleaseDate;
pub use rust_version::RustVersion;
pub use toolchain::Toolchain;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_toolchain() {
        let toolchain = Toolchain::new(
            Channel::Stable,
            ReleaseDate::new(20, 1, 1),
            Platform::host(),
            None,
        );

        assert_eq!(&toolchain.channel, &Channel::Stable);
        assert_eq!(&toolchain.date, &ReleaseDate::new(20, 1, 1));
        assert_eq!(&toolchain.platform, &Platform::host());
        assert!(toolchain.version.is_none());
    }
}
