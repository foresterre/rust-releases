use crate::{ManifestaError, TResult};
use serde::Deserialize;

#[derive(Deserialize)]
struct Manifest {
    pkg: Pkg,
}
#[derive(Deserialize)]
struct Pkg {
    rust: Rust,
}

#[derive(Deserialize)]
struct Rust {
    version: String,
}

pub(crate) fn parse_release_manifest(manifest_contents: &[u8]) -> TResult<semver::Version> {
    let parsed: Manifest = toml::from_slice(manifest_contents)?;

    let version = parsed
        .pkg
        .rust
        .version
        .split_ascii_whitespace()
        .next()
        .ok_or(ManifestaError::RustVersionNotFoundInManifest)?;

    Ok(semver::Version::parse(version)?)
}
