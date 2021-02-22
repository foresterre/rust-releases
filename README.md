# rust-releases


## Introduction

The Rust programming language uses deterministic versioning for toolchain releases. Stable versions use SemVer, 
while nightly, beta and historical builds can be accessed by using dated builds (YY-MM-DD).

[cargo-msrv](https://github.com/foresterre/cargo-msrv) is a tool which can be used to determine the minimal supported Rust version (MSRV).
In cargo-msrv I started by parsing the latest channel manifest, and then decreasing the minor semver version.
This is not great for many reasons:
  * Except for the latest released version, we are left guessing the decreased version numbers
actually exist
  * Only stable versions are supported, not nightly, beta, or other channels
  * Only 1.x.0 versions are supported

As a result of the above limitations, I decided to look for an actual index of releases. After doing some research I found
the following options:
    
  1) Use the AWS index (e.g. `aws --no-sign-request s3 ls static-rust-lang-org/dist/ > dist.txt`)
      * Rate-limited (only obtaining the index took ~40 seconds)
      * [source](https://github.com/rust-lang/rust/issues/56971#issuecomment-527199391)
  2) Build from individual [release manifests](https://static.rust-lang.org/manifests.txt)
      * Requires parsing multiple documents
      * Approx. one week delay after a new release
      * Also has more specific toolchain information 
      * [source](https://github.com/rust-lang/rust/issues/56971#issuecomment-532783994)
  3) Parse Rust in-repo [RELEASES.md](https://raw.githubusercontent.com/rust-lang/rust/master/RELEASES.md)
      * Note: stable only

Each of these options requires additional parsing, which is where this crate comes in: this crate provides an index of all Rust releases.
It will eventually support all three options, but initially, only the second one will be supported.

## Technical options

* Bring your own download tool (planned, will be a cfg option in the future)
* Optionally, use built in download tool