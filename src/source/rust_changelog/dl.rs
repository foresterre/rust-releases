use crate::io::{base_cache_dir, download_if_not_stale};
use crate::source::Document;
use crate::TResult;
use std::time::Duration;

const URL: &str = "https://raw.githubusercontent.com/rust-lang/rust/master/RELEASES.md";
const TIMEOUT: Duration = Duration::from_secs(86_400);
const SOURCE_CACHE_DIR: &str = "source_rust_changelog";

pub(in crate::source::rust_changelog) fn fetch_releases_md() -> TResult<Document> {
    let cache = base_cache_dir()?.join(SOURCE_CACHE_DIR);
    let source = download_if_not_stale(URL, &cache, "RELEASES.md", TIMEOUT)?;

    Ok(source)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dl_test;

    #[test]
    fn test_fetch_meta_manifest() {
        dl_test!({
            let meta = fetch_releases_md();
            assert!(meta.is_ok());
        })
    }
}
