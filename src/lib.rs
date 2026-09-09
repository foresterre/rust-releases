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
//! Unfortunately, a complete index of releases is not available anymore. There are however
//! a few places where we can find partial release indices, from which we can build our own
//! index.
//!
//! The data to build an index of releases is fetched from one of four currently supported data sources.
//! Not every data source supports all release channels (stable, beta, nightly). See the list below.
//!
//! Common types can be found in the `rust-releases-core` crate, and are re-exported in `rust-releases`.
//!
//! # Using `rust-releases`
//!
//! To use this library, you can either add `rust-releases-core` as a dependency, combined with any
//! implemented source library, or you can add `rust-releases` as a dependency, and enable the
//! implemented source libraries of your choice as [`features`].
//!
//! By default, only the `rust-changelog` source is enabled when depending on `rust-releases`. You can disable it
//! by setting `default-features = false` for `rust-releases` in the `Cargo.toml` manifest, or by
//! calling cargo with `cargo --no-default-features`. You can cherry pick sources by adding the `features`
//! key to the `rust-releases` dependency and enabling the features you want, or by calling cargo with
//! `cargo --features "rust-changelog,rust-dist"` or any other combination of features
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
//! rust-releases-core = "0.33.0"
//! rust-releases-rust-dist = "0.33.0"
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
//! features = ["$RUST_RELEASES_SOURCE"]
//! ```
//!
//! For example:
//!
//! ```toml
//! [dependencies.rust-releases]
//! version = "0.33.0"
//! default-features = false
//! features = ["rust-dist"]
//! ```
//!
//! # Implemented sources
//!
//! `rust-releases` provides four source implementations. Three out of four obtain their data over the
//! network. Each implementation requires adding the implementation crate
//! as an additional dependency or feature (see <a href="#using-rust-releases">using rust-releases</a>.
//!
//! The implementations are:
//! 1) [`RustChangelog`]: Obtain the stable releases from the [RELEASES.md](https://raw.githubusercontent.com/rust-lang/rust/master/RELEASES.md) found in the root of the Rust source code repository.
//!     * Select this implementation by adding `rust-releases-rust-changelog` as a dependency, or by enabling the `rust-changelog` feature
//! 2) [`GithubReleases`]: Obtain the stable releases from the GitHub releases API of the Rust source code repository.
//!     * Select this implementation by adding `rust-releases-github` as a dependency, or by enabling the `github` feature
//! 3) [`RustDist`]: Obtain the stable, beta and nightly releases from the AWS S3 Rust distribution bucket; the details of a release can additionally be read from its channel manifest.
//!     * Select this implementation by adding `rust-releases-rust-dist` as a dependency, or by enabling the `rust-dist` feature
//! 4) [`BundledReleases`]: Read the stable, beta and nightly releases from generated Rust code, which is bundled with the crate; requires no network access, but is only up-to-date up to the moment it was generated.
//!     * Select this implementation by adding `rust-releases-bundled` as a dependency, or by enabling the `bundled` feature; the beta and nightly channels are enabled with the `bundled-beta` and `bundled-nightly` features
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
//! [`RustChangelog`]: rust_releases_rust_changelog::RustChangelog
//! [`GithubReleases`]: rust_releases_github::GithubReleases
//! [`RustDist`]: rust_releases_rust_dist::RustDist
//! [`BundledReleases`]: rust_releases_bundled::BundledReleases
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

#[cfg(feature = "bundled")]
pub use rust_releases_bundled::{self as bundled, BundledReleases};

#[cfg(feature = "github")]
pub use rust_releases_github::{self as github, GithubReleases, GithubReleasesError};

#[cfg(feature = "rust-changelog")]
pub use rust_releases_rust_changelog::{self as rust_changelog, RustChangelog, RustChangelogError};

#[cfg(feature = "rust-dist")]
pub use rust_releases_rust_dist::{self as rust_dist, Detail, RustDist};
