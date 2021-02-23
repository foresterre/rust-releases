pub use dl::fetch_meta_manifest;
pub use dl::fetch_release_manifests;
pub use dl::DocumentSource;
pub use errors::{RustReleasesError, TResult};
pub use index::Release;
pub use index::ReleaseIndex;
pub use strategy::release_manifests::manifests::ManifestSource;
pub use strategy::release_manifests::manifests::MetaManifest;
pub use strategy::release_manifests::release_channel::Channel;

// public API

mod dl;
mod errors;
mod index;
mod strategy;

#[cfg(test)]
mod tests {
    use yare::parameterized;

    use super::*;

    #[parameterized(
        stable = { "/resources/stable_2016-04-12.toml", "1.8.0" },
        beta = { "/resources/beta_2016-03-23.toml", "1.8.0-beta.2" },
        nightly = { "/resources/nightly_2016-03-08.toml", "1.9.0-nightly" },
    )]
    fn example_usage(resource: &str, expected_version: &str) {
        let expected_version = semver::Version::parse(expected_version).unwrap();

        let path = [env!("CARGO_MANIFEST_DIR"), resource].join("");
        let index = ReleaseIndex::try_from_documents(&vec![DocumentSource::LocalPath(path.into())])
            .unwrap();

        assert!(index.releases().len() > 0);
        assert_eq!(index.releases()[0].version(), &expected_version);
    }
}
