//! # rust-toolchain
//!
//! The [`rust-toolchain`] crate defines a set of types which model a Rust toolchain.
//!
//! While there is no definitive spec which defines what a "Rust toolchain" is,
//! it tries to follow official Rust sources as closely as possible.
//!
//! The project is currently primarily modelled around the rough [`toolchain`]
//! specification written by the [`rustup`] developers. For now, we have disregarded
//! custom toolchains altogether though, both in the name of simplicity, and because
//! the current users didn't really need it yet. If you would like to see it added,
//! please open an issue (preferably including your use case :)).
//!
//! This project is part of the [`rust-releases`] and [`cargo-msrv`] projects.
//!
//! In case you have a feature request, question, bug, or have another reason to
//! contact the developers, please create a new issue at the `rust-releases` [`repository`].
//!
//! [`rust-toolchain`]: https://docs.rs/rust-toolchain/latest/rust_toolchain/
//! [`rustup`]: https://github.com/rust-lang/rustup
//! [`toolchain`]: https://rust-lang.github.io/rustup/concepts/toolchains.html
//! [`rust-releases`]: https://github.com/foresterre/rust-releases
//! [`cargo-msrv`]: https://github.com/foresterre/cargo-msrv
//! [`repository`]: https://github.com/foresterre/rust-releases/issues
// #![deny(missing_docs)]
#![warn(clippy::all)]
#![deny(unsafe_code)]
#![deny(missing_docs)]

/// Rust release channels, such as stable, beta and nightly.
pub mod channel;
mod component;
mod date;
mod target;
mod toolchain;
mod version;

pub use channel::Channel;
pub use component::Component;
pub use date::Date;
pub use target::Target;
pub use toolchain::Toolchain;
pub use version::RustVersion;
