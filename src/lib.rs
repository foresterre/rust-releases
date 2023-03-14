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
//! 1) [`ChannelManifests`]: Build an index from Rust [release manifests](https://static.rust-lang.org/manifests.txt).
//!     * Select this implementation by adding `rust-releases-channel-manifests` as a dependency
//! 2) [`RustChangelog`]: Build an index from the [RELEASES.md](https://raw.githubusercontent.com/rust-lang/rust/master/RELEASES.md) found in the root of the Rust source code repository.
//!     * Select this implementation by adding `rust-releases-rust-changelog` as a dependency
//! 3) [`RustDist`]: Build an index from the AWS S3 Rust distribution bucket; input data can be obtained using the [`FetchResources`] trait.
//!     * Select this implementation by adding `rust-releases-rust-dist` as a dependency
//! 4) [`RustDistWithCLI`]: Build an index from the AWS S3 Rust distribution bucket; obtain the input data yourself using the `aws` cli.
//!     * Select this implementation by adding `rust-releases-rust-dist-with-cli` as a dependency
//!
//! # Choosing an implementation
//!
//! When in doubt, use the [`RustChangelog`] source for stable releases, and [`RustDist`] for anything else.
//! [`ChannelManifests`] should usually not be used, as it's out of date (last checked April 2021;
//! the last available input manifest is dated 2020-02-23). [`RustDistWithCLI`] requires manual input
//! but pulls from the same data source as [`RustDist`]. The [`RustDist`] source is however more complete
//! than the [`RustDistWithCLI`] source (as of April 2021).
//!
//! In the below example, we'll use one of the above sources ([`RustChangelog`]) to show you how you can
//! use this library.
//!
//! # Example
//!
//! ```rust
//! use rust_releases_core::{FetchResources, Source, Channel, ReleaseIndex};
//! use rust_releases_rust_changelog::RustChangelog;
//!
//! // We choose the RustChangelog source for this example; alternatives are RustDistWithCLI and ChannelManifests
//! let source = RustChangelog::fetch_channel(Channel::Stable).unwrap();
//!
//! // Build a release index using our source of choice
//! let index = ReleaseIndex::from_source(source).unwrap();
//!
//! // Do something with the release information
//! index.releases()
//!     .iter()
//!     .for_each(|release| {
//!         println!("release {:?}", release)
//!     });
//!
//! ```
//! # Table of implemented features
//!
//! <table>
//! <thead>
//!      <tr>
//!           <th>Type of data source</th>
//!           <th>Crate</th>
//!           <th>Trait</th>
//!           <th>Implemented</th>
//!           <th>Channels<sup>1</sup></th>
//!           <th>Speed<sup>2, 3</sup></th>
//!           <th>On disk cache size<sup>4</sup></th>
//!           <th>Notes</th>
//!      </tr>
//! </thead>
//! <tbody>
//!      <tr>
//!           <td rowspan="2">ChannelManifests</td>
//!           <td rowspan="2"><code>rust-releases-channel-manifests</code></td>
//!           <td>Source</td>
//!           <td>✅</td>
//!           <td rowspan="2">Stable, <strike>Beta & Nightly</strike><sup>Won't be implemented</sup></td>
//!           <td>Medium</td>
//!           <td>-</td>
//!           <td rowspan="2">Deprecated: Input data has not been updated since 2020-02-23 <sup>(<a href="https://github.com/foresterre/rust-releases/issues/9">#9</a>)</sup>. Use <code>RustDist</code> instead.</td>
//!      </tr>
//!      <tr>
//!           <td>FetchResources</td>
//!           <td>✅</td>
//!           <td>Extremely slow (~1 hour)</td>
//!           <td>~418 MB</td>
//!      </tr>
//!      <tr>
//!           <td rowspan="2">RustChangelog</td>
//!           <td rowspan="2"><code>rust-releases-rust-changelog</code></td>
//!           <td>Source</td>
//!           <td>✅</td>
//!           <td rowspan="2">Stable</td>
//!           <td>Fast</td>
//!           <td>-</td>
//!           <td rowspan="2"></td>
//!      </tr>
//!      <tr>
//!           <td>FetchResources</td>
//!           <td>✅</td>
//!           <td>Instant (<1 second)</td>
//!           <td>~491 KB</td>
//!      </tr>
//!      <tr>
//!           <td rowspan="2">RustDist</td>
//!           <td rowspan="2"><code>rust-releases-rust-dist</code></td>
//!           <td>Source</td>
//!          <td>✅</td>
//!           <td rowspan="2">Stable, <strike>Beta & Nightly</strike><sup>To be implemented</sup></td>
//!           <td>Fast</td>
//!           <td>-</td>
//!           <td rowspan="2"></td>
//!      </tr>
//!      <tr>
//!           <td>FetchResources</td>
//!           <td>✅</td>
//!           <td>Medium fast (~20 seconds)</td>
//!           <td>~1 MB</td>
//!      </tr>
//!      <tr>
//!           <td rowspan="2">RustDistWithCLI</td>
//!           <td rowspan="2"><code>rust-releases-rust-dist-with-cli</code></td>
//!           <td>Source</td>
//!           <td>✅</td>
//!           <td rowspan="2">Stable</td>
//!           <td>Fast</td>
//!           <td>-</td>
//!           <td rowspan="2"></td>
//!      </tr>
//!      <tr>
//!           <td>FetchResources</td>
//!           <td>❌</td>
//!           <td>Slow (~1 minute)</td>
//!           <td>~8 MB</td>
//!      </tr>
//! </tbody>
//! </table>
//!
//! <sup>1</sup>: Currently most of the `rust-releases` public API supports only stable. Support for the beta and nightly channel is work-in-progress, and the table currently lists whether there is theoretical support for these channels.<br>
//! <sup>2</sup>: Speed for the `Source` trait primarily consist of parsing speed<br>
//! <sup>3</sup>: Speed for the `FetchResources` trait is primarily limited by your own download speed, and the rate limiting of the server from which the resources are fetched<br>
//! <sup>4</sup>: Approximate as of 2021-03-03 <br>
//!
//!
//! # Issues
//!
//! Feel free to open an issue at our [repository](https://github.com/foresterre/rust-releases/issues)
//! for questions, feature requests, bug fixes, or other points of feedback 🤗.
//!
//! [`FetchResources`]: rust_releases_core::FetchResources
//! [`Source`]: rust_releases_core::Source
//! [`ChannelManifests`]: rust_releases_channel_manifests::ChannelManifests
//! [`RustChangelog`]: rust_releases_rust_changelog::RustChangelog
//! [`RustDist`]: rust_releases_rust_dist::RustDist
//! [`RustDistWithCLI`]: rust_releases_rust_dist_with_cli::RustDistWithCLI
//! [`features`]: https://doc.rust-lang.org/cargo/reference/features.html#features

