use crate::{ChannelManifestsError, ChannelManifestsResult};
use rust_releases_core::semver;
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

pub(crate) fn parse_release_manifest(
    manifest_contents: &[u8],
) -> ChannelManifestsResult<semver::Version> {
    let parsed: Manifest =
        toml::from_slice(manifest_contents).map_err(ChannelManifestsError::DeserializeToml)?;

    let version = parsed
        .pkg
        .rust
        .version
        .split_ascii_whitespace()
        .next()
        .ok_or(ChannelManifestsError::RustVersionNotFoundInManifest)?;

    Ok(semver::Version::parse(version)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_releases_io::Document;

    #[test]
    fn test_parse_meta_manifest() {
        let path = [
            env!("CARGO_MANIFEST_DIR"),
            "/../../resources/channel_manifests/stable_2016-04-12.toml",
        ]
        .join("");
        let release_manifest = Document::LocalPath(path.into());

        let buffer = release_manifest.load().unwrap();
        let version = parse_release_manifest(&buffer);
        assert_eq!(version.unwrap(), semver::Version::parse("1.8.0").unwrap());
    }
}
