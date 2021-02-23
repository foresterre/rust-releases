use crate::{RustReleasesError, TResult};
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
        .ok_or(RustReleasesError::RustVersionNotFoundInManifest)?;

    Ok(semver::Version::parse(version)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strategy::from_manifests::DocumentSource;

    #[test]
    fn test_parse_meta_manifest() {
        let path = [
            env!("CARGO_MANIFEST_DIR"),
            "/resources/stable_2016-04-12.toml",
        ]
        .join("");
        let release_manifest = DocumentSource::LocalPath(path.into());

        let buffer = release_manifest.load().unwrap();
        let version = parse_release_manifest(&buffer);
        assert_eq!(version.unwrap(), semver::Version::parse("1.8.0").unwrap());
    }
}
