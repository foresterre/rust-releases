use crate::cache::CachedDistClient;
use crate::version;
use rust_releases_core::{Beta, Nightly, Stable};
use std::path::PathBuf;

const MANIFESTS_FOLDER: &str = "manifests";

const STABLE_RELEASES: &str = "stable-releases.txt";

const BETA_RELEASES: &str = "beta-releases.txt";

const NIGHTLY_RELEASES: &str = "nightly-releases.txt";

// Where the cache holds what it read. A release manifest is named after the object it was served
// as, so a cache folder reads like the part of the distribution it holds.
impl<C> CachedDistClient<C> {
    pub fn stable_releases_cache_file(&self) -> PathBuf {
        self.cache_folder().join(STABLE_RELEASES)
    }

    pub fn beta_releases_cache_file(&self) -> PathBuf {
        self.cache_folder().join(BETA_RELEASES)
    }

    pub fn nightly_releases_cache_file(&self) -> PathBuf {
        self.cache_folder().join(NIGHTLY_RELEASES)
    }

    pub fn stable_manifest_cache_file(&self, version: &Stable) -> PathBuf {
        self.manifest_cache_file(format!("channel-rust-{}.toml", version.version))
    }

    pub fn beta_manifest_cache_file(&self, version: &Beta) -> PathBuf {
        self.manifest_cache_file(format!("channel-rust-{}.toml", version::beta_name(version)))
    }

    pub fn nightly_manifest_cache_file(&self, version: &Nightly) -> PathBuf {
        self.manifest_cache_file(format!("channel-rust-nightly-{}.toml", version.date.ymd()))
    }

    fn manifest_cache_file(&self, name: String) -> PathBuf {
        let mut path = self.cache_folder().join(MANIFESTS_FOLDER);
        path.push(name);
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn client() -> CachedDistClient<()> {
        CachedDistClient::new((), PathBuf::from("/cache"))
    }

    #[test]
    fn the_cache_file_of_a_channel() {
        assert_eq!(
            client().stable_releases_cache_file(),
            PathBuf::from("/cache/stable-releases.txt")
        );
        assert_eq!(
            client().beta_releases_cache_file(),
            PathBuf::from("/cache/beta-releases.txt")
        );
        assert_eq!(
            client().nightly_releases_cache_file(),
            PathBuf::from("/cache/nightly-releases.txt")
        );
    }

    #[test]
    fn the_cache_file_of_a_release_manifest() {
        assert_eq!(
            client().stable_manifest_cache_file(&Stable::new(1, 53, 0)),
            PathBuf::from("/cache/manifests/channel-rust-1.53.0.toml")
        );
        assert_eq!(
            client().beta_manifest_cache_file(&Beta::new(1, 75, 0, Some(1))),
            PathBuf::from("/cache/manifests/channel-rust-1.75.0-beta.1.toml")
        );
        assert_eq!(
            client().nightly_manifest_cache_file(&Nightly::new(2016, 3, 8)),
            PathBuf::from("/cache/manifests/channel-rust-nightly-2016-03-08.toml")
        );
    }
}
