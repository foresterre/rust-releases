use crate::RustChangelogResult;
use rust_releases_io::base_cache_dir;
use rust_releases_io::{CachedClient, Document, ResourceFile, RustReleasesClient};
use std::time::Duration;

const URL: &str = "https://raw.githubusercontent.com/rust-lang/rust/master/RELEASES.md";
const TIMEOUT: Duration = Duration::from_secs(86_400);
const SOURCE_CACHE_DIR: &str = "source_rust_changelog";
const RESOURCE_NAME: &str = "RELEASES.md";

pub(crate) fn fetch() -> RustChangelogResult<Document> {
    let cache = base_cache_dir()?.join(SOURCE_CACHE_DIR);

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
            let meta = fetch();
            assert!(meta.is_ok());
        })
    }
}
