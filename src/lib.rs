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

    #[test]
    fn pass() {
        let path = [
            env!("CARGO_MANIFEST_DIR"),
            "/resources/stable_2016-04-12.toml",
        ]
        .join("");
        let index = ReleaseIndex::try_from_documents(&vec![DocumentSource::LocalPath(path.into())])
            .unwrap();

        assert!(index.releases().len() > 0);
        assert_eq!(
            index.releases()[0].version(),
            &semver::Version::new(1, 8, 0)
        );
    }
}
