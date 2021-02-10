pub use errors::{ManifestaError, TResult};

// public API

pub use dl::fetch_meta_manifest;
pub use dl::fetch_release_manifests;
pub use dl::DocumentSource;
pub use index::Release;
pub use index::ReleaseIndex;
pub use manifests::ManifestSource;
pub use manifests::MetaManifest;
pub use release_channel::Channel;

mod dl;
mod errors;
mod index;
mod manifests;
mod release_channel;
mod release_manifest;

#[cfg(test)]
mod tests {
    use super::*;
    use yare::parameterized;

    #[parameterized(
        stable = { "/resources/stable_2016-04-12.toml", "1.8.0" },
        beta = { "/resources/beta_2016-03-23.toml", "1.8.0-beta.2" },
        nightly = { "/resources/nightly_2016-03-08.toml", "1.9.0-nightly" },
    )]
    fn pass(resource: &str, expected_version: &str) {
        let expected_version = semver::Version::parse(expected_version).unwrap();

        let path = [env!("CARGO_MANIFEST_DIR"), resource].join("");
        let index = ReleaseIndex::try_from_documents(&vec![DocumentSource::LocalPath(path.into())])
            .unwrap();

        assert!(index.releases().len() > 0);
        assert_eq!(index.releases()[0].version(), &expected_version);
    }
}
