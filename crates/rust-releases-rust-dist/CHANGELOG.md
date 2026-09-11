# Changelog

## Unreleased

## 0.34.0 - 2026-09-11

## 0.34.0 - 2026-09-11

## 0.34.0 - 2026-09-11

### Added

- Add fetching og the beta and nightly release channels
- Add `ReleaseManifest`, which describes a release manifest, and can be used to extend a `RustRelease` with extra fields

### Changed

- Rewrite the rust-releases-rust-dist crate to sync + async client with optional caching
- Replace `DistIndexClient` with `DistClient`, whose methods are the actions the crate performs rather than the objects it downloads
- Collect stable releases from the release manifests of the bucket, and only use release artifacts for releases before 1.8.0 (which do not have these manifests)

### Removed

- Remove `RustDistError` and `DistIndexError`

## 0.33.0 - 2026-05-08

### Maintenance

- Updated `aws-sdk-s3` to `1.132.0`

## 0.32.0 - 2026-03-25

### Fixed

- Updated `aws-sdk-s3` (fixes security vulnerability)
