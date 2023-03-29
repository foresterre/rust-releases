//! # rust-release
//!
//! The `rust-release` crate defines a set of types which describe a Rust release.
//!
//! // todo
//! While there is no definitive spec which defines what a "Rust toolchain" is,
//! we try to follow the official Rust release process as closely as possible.
//! The [`rustup`] project has written down a rough specification for [`toolchains`] used
//! by the Rust project. In the initial version, we will follow this spec, but disregard custom
//! toolchains altogether, in the name of simplicity.
//!
//! Additionally, the [`Release Channel Layout`] document in the Rust Forge book, describes
//! the contents of the channel manifests, which is probably the most exhaustive source of release
//! data in existence.
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
//! [`Release Channel Layout`]: https://forge.rust-lang.org/infra/channel-layout.html
#![allow(missing_docs)] // TODO: deny again
#![deny(clippy::all)]
#![deny(unsafe_code)]

use crate::channel::Channel;
use crate::date::Date;
use crate::target::{Target, Triple};
use std::collections::HashMap;

mod channel;
mod component;
mod date;
mod package;
mod target;
mod version;

/// A single Rust release.
pub struct Release {
    channel: Channel,
    date: Date,

    targets: HashMap<Triple, Target>,
}
