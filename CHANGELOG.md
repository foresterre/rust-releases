# Changelog

## [Unreleased]

[Unreleased]: https://github.com/foresterre/rust-releases

## [0.23.0]

### Added

* **rust-releases-io** Added `RustReleasesClient` trait.
* **rust-releases-io** Added `CachedClient`, which implements `RustReleasesClient`, a replacement for the `download_if_not_stale` function.
* **rust-releases-io** Added `RetrievedDocument` and `RetrievalLocation`, to replace the function of the `Document` variants.

### Changed

*  **rust-releases-io** `Document` is now a wrapper for a byte buffer, and no longer has variants.
*  **rust-releases-io** Split top level `IoError` into separate errors: `BaseCacheDirError`, `IsStaleError` and `CachedClientError`.
*  **(all crates)** Updated for compatibility with new **rust-releases-io** types, where necessary.
*  **(all crates)** MSRV is now 1.63

### Removed

* **rust-releases-io** Removed `download_if_not_stale`
* **rust-releases-io** Removed `IoError`

[0.23.0]: https://github.com/foresterre/rust-releases/releases/tag/v0.23.0
