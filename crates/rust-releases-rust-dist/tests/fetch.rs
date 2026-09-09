use rust_releases_core::Stable;
use rust_releases_io::{BoxFuture, Document};
use rust_releases_rust_dist::{
    AsyncDistIndexClient, CachedDistIndexClient, CachedDistIndexError, DistIndexClient,
    DistIndexError, RustDist, RustDistError,
};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[test]
fn fetch_the_stable_releases_of_rust() {
    let bucket = FakeDistBucket::new();
    bucket.serve_index("dist_static-rust-lang-org.txt");

    let source = RustDist::new(bucket.clone());

    let releases = source.fetch().unwrap();

    // 74 releases including minor releases from 1.0.0 to 1.53.0
    assert_eq!(releases.len(), 74);
    assert_eq!(
        releases.iter().next().unwrap().version(),
        &Stable::new(1, 0, 0)
    );
    assert_eq!(
        releases.iter().last().unwrap().version(),
        &Stable::new(1, 53, 0)
    );

    assert_eq!(bucket.downloads(), 1);
}

#[tokio::test]
async fn fetch_the_stable_releases_of_rust_asynchronously() {
    let bucket = FakeDistBucket::new();
    bucket.serve_index("dist_static-rust-lang-org.txt");

    let source = RustDist::new(bucket.clone());

    let releases = source.fetch_async().await.unwrap();

    assert_eq!(releases.len(), 74);
    assert_eq!(
        releases.iter().last().unwrap().version(),
        &Stable::new(1, 53, 0)
    );
}

#[test]
fn fetch_the_stable_releases_of_rust_through_a_cache() {
    let cache_file = scratch_cache_file("cached-fetch");

    let bucket = FakeDistBucket::new();
    bucket.serve_index("dist_static-rust-lang-org.txt");

    let client = CachedDistIndexClient::new(
        bucket.clone(),
        cache_file.clone(),
        Duration::from_secs(86_400),
    );
    let source = RustDist::new(client);

    let first = source.fetch().unwrap();
    let second = source.fetch().unwrap();

    assert_eq!(first, second);
    assert_eq!(bucket.downloads(), 1);
    assert!(cache_file.is_file());

    std::fs::remove_dir_all(cache_file.parent().unwrap()).unwrap();
}

#[test]
fn a_stale_cache_is_downloaded_again() {
    let cache_file = scratch_cache_file("stale-cache");

    let bucket = FakeDistBucket::new();
    bucket.serve_keys("dist/rustc-1.53.0-x86_64-apple-darwin.tar.gz\n");

    let client =
        CachedDistIndexClient::new(bucket.clone(), cache_file.clone(), Duration::from_secs(0));
    let source = RustDist::new(client);

    source.fetch().unwrap();
    source.fetch().unwrap();

    assert_eq!(bucket.downloads(), 2);

    std::fs::remove_dir_all(cache_file.parent().unwrap()).unwrap();
}

#[test]
fn every_artifact_of_a_release_yields_a_single_release() {
    let bucket = FakeDistBucket::new();
    bucket.serve_keys(
        "dist/rustc-1.53.0-x86_64-apple-darwin.tar.gz\n\
         dist/rustc-1.53.0-x86_64-apple-darwin.tar.gz.asc\n\
         dist/rustc-1.53.0-x86_64-unknown-linux-gnu.tar.xz\n",
    );

    let source = RustDist::new(bucket.clone());

    let releases = source.fetch().unwrap();

    assert_eq!(releases.len(), 1);
    assert_eq!(
        releases.iter().next().unwrap().version(),
        &Stable::new(1, 53, 0)
    );
}

#[test]
fn releases_carry_no_release_date_and_no_toolchains() {
    let bucket = FakeDistBucket::new();
    bucket.serve_index("dist_static-rust-lang-org.txt");

    let source = RustDist::new(bucket.clone());

    let releases = source.fetch().unwrap();

    assert!(
        releases
            .iter()
            .all(|release| { release.release_date().is_none() && release.toolchains().is_empty() })
    );
}

