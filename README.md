# rust-releases

[![GitHub Actions: CI](https://github.com/foresterre/rust-releases/actions/workflows/ci.yml/badge.svg)](https://github.com/foresterre/rust-releases/actions/workflows/ci.yml)
[![Crates.io version shield](https://img.shields.io/crates/v/rust-releases.svg)](https://crates.io/crates/rust-releases)
[![Docs](https://docs.rs/rust-releases/badge.svg)](https://docs.rs/rust-releases)
[![Crates.io license shield](https://img.shields.io/crates/l/rust-releases.svg)](https://crates.io/crates/rust-releases)

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
build a catalog of released Rust versions. In addition, for all solution except `RustDistWithCLI`, it is possible to let this crate
[fetch](https://docs.rs/rust-releases/latest/rust_releases/source/trait.FetchResources.html) the required input documents. 

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
          <td rowspan="2"></td>
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
     <tr>
          <td rowspan="2"><code>RustDistWithCLI</code></td>
          <td>Source</td>
          <td>✅</td>
          <td rowspan="2">Stable, <strike>Beta & Nightly</strike><sup>To be implemented</sup></td>
          <td>Fast</td>
          <td>-</td>
          <td rowspan="2"></td>
     </tr>
     <tr>
          <td>FetchResources</td>
          <td>❌</td>
          <td>Slow (~1 minute)</td>
          <td>~8 MB</td>
     </tr>
     <tr>
          <td rowspan="2"><code><strike>ChannelManifests</strike></code><sup>Deprecated</sup></td>
          <td>Source</td>
          <td>✅</td>
          <td rowspan="2">Stable, <strike>Beta & Nightly</strike><sup>Won't be implemented</sup></td>
          <td>Medium</td>
          <td>-</td>
          <td rowspan="2">Input data not updated since 2020-02-23<sup>(<a href="https://github.com/foresterre/rust-releases/issues/9">#9</a>)</td>
     </tr>
     <tr>
          <td>FetchResources</td>
          <td>✅</td>
          <td>Extremely slow (~1 hour)</td>
          <td>~418 MB</td>
     </tr>
</tbody>
</table>

<sup>1</sup>: Currently most of the `rust-releases` public API supports only stable. Support for the beta and nightly channel is work-in-progress, and the table currently lists whether there is theoretical support for these channels.<br> 
<sup>2</sup>: Speed for the `Source` trait primarily consist of parsing speed<br> 
<sup>3</sup>: Speed for the `FetchResources` trait is primarily limited by your own download speed, and the rate limiting of the server from which the resources are fetched<br>
<sup>4</sup>: Approximate as of 2021-03-03 <br>

**Which data source should I use?**

Since support for the beta and nightly channels is work-in-progress, I would advise to use the `RustChangelog` data source as it's
a small download, immediately up-to-date on release and fast to parse. It only supports stable channel releases.

Alternatively, the `RustDist` or `RustDistWithCLI` data sources can be useful, especially when support for the beta and nightly channel are added.
They both get their input data from the Rust AWS S3 distribution bucket. When using `RustDist`, the input data can be obtained
with the `FetchResources` trait implementation. For `RustDistWithCLI`, you have to obtain the input data yourself (by running the
`aws` cli with the following options `aws --no-sign-request s3 ls static-rust-lang-org/dist/ > dist.txt`<sup>(<a href="https://github.com/rust-lang/rust/issues/56971#issuecomment-527199391">source</a>)</sup>).

You should **not** use the `ChannelManifests` source, unless you have a good reason to do so. This source had a lot of potential, as the input data is the most complete (although with a bit of extra work we can get the same data with `RustDist`). 
With the published channel manifests, we could easily extend information about releases beyond the release version. The separate manifest files could be parsed rather fast, and
new manifests can be downloaded iteratively.
There were however also major downsides. The initial download is quite large, and slow (because of rate limiting), in the order of hours, and,
the resource is approximately one-week out of date since the root manifest is only updated one week after a release <sup>(<a href="https://github.com/rust-lang/rust/issues/56971#issuecomment-527199391">source</a>)</sup>.
Most importantly however, the input data has not been updated since 2020-02-23<sup>(<a href="https://github.com/foresterre/rust-releases/issues/9">#9</a>)</sup>. As a result, this source has been deprecated, and will not be further extended.

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