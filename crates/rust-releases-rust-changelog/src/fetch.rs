use crate::RustChangelogResult;
use rust_releases_io::base_cache_dir;
use rust_releases_io::{CachedClient, Document, ResourceFile, RustReleasesClient};
use std::path::Path;
use std::time::Duration;

const URL: &str = "https://raw.githubusercontent.com/rust-lang/rust/master/RELEASES.md";
const TIMEOUT: Duration = Duration::from_secs(86_400);
const SOURCE_CACHE_DIR: &str = "source_rust_changelog";
const RESOURCE_NAME: &str = "RELEASES.md";

pub fn fetch(cache_dir: Option<impl AsRef<Path>>) -> RustChangelogResult<Document> {
    let cache = if let Some(cache_dir) = cache_dir {
        cache_dir.as_ref().join(SOURCE_CACHE_DIR)
    } else {
        base_cache_dir()?.join(SOURCE_CACHE_DIR)
    };

    let client = CachedClient::new(cache, TIMEOUT);
    let source = client.fetch(ResourceFile::new(URL, RESOURCE_NAME))?;

    Ok(source.into_document())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_meta_manifest() {
        __internal_dl_test!({
            let meta = fetch(None);
            assert!(meta.is_ok());
        })
    }
}
