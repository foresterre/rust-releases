# Changelog

## Unreleased

### Added

- Added `RustRelease::version_mut` which returns an exclusive reference to version of a release
- Added `RustRelease::release_date_mut` which returns an option of an exclusive reference to the release date of a release, if set
- Added new `RustRelease::toolchains` which returns a shared reference to the toolchains associated with the release
- Added `RustRelease::toolchains_mut` which returns an exclusive reference to the toolchains associated with the release

### Changed

- Renamed previous `RustRelease::toolchains` to `RustRelease::toolchains_iter` (breaking change)

## 1.1.0 - 2026-05-08

### Maintenance

- Update `rust-toolchain` to `2.0.0`

## 1.0.0 - 2026-05-08

### Added

- Type `RustRelease`

### Notice

- init own CHANGELOG.md for `rust-release` iso shared by `rust-releases` crates
