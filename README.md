# rust-releases

[![ci-msrv](https://github.com/foresterre/rust-releases/actions/workflows/msrv.yml/badge.svg)](https://github.com/foresterre/rust-releases/actions/workflows/msrv.yml)
[![Crates.io version shield](https://img.shields.io/crates/v/rust-releases.svg)](https://crates.io/crates/rust-releases)
[![Docs](https://docs.rs/rust-releases/badge.svg)](https://docs.rs/rust-releases)
[![Crates.io license shield](https://img.shields.io/crates/l/rust-releases.svg)](https://crates.io/crates/rust-releases)

`*` When unreleased, MSRV subject to change  

## Introduction

The Rust programming language uses deterministic versioning for toolchain releases. Stable versions use SemVer, 
while nightly, beta and historical builds can be accessed by using dated builds (YY-MM-DD).

Unfortunately, a clean index of releases is not available any more. I decided to research which resources where still available
and found the following solutions:
    
  1) Use the AWS index <sup>(<a href="https://github.com/rust-lang/rust/issues/56971#issuecomment-527199391">source</a>)</sup>
  2) Build from individual [release manifests](https://static.rust-lang.org/manifests.txt) <sup>(<a href="https://github.com/rust-lang/rust/issues/56971#issuecomment-527199391">source</a>)</sup>
  3) Parse Rust in-repo [RELEASES.md](https://raw.githubusercontent.com/rust-lang/rust/master/RELEASES.md)
  4) (new) Query the [GitHub releases](https://api.github.com/repos/rust-lang/rust/releases) of the Rust repository

Each of these options requires additional parsing, which is where this crate comes in: the `rust-releases` crate
can obtain and parse the above resources, and hand you the releases of the stable, beta and nightly channel.

Each data source lives in its own crate, and can be enabled as a feature of `rust-releases`. Sources which obtain
their data over the network provide a client with fetch methods for each channel they support. These methods return
the set of released Rust versions of that channel.

NB: Some sources can have some missing or incorrect data points. This can in some instances be fixed by merging
two or more data sources (although you may want to consider a more accurate source, if your application needs it).
For example, the `rust-releases-github` source has some incorrect release dates for early Rust versions, because
the GitHub releases were created on a much later date, and the client of `rust-releases-github` has no way (without
combining with other data sources) to get the actual release date, so it takes the release date from the GitHub release,
which are thus incorrect for the first few because they were created in a batch once the Rust project started using
the GitHub releases feature (it may even have not been available in 2015 when Rust 1.0.0 was released, not sure anymore).

## Implemented options

**Which data source should I use?**

If you only need stable releases, I would advise to use the `RustChangelog` data source as it's a small download,
immediately up-to-date on release and fast to parse. `GithubReleases` is an alternative for stable releases, which
queries the GitHub releases API of the `rust-lang/rust` repository.

For the beta and nightly channels (it also has stable of course), use the `RustDist` data source. It enumerates the
Rust AWS S3 distribution bucket, and can additionally provide the details of a release from its channel manifest.

If you rather not do any network requests at all, the `BundledReleases` data source ships the releases of the stable,
beta and nightly channel as generated Rust code. It's generated using `RustDist`, and since its bundled, it's only
up-to-date up to the moment it was generated.

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
