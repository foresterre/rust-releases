pub use channel::Channel;
pub use errors::{RustReleasesError, TResult};
pub use index::Release;
pub use index::ReleaseIndex;

// public API

mod channel;
mod errors;
mod index;
mod source;
mod strategy;

#[cfg(test)]
mod tests {
    use yare::parameterized;

    use crate::strategy::from_manifests::FromManifests;

    use super::*;
    use crate::source::DocumentSource;

    #[parameterized(
        stable = { "/resources/stable_2016-04-12.toml", "1.8.0" },
        beta = { "/resources/beta_2016-03-23.toml", "1.8.0-beta.2" },
        nightly = { "/resources/nightly_2016-03-08.toml", "1.9.0-nightly" },
    )]
    fn example_usage(resource: &str, expected_version: &str) {
        let expected_version = semver::Version::parse(expected_version).unwrap();

        let path = [env!("CARGO_MANIFEST_DIR"), resource].join("");
        let strategy = FromManifests::from_documents(vec![DocumentSource::LocalPath(path.into())]);
        let index = ReleaseIndex::with_strategy(strategy).unwrap();

        assert!(!index.releases().is_empty());
        assert_eq!(index.releases()[0].version(), &expected_version);
    }
}
