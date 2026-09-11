# Changelog

## Unreleased

## 0.34.0 - 2026-09-11

## 0.34.0 - 2026-09-11

## 0.34.0 - 2026-09-11

### Added

- Added convenience function `empty` to instantiate an empty `StableReleases`, `BetaReleases`, `NightlyReleases` instance with a context `C = ()`
- Implement `Clone` for `StableReleases`, `BetaReleases` and `NightlyReleases`
- Implement `FromIterator` for `StableReleases`, `BetaReleases` and `NightlyReleases`
- Implement `IntoIterator` for `StableReleases`, `BetaReleases` and `NightlyReleases`
- Implement `PartialEq` for `StableReleases`, `BetaReleases` and `NightlyReleases`
- Added `map`, `map_ver sion`, `map_release_date`, `map_toolchains`, and `map_context` to `StableReleases`, `BetaReleases` and `NightlyReleases`
- Added constructor `new` to instantiate a `StableReleases`, `BetaReleases` or `NightlyReleases` instance from an iterator of releases

### Changed

- Renamed `ContextMerge` to `MergeContext` for consistency with `MergeReleaseDate` amd `MergeToolchains`

## 0.33.0 - 2026-05-08

### Maintenance

- Update `rust-release` to `1.1.0`

## 0.32.0 - 2026-05-08

### Notice

- init own CHANGELOG.md for rust-releases-core iso shared by `rust-releases` crates
