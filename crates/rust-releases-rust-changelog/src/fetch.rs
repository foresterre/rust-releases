use crate::RustChangelogResult;
use rust_releases_io::Document;
use rust_releases_io::{base_cache_dir, download_if_not_stale};
use std::time::Duration;

const URL: &str = "https://raw.githubusercontent.com/rust-lang/rust/master/RELEASES.md";
const TIMEOUT: Duration = Duration::from_secs(86_400);
const SOURCE_CACHE_DIR: &str = "source_rust_changelog";

pub(crate) fn fetch() -> RustChangelogResult<Document> {
    let cache = base_cache_dir()?.join(SOURCE_CACHE_DIR);
    let source = download_if_not_stale(URL, &cache, "RELEASES.md", TIMEOUT)?;

    Ok(source)
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