#[test]
fn download_faults_are_reported() {
    let bucket = FakeDistBucket::new();
    bucket.serve_fault();

    let source = RustDist::new(bucket.clone());

    let error = source.fetch().unwrap_err();

    assert!(matches!(&error, RustDistError::Download(BucketError)));
    assert_eq!(
        error.to_string(),
        "Failed to obtain the Rust distribution index: the distribution bucket is unavailable"
    );
}

#[test]
fn download_faults_of_a_cached_client_are_reported() {
    let cache_file = scratch_cache_file("cached-fault");

    let bucket = FakeDistBucket::new();
    bucket.serve_fault();

    let client =
        CachedDistIndexClient::new(bucket.clone(), cache_file, Duration::from_secs(86_400));
    let source = RustDist::new(client);

    let error = source.fetch().unwrap_err();

    assert!(matches!(
        &error,
        RustDistError::Download(CachedDistIndexError::Client(BucketError))
    ));
}

#[test]
fn an_index_which_is_not_utf8_is_reported() {
    let bucket = FakeDistBucket::new();
    bucket.serve_bytes(&[0xff, 0xfe]);

    let source = RustDist::new(bucket.clone());

    let error = source.fetch().unwrap_err();

    assert!(matches!(
        &error,
        RustDistError::Parse(DistIndexError::UnrecognizedText(_))
    ));
}

fn index_path(index: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../resources/rust_dist")
        .join(index)
}

fn scratch_cache_file(test: &str) -> PathBuf {
    let folder = std::env::temp_dir().join(format!(
        "rust-releases-rust-dist-{}-{}",
        std::process::id(),
        test
    ));

    let _ = std::fs::remove_dir_all(&folder);

    folder.join("dist_static-rust-lang-org.txt")
}

#[derive(Clone, Default)]
struct FakeDistBucket {
    state: Arc<State>,
}

#[derive(Default)]
struct State {
    downloads: Mutex<u32>,
    reply: Mutex<Option<Reply>>,
}

enum Reply {
    Index(Vec<u8>),
    Fault,
}

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
#[error("the distribution bucket is unavailable")]
struct BucketError;

impl FakeDistBucket {
    fn new() -> Self {
        Self::default()
    }

    fn serve_index(&self, index: &str) {
        let path = index_path(index);

        let body = std::fs::read(&path)
            .unwrap_or_else(|error| panic!("unable to read '{}': {error}", path.display()));

        self.reply(Reply::Index(body));
    }

    fn serve_keys(&self, keys: &str) {
        self.serve_bytes(keys.as_bytes());
    }

    fn serve_bytes(&self, index: &[u8]) {
        self.reply(Reply::Index(index.to_vec()));
    }

    fn serve_fault(&self) {
        self.reply(Reply::Fault);
    }

    fn reply(&self, reply: Reply) {
        *self.state.reply.lock().unwrap() = Some(reply);
    }

    fn downloads(&self) -> u32 {
        *self.state.downloads.lock().unwrap()
    }

    fn respond(&self) -> Result<Document, BucketError> {
        *self.state.downloads.lock().unwrap() += 1;

        match self.state.reply.lock().unwrap().as_ref() {
            Some(Reply::Index(index)) => Ok(Document::new(index.clone())),
            Some(Reply::Fault) | None => Err(BucketError),
        }
    }
}

impl DistIndexClient for FakeDistBucket {
    type Error = BucketError;

    fn download(&self) -> Result<Document, Self::Error> {
        self.respond()
    }
}

impl AsyncDistIndexClient for FakeDistBucket {
    type Error = BucketError;

    fn download(&self) -> BoxFuture<'_, Result<Document, Self::Error>> {
        let result = self.respond();

        Box::pin(async move { result })
    }
}
