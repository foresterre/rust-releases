# rust-releases

[![ci-msrv](https://github.com/foresterre/rust-releases/actions/workflows/msrv.yml/badge.svg)](https://github.com/foresterre/rust-releases/actions/workflows/msrv.yml)
[![Crates.io version shield](https://img.shields.io/crates/v/rust-releases.svg)](https://crates.io/crates/rust-releases)
[![Docs](https://docs.rs/rust-releases/badge.svg)](https://docs.rs/rust-releases)
[![Crates.io license shield](https://img.shields.io/crates/l/rust-releases.svg)](https://crates.io/crates/rust-releases)
[![MSRV shield](https://img.shields.io/badge/MSRV-1.53.0-informational)](https://github.com/foresterre/cargo-msrv)

| rust-releases version | MSRV |
|-----------------------|------|
| 0.21.1                | 1.51 |
| 0.22.0                | 1.53 |
| ~~0.23.0~~            | 1.63 |
| 0.24.0                | 1.63 |
| 0.25.0                | 1.63 |
| 0.26.0                | 1.63 |
| 0.27.0                | 1.67 |
| 0.28.0                | 1.68 |
| 0.29.0                | 1.78 |

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

Each data source implements the [Source](https://docs.rs/rust-releases/latest/rust_releases/source/trait.Source.html) trait.  `Source` provides a `build_index` method, which can be used to
build a catalog of released Rust versions.

## Implemented options

<table>
<thead>
     <tr>
          <th>Type of data source</th>
          <th>Trait</th>
          <th>Available</th>
          <th>Channels<sup>1</sup></th>
          <th>Speed<sup>2, 3</sup></th>
          <th>On disk cache size<sup>4</sup></th>
          <th>Notes</th>
     </tr>
</thead>
<tbody>
     <tr>
          <td rowspan="2"><code>RustChangelog</code></td>
          <td>Source</td>
          <td>✅</td>
          <td rowspan="2">Stable</td>
          <td>Fast</td>
          <td>-</td>
          <td rowspan="2"><i>Enabled by default. Disable by setting <code>default-features = false</code> for the <code>rust-releases</code> dependency in your Cargo.toml manifest.</i></td>
     </tr>
     <tr>
          <td>FetchResources</td>
          <td>✅</td>
          <td>Instant (<1 second)</td>
          <td>~491 KB</td>
     </tr>
     <tr>
          <td rowspan="2"><code>RustDist</code></td>
          <td>Source</td>
          <td>✅</td>
          <td rowspan="2">Stable, <strike>Beta & Nightly</strike><sup>To be implemented</sup></td>
          <td>Fast</td>
          <td>-</td>
          <td rowspan="2"></td>
     </tr>
     <tr>
          <td>FetchResources</td>
          <td>✅</td>
          <td>Medium fast (~10 seconds)</td>
          <td>~1 MB</td>
     </tr>
</tbody>
</table>

<sup>1</sup>: Currently most of the `rust-releases` public API supports only stable. Support for the beta and nightly channel is work-in-progress, and the table currently lists whether there is theoretical support for these channels.<br> 
<sup>2</sup>: Speed for the `Source` trait primarily consist of parsing speed<br> 
<sup>3</sup>: Speed for the `FetchResources` trait is primarily limited by your own download speed, and the rate limiting of the server from which the resources are fetched<br>
<sup>4</sup>: Approximate as of 2021-03-03<br>
<sup>5</sup>: While the channel manifests are the most complete source available, they're practically too slow to download without
adding some incremental implementation first, while this would still require a large initial download. Since
we currently do not use most of the data available in the manifest files, it's usually better to instead pick
a different source. It's more likely we'll take the channel manifests, take a subset and compile it into a smaller source type.<br>

**Which data source should I use?**

Since support for the beta and nightly channels is work-in-progress, I would advise to use the `RustChangelog` data source as it's
a small download, immediately up-to-date on release and fast to parse. It only supports stable channel releases.

Alternatively, the `RustDist` or `RustDistWithCLI` data sources can be useful, especially when support for the beta and nightly channel are added.
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
