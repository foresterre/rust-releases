use crate::io::{base_cache_dir, is_stale};
use crate::source::rust_dist::RustDistError;
use crate::source::Document;
use crate::TResult;
use rusoto_core::credential::{AwsCredentials, StaticProvider};
use rusoto_core::{HttpClient, Region};
use rusoto_s3::{ListObjectsV2Request, S3Client, S3};
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::time::Duration;

// Rust currently always uses the US West 1 bucket
const RUST_DIST_REGION: Region = Region::UsWest1;

// The bucket from which the official Rust sources are distributed
const RUST_DIST_BUCKET: &str = "static-rust-lang-org";

// We only request objects which start with the following string, which currently only matches stable
// releases
const OBJECT_PREFIX: &str = "dist/rustc-";

// The output file path
const OUTPUT_PATH: &str = "dist_static-rust-lang-org.txt";

// Use the filtered index cache for up to 1 day
const TIMEOUT: Duration = Duration::from_secs(86_400);

// Directory where cached files reside for this source
const SOURCE_CACHE_DIR: &str = "source_dist_index";

pub(in crate::source::rust_dist) fn fetch() -> TResult<Document> {
    let cache_dir = base_cache_dir()?.join(SOURCE_CACHE_DIR);
    let output_path = cache_dir.join(OUTPUT_PATH);

    // Use the locally cached version if it exists, and is not stale
    if output_path.exists() && !is_stale(&output_path, TIMEOUT)? {
        return Ok(Document::LocalPath(output_path));
    } else {
        std::fs::create_dir_all(cache_dir)?;
    }

    // Create a new HTTP Client
    let mut http_client = HttpClient::new().map_err(RustDistError::from)?;

    // Append our user agent, so the Rust maintainers can see where these requests originate from.
    http_client.local_agent_append(
        "rust-releases (github.com/foresterre/rust-releases/issues)".to_string(),
    );

    // Create an S3 client, with the appropriate region and anonymous credentials.
    let s3 = S3Client::new_with(
        http_client,
        StaticProvider::from(AwsCredentials::default()),
        RUST_DIST_REGION,
    );

    // Setup a Tokio runtime for the Rusoto S3 client.
    let rt = tokio::runtime::Runtime::new()?;

    // Create a new output file, if it does not exist yet.
    let mut file = BufWriter::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&output_path)?,
    );

    // The S3 client returns a conceptual page of up to 1000 results in 1 API call.
    // This is the last key returned by the page of results.
    let mut last_key = None;

    // The amount of results we want per API call.
    // NB: for some reason, rusoto_s3 uses i64 as type for max_keys instead of an unsigned integer
    const PAGE_SIZE: i64 = 1000;

    loop {
        // Perform the API call
        let objects = rt.block_on(s3.list_objects_v2(ListObjectsV2Request {
            bucket: RUST_DIST_BUCKET.to_string(),
            start_after: last_key.to_owned(),
            max_keys: Some(PAGE_SIZE),
            prefix: Some(OBJECT_PREFIX.to_owned()),
            ..Default::default()
        }));

        if let Ok(output) = objects {
            if let Some(objects) = output.contents {
                last_key = write_objects(&mut file, &objects);

                // Stop if there are no more relevant objects.
                if objects.len() < PAGE_SIZE as usize {
                    break;
                }
            }
        }

        // Also stop if there is nothing to process.
        if last_key.is_none() {
            break;
        }
    }

    // FIXME: fix caching (save to memory and as a local file, like the rust_changelog source)
    Ok(Document::LocalPath(output_path))
}

fn write_objects(
    buffer: &mut impl std::io::Write,
    objects: &[rusoto_s3::Object],
) -> Option<String> {
    for object in objects {
        let key = object.key.clone().unwrap_or_else(|| "".to_string());

        let _ = buffer.write(format!("{}\n", key).as_bytes());
    }

    let _ = buffer.flush();

    // return the last detected key
    objects
        .last()
        .and_then(|obj| obj.key.as_ref().map(|o| o.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dl_test;

    // @runWith cargo test --all-features --package rust-releases --lib source::rust_dist_with_cli::dl::tests::test_fetch_meta_manifest -- --exact
    #[test]
    fn test_fetch_meta_manifest() {
        dl_test!({
            let meta = fetch();
            assert!(meta.is_ok());
        })
    }
}