/// Provides a binary search operation which is intended to be used to search for the lowest required
/// version.
pub mod bisect;
/// Provides an iterator over the latest patch versions for stable releases.
pub mod linear;

// core re-exports
pub use rust_releases_core::{
    semver, Channel, CoreError, CoreResult, FetchResources, Release, ReleaseIndex, Source,
};

#[cfg(feature = "rust-releases-io")]
pub use rust_releases_io::{
    base_cache_dir, is_stale, BaseCacheDirError, CachedClient, CachedClientError, Document,
    IsStaleError, RetrievedDocument, RustReleasesClient,
};

#[cfg(feature = "rust-releases-channel-manifests")]
#[allow(deprecated)]
pub use rust_releases_channel_manifests::{
    ChannelManifests, ChannelManifestsError, ChannelManifestsResult,
};

#[cfg(feature = "rust-releases-rust-changelog")]
pub use rust_releases_rust_changelog::{RustChangelog, RustChangelogError, RustChangelogResult};

#[cfg(feature = "rust-releases-rust-dist")]
pub use rust_releases_rust_dist::{RustDist, RustDistError, RustDistResult};

#[cfg(feature = "rust-releases-rust-dist-with-cli")]
pub use rust_releases_rust_dist_with_cli::{
    RustDistWithCLI, RustDistWithCLIError, RustDistWithCLIResult,
};
