//! This crate aims to provide an index of Rust releases, and make it available to Rust programs.
//!
//! ## Introduction
//!
//! The Rust programming language uses deterministic versioning for toolchain releases. Stable versions use SemVer,
//! while nightly, beta and historical builds can be accessed by using dated builds (YY-MM-DD).
//!
//! Unfortunately, a complete index of releases is not available any more. There are however
//! a few places where we can find partial release indices, from which we can build our own
//! index.
//!
//! This process consists of two parts: 1) obtaining the data sources, and 2) building the index
//! from these data sources. For the first part this crate provides the [`FetchResources`] trait, and
//! for the second part the crate provides the [`Source`] trait.
//!
//! These traits are implemented for certain index strategies:
//! 1) [`DistIndex`]: Build an index from the AWS S3 Rust distribution index
//! 2) [`ChannelManifests`]: Build an index from Rust [release manifests](https://static.rust-lang.org/manifests.txt)
//! 3) [`RustChangelog`]: Build an index from the [RELEASES.md](https://raw.githubusercontent.com/rust-lang/rust/master/RELEASES.md) found in the root of the Rust source code repository
//!
//! In the below example, we chose the third source type, and we'll use it to show you how you can
//! use this library.
//!
//! ## Example
//!
//! ```rust
//! use rust_releases::{FetchResources, Source, Channel, ReleaseIndex};
//! use rust_releases::source::RustChangelog;
//!
//! // We choose the RustChangelog source for this example; alternatives are DistIndex and ChannelManifests
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
//! ## Implemented features
//!
//! <table>
//! <thead>
//!      <tr>
//!           <th>Type of data source</th>
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
//!           <td rowspan="2"><code>DistIndex</code></td>
//!           <td>Source</td>
//!           <td>✅</td>
//!           <td rowspan="2">Stable, Beta & Nightly</td>
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
//!      <tr>
//!           <td rowspan="2"><code>ChannelManifests</code></td>
//!           <td>Source</td>
//!           <td>✅</td>
//!           <td rowspan="2">Stable, Beta & Nightly</td>
//!           <td>Medium</td>
//!           <td>-</td>
//!           <td rowspan="2">Once cached, much faster</td>
//!      </tr>
//!      <tr>
//!           <td>FetchResources</td>
//!           <td>✅</td>
//!           <td>Extremely slow (~1 hour)</td>
//!           <td>~418 MB</td>
//!      </tr>
//!      <tr>
//!           <td rowspan="2"><code>RustChangelog</code></td>
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
//! </tbody>
//! </table>
//!
//! <sup>1</sup>: Currently most of the `rust-releases` public API supports only stable. Support for the beta and nightly channel is work-in-progress, and the table currently lists whether there is theoretical support for these channels.<br>
//! <sup>2</sup>: Speed for the `Source` trait primarily consist of parsing speed<br>
//! <sup>3</sup>: Speed for the `FetchResources` trait is primarily limited by your own download speed, and the rate limiting of the server from which the resources are fetched<br>
//! <sup>4</sup>: Approximate as of 2020-03-03 <br>
//!
//!
//! ## Issues
//!
//! Feel free to open an issue at our [repository](https://github.com/foresterre/rust-releases/issues)
//! for questions, feature requests, bug fixes, or other points of feedback 🤗.
//!
//! [`FetchResources`]: crate::FetchResources
//! [`Source`]: crate::Source
//! [`DistIndex`]: crate::source::DistIndex
//! [`ChannelManifests`]: crate::source::ChannelManifests
//! [`RustChangelog`]: crate::source::RustChangelog

pub use crate::channel::Channel;
pub use crate::errors::{RustReleasesError, TResult};
pub use crate::index::{Release, ReleaseIndex};
pub use crate::source::{FetchResources, Source};

pub use semver;

/// See [`Channel`], enumerates the Rust release channels.
///
/// [`Channel`]: crate::channel::Channel
pub mod channel;

/// Top level rust-releases errors
pub mod errors;

/// Module which provides the Rust releases index which is produced by a strategy.
/// See [`ReleaseIndex`] and [`Release`]
///
/// [`ReleaseIndex`]: crate::index::ReleaseIndex
/// [`Release`]: crate::index::Release
pub mod index;

/// i/o related methods used internally.
pub(crate) mod io;

/// Module which contains multiple types of _sources_ and the methods necessary to transform those
/// sources into a `ReleaseIndex`.
pub mod source;

/// Module which contains search strategies, which can be used to go over an index in a certain order
pub mod search;
