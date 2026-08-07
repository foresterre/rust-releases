#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(unsafe_code)]

//! This crate aims to provide an index of Rust releases, and make it available to Rust programs.
//!
//! # Introduction
//!
//! The Rust programming language uses deterministic versioning for toolchain releases. Stable versions use SemVer,
//! while nightly, beta and historical builds can be accessed by using dated builds (YY-MM-DD).
//!
//! Unfortunately, a complete index of releases is not available any more. There are however
//! a few places where we can find partial release indices, from which we can build our own
//! index.
//!
//! This process consists of two parts: 1) obtaining the data sources, and 2) building the index
//! from these data sources. For the first part `rust-releases` provides the [`FetchResources`] trait, and
//! for the second part `rust-releases` provides the [`Source`] trait.
//! Both traits find their origin in the `rust-releases-core` crate, and re-exported here.
//!
//! # Using `rust-releases`
//!
//! To use this library, you can either add `rust-releases-core` as a dependency, combined with any
//! implemented source library, or you can add `rust-releases` as a dependency, and enable the
//! implemented source libraries of your choice as [`features`].
//!
//! By default, all four sources are enabled when depending on `rust-releases`. You can disable these
//! by setting `default-features = false` for `rust-releases` in the `Cargo.toml` manifest, or by
//! calling cargo with `cargo --no-default-features`. You can then cherry pick sources by adding the `features`
//! key to the `rust-releases` dependency and enabling the features you want, or by calling cargo with
//! `cargo --features "rust-releases-rust-changelog,rust-releases-rust-dist"` or any other combination of features
//! and sources.
//!
//! To use rust-releases, you must add at least one source implementation.
//!
//! **Example: using rust-releases-core + implemented source as dependency**
//!
//! To use `rust-releases-core` as a dependency, combined with any implemented source library; add
//! the following to your `Cargo.toml`:
//!
//! ```toml
//! # replace `*` with latest version, and
//! # replace `$RUST_RELEASES_SOURCE` with one of the implemented source crates
//! [dependencies]
//! rust-releases-core = "*"
//! rust-releases-$RUST_RELEASES_SOURCE
//! ```
//!
//! For example:
//!
//! ```toml
//! [dependencies]
//! rust-releases-core = "0.15.0"
//! rust-releases-rust-dist = "0.15.0"
//! ```
//!
//!
//! **Example using rust-releases + implemented source(s) as feature**
//!
//! To use `rust-releases` as a dependency, and enable the implemented source libraries of your choice
//! as [`features`], add the following to your `Cargo.toml`:
//!
//! ```toml
//! # replace `*` with latest version, and replace `$RUST_RELEASES_SOURCE` with one of the available source implementations
//! [dependencies.rust-releases]
//! version = "*"
//! default-features = false
//! features = ["rust-release-$RUST_RELEASES_SOURCE"]
//! ```
//!
//! For example:
//!
//! ```toml
//! [dependencies.rust-releases]
//! version = "0.15.0"
//! default-features = false
//! features = ["rust-release-rust-dist"]
//! ```
//!
//! # Implemented sources
//!
//! `rust-releases` provides four [`Source`] implementations. Three out of four also provide
//! a [`FetchResources`] implementation. Each implementation requires adding the implementation crate
//! as an additional dependency or feature (see <a href="#using-rust-releases">using rust-releases</a>.
//!
//! The implementations are:
//! 1) [`RustChangelog`]: Build an index from the [RELEASES.md](https://raw.githubusercontent.com/rust-lang/rust/master/RELEASES.md) found in the root of the Rust source code repository.
//!     * Select this implementation by adding `rust-releases-rust-changelog` as a dependency
//! 2) [`RustDist`]: Build an index from the AWS S3 Rust distribution bucket; input data can be obtained using the [`FetchResources`] trait.
//!     * Select this implementation by adding `rust-releases-rust-dist` as a dependency
//!
//! # Choosing an implementation
//!
//! When in doubt, use the [`RustChangelog`] source for stable releases, and [`RustDist`] for anything else.
//!
//! # Issues
//!
//! Feel free to open an issue at our [repository](https://github.com/foresterre/rust-releases/issues)
//! for questions, feature requests, bug fixes, or other points of feedback 🤗.
//!
//! [`FetchResources`]: rust_releases_core::FetchResources
//! [`Source`]: rust_releases_core::Source
//! [`RustChangelog`]: rust_releases_rust_changelog::RustChangelog
//! [`RustDist`]: rust_releases_rust_dist::RustDist
//! [`features`]: https://doc.rust-lang.org/cargo/reference/features.html#features

// core re-export
pub use rust_releases_core as core;

#[cfg(feature = "rust-releases-io")]
pub use rust_releases_io::{
    AsyncHttpTransport, AsyncRustReleasesClient, BaseCacheDirError, BoxFuture, Document,
    HttpCachedClient, HttpCachedClientError, HttpClient, HttpTransport, IsStaleError,
    RetrievedDocument, RustReleasesClient, Timeout, TransportError, TransportResult,
    base_cache_dir, is_stale,
};

#[cfg(all(feature = "rust-releases-io", feature = "reqwest"))]
pub use rust_releases_io::ReqwestTransport;

#[cfg(all(feature = "rust-releases-io", feature = "ureq"))]
pub use rust_releases_io::UreqTransport;

#[cfg(feature = "github")]
pub use rust_releases_github as github;

#[cfg(feature = "github")]
pub use rust_releases_github::{GithubReleases, GithubReleasesError};

#[cfg(feature = "rust-changelog")]
pub use rust_releases_rust_changelog::{RustChangelog, RustChangelogError, RustChangelogResult};

#[cfg(feature = "rust-releases-rust-dist")]
pub use rust_releases_rust_dist::{RustDist, RustDistError, RustDistResult};
