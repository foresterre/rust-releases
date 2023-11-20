//! For the purpose of this library, a _Rust release_, refers to a new version of
//! the Rust programming language. A _Rust distribution_, is a combined package
//! which includes a _Rust toolchain_, consisting of the Rust compiler (`rustc`),
//! and usually several common tools and libraries, like the Rust package
//! manager (`cargo`) and the Rust standard libraries (`alloc`, `core` and `std`).
//!
//! # Channels
//!
//! The Rust project currently produces three types of releases: stable, beta and nightly releases.
//! These are distributed via stable, beta and nightly release channel respectively.
//!
//! _Stable_ and _beta_ releases can be identified by their semver version number.
//!
//! _Nightly_ releases can be identified by their release date.

mod api;
mod comparable_distribution;
mod distribution;
mod register;
mod release;
mod set;
mod storage;

pub use api::*;

#[test]
fn rust_releases() {
    let input = vec![];
    let register = Register::from_iter(input);

    assert_eq!(register.size(), 0);
}
