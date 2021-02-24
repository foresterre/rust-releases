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
//! for the second part the crate provides the [`Strategy`] trait.
//!
//! These traits are implemented for certain index strategies:
//! 1) [`DistIndex`]: Build an index from the AWS S3 Rust distribution index
//! 2) [`FromManifests`]: Build an index from Rust [release manifests](https://static.rust-lang.org/manifests.txt)
//! 3) [`ReleasesMd`]: Build an index from the [RELEASES.md](https://raw.githubusercontent.com/rust-lang/rust/master/RELEASES.md) found in the root of the Rust source code repository
//!
//! In the below example, we chose the third strategy, and we'll use it to show you how you can
//! use this library.
//!
//! ## Example
//!
//! ```rust
//! use rust_releases::{FetchResources, Strategy, Channel, ReleaseIndex};
//! use rust_releases::strategy::ReleasesMd;
//!
//! // We choose the ReleasesMd strategy for this example; alternatives are DistIndex and FromManifests
//! let strategy = ReleasesMd::fetch_channel(Channel::Stable).unwrap();
//!
//! // Build a release index using our strategy
//! let index = ReleaseIndex::with_strategy(strategy).unwrap();
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
//!   <tr>
//!     <th>strategy name</th>
//!     <th>trait</th>
//!     <th>implemented</th>
//!     <th>notes</th>
//!   </tr>
//! </thead>
//! <tbody>
//!   <tr>
//!     <td rowspan="2">DistIndex</td>
//!     <td>Strategy</td>
//!     <td>✅</td>
//!     <td></td>
//!   </tr>
//!   <tr>
//!     <td>FetchResources</td>
//!     <td>❌</td>
//!     <td>slow (~1 minute)</td>
//!   </tr>
//!   <tr>
//!     <td rowspan="2">FromManifests</td>
//!     <td>Strategy</td>
//!     <td>✅</td>
//!     <td></td>
//!   </tr>
//!   <tr>
//!     <td>FetchResources</td>
//!     <td>✅ </td>
//!     <td>very slow</td>
//!   </tr>
//!   <tr>
//!     <td rowspan="2">ReleasesMd</td>
//!     <td>Strategy</td>
//!     <td>✅</td>
//!     <td></td>
//!   </tr>
//!   <tr>
//!     <td>FetchResources</td>
//!     <td>✅</td>
//!     <td>stable channel only</td>
//!   </tr>
//! </tbody>
//! </table>
//!
//!
//! ## Issues
//!
//! Feel free to open an issue at our [repository](https://github.com/foresterre/rust-releases/issues)
//! for questions, feature requests, bug fixes, or other points of feedback 🤗.
//!
//! [`FetchResources`]: crate::FetchResources
//! [`Strategy`]: crate::Strategy
//! [`DistIndex`]: crate::strategy::DistIndex
//! [`FromManifests`]: crate::strategy::FromManifests
//! [`ReleasesMd`]: crate::strategy::ReleasesMd

pub use crate::channel::Channel;
pub use crate::errors::{RustReleasesError, TResult};
pub use crate::index::{Release, ReleaseIndex};
pub use crate::strategy::{FetchResources, Strategy};

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
pub(crate) mod io;

/// Module which describes input resource files from which the index is built.
pub mod source;

/// Module which contains indexing strategies.
pub mod strategy;
