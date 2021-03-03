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
build a catalog of released Rust versions. In addition, for all solution except `DistIndex`, it is possible to let this crate
[fetch](https://docs.rs/rust-releases/latest/rust_releases/source/trait.FetchResources.html) the required input documents. 

## Implemented options

<table>
<thead>
     <tr>
          <th>Type of data source</th>
          <th>Trait</th>
          <th>Implemented</th>
          <th>Channels<sup>1</sup></th>
          <th>Speed<sup>2, 3</sup></th>
          <th>On disk cache size<sup>4</sup></th>
          <th>Notes</th>
     </tr>
</thead>
<tbody>
     <tr>
          <td rowspan="2"><code>DistIndex</code></td>
          <td>Source</td>
          <td>✅</td>
          <td rowspan="2">Stable, Beta & Nightly</td>
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
          <td rowspan="2"><code>ChannelManifests</code></td>
          <td>Source</td>
          <td>✅</td>
          <td rowspan="2">Stable, Beta & Nightly</td>
          <td>Medium</td>
          <td>-</td>
          <td rowspan="2">Once cached, much faster</td>
     </tr>
     <tr>
          <td>FetchResources</td>
          <td>✅</td>
          <td>Extremely slow (~1 hour)</td>
          <td>~418 MB</td>
     </tr>
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
</tbody>
</table>

<sup>1</sup>: Currently most of the `rust-releases` public API supports only stable. Support for the beta and nightly channel is work-in-progress, and the table currently lists whether there is theoretical support for these channels.<br> 
<sup>2</sup>: Speed for the `Source` trait primarily consist of parsing speed<br> 
<sup>3</sup>: Speed for the `FetchResources` trait is primarily limited by your own download speed, and the rate limiting of the server from which the resources are fetched<br>
<sup>4</sup>: Approximate as of 2020-03-03 <br>

**Which data source should I use?**

Since support for the beta and nightly channels is work-in-progress, I would advise to use the `RustChangelog` data source as it's
a small download, immediately up-to-date on release and fast to parse. It only supports stable channel releases.

The `DistIndex`data source can be useful when eventually support for the beta and nightly channel is completed. For now, this source
also has not integration for fetching the resource files. One way to obtain the input file for this data source is by using the `aws` CLI:
`aws --no-sign-request s3 ls static-rust-lang-org/dist/ > dist.txt`<sup>(<a href="https://github.com/rust-lang/rust/issues/56971#issuecomment-527199391">source</a>)</sup> this input resource by using the aws.

The `ChannelManifests` source is the most complete, and we can most easily extend information about releases beyond the version
for this source. Once downloaded once, it's quite fast since each release manifest is cached as a separate file. 
However, the initial download is quite large and frankly, because of rate limiting, also quite slow. In addition,
the resource is approximately one-week out of date since the root manifest is only updated one week after a release <sup>(<a href="https://github.com/rust-lang/rust/issues/56971#issuecomment-527199391">source</a>)</sup>.

## Technical options (eventually)

* Bring your own download tool (planned, will be a cfg option in the future)
* Optionally, use built in download tool

## Applications

[cargo-msrv](https://github.com/foresterre/cargo-msrv) is a tool which can be used to determine the minimal supported Rust version (MSRV).
In cargo-msrv I started by parsing the latest channel manifest, and then decreasing the minor semver version.

This is not great for many reasons:
* Except for the latest released version, we are left guessing the decreased version numbers
  actually exist
* Only stable versions are supported, not nightly, beta, or other channels
* Only 1.x.0 versions are supported

This is not ideal, thus `rust-releases` was born. Now cargo-msrv can iterate over Rust releases of which we know they exist and are available.