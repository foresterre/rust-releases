# rust-releases

[![ci-msrv](https://github.com/foresterre/rust-releases/actions/workflows/msrv.yml/badge.svg)](https://github.com/foresterre/rust-releases/actions/workflows/msrv.yml)
[![Crates.io version shield](https://img.shields.io/crates/v/rust-releases.svg)](https://crates.io/crates/rust-releases)
[![Docs](https://docs.rs/rust-releases/badge.svg)](https://docs.rs/rust-releases)
[![Crates.io license shield](https://img.shields.io/crates/l/rust-releases.svg)](https://crates.io/crates/rust-releases)
[![MSRV shield](https://img.shields.io/badge/MSRV-1.53.0-informational)](https://github.com/foresterre/cargo-msrv)

`*` When unreleased, MSRV subject to change  

## Introduction

The Rust programming language uses deterministic versioning for toolchain releases. Stable versions use SemVer, 
while nightly, beta and historical builds can be accessed by using dated builds (YY-MM-DD).

Unfortunately, a clean index of releases is not available any more. I decided to research which resources where still available
and found the following solutions:
    
  1) Use the AWS index <sup>(<a href="https://github.com/rust-lang/rust/issues/56971#issuecomment-527199391">source</a>)</sup>
  2) Build from individual [release manifests](https://static.rust-lang.org/manifests.txt) <sup>(<a href="https://github.com/rust-lang/rust/issues/56971#issuecomment-527199391">source</a>)</sup>
  3) Parse Rust in-repo [RELEASES.md](https://raw.githubusercontent.com/rust-lang/rust/master/RELEASES.md)

Each of these options requires additional parsing, which is where this crate comes in: the `rust-releases` crate
can obtain, parse and build an index from the above resources. This crate also provides methods to iterate over versions 
in a _linear_ fashion, or by using a _bisect_ binary search strategy.

Each data source implements the [Source](https://docs.rs/rust-releases/latest/rust_releases/source/trait.Source.html) trait.  `Source` provides a `build` method, which can be used to
build a catalog of released Rust versions.

## Implemented options

**Which data source should I use?**

Since support for the beta and nightly channels is work-in-progress, I would advise to use the `RustChangelog` data source as it's
a small download, immediately up-to-date on release and fast to parse. It only supports stable channel releases.

Alternatively, the `RustDist` data source can be useful, especially when support for the beta and nightly channel are added.
They both get their input data from the Rust AWS S3 distribution bucket. When using `RustDist`, the input data can be obtained
with the `FetchResources` trait implementation. For `RustDistWithCLI`, you have to obtain the input data yourself (by running the
`aws` cli with the following options `aws --no-sign-request s3 ls static-rust-lang-org/dist/ > dist.txt`<sup>(<a href="https://github.com/rust-lang/rust/issues/56971#issuecomment-527199391">source</a>)</sup>).

## Applications

[cargo-msrv](https://github.com/foresterre/cargo-msrv) is a tool which can be used to determine the minimal supported Rust version (MSRV).
It builds your Rust crate and checks whether the build succeeds or fails, as this gives the most complete idea whether a version will work
with your (external) dependencies. `cargo-msrv` uses bisection, or a reverse-linear search, to find the lowest appropriate Rust version.
For this, it needs to have an idea about the toolchains which have been released, and can be installed.

Originally we simply parsed the latest channel manifest, and then decreased the minor semver version, but this was obviously not great for many reasons, including:
* Except for the latest released version, we are left guessing the decreased version numbers
  actually exist
* Only stable versions were supported, not nightly, beta, or other channels
* Only 1.x.0 versions were supported

This was not ideal, thus `rust-releases` was born. Now cargo-msrv can iterate over Rust releases of which we know they exist and are available.
