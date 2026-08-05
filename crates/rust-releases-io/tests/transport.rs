use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use http::header::USER_AGENT;
use http::{Method, Request, Response};
use rust_releases_io::{
    AsyncHttpTransport, AsyncRustReleasesClient, BoxFuture, ClientError, HttpCachedClient,
    HttpClient, HttpTransport, ResourceFile, RetrievalLocation, RustReleasesClient, Timeout,
    TransportError, TransportResult,
};

const URL: &str = "http://rust-releases.example/releases.md";

#[derive(Default)]
struct StubTransport {
    requests: Mutex<Vec<Request<Vec<u8>>>>,
    responses: Mutex<VecDeque<TransportResult>>,
}

impl StubTransport {
    fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    fn queue(&self, status: u16, body: &[u8]) {
        let response = Response::builder()
            .status(status)
            .body(body.to_vec())
            .expect("valid response");

        self.responses.lock().unwrap().push_back(Ok(response));
    }

    fn queue_error(&self) {
        self.responses
            .lock()
            .unwrap()
            .push_back(Err(TransportError::new("the network is down")));
    }

    fn requests(&self) -> Vec<Request<Vec<u8>>> {
        std::mem::take(&mut self.requests.lock().unwrap())
    }

    fn next(&self, request: Request<Vec<u8>>) -> TransportResult {
        self.requests.lock().unwrap().push(request);

        self.responses
            .lock()
            .unwrap()
            .pop_front()
            .unwrap_or_else(|| Err(TransportError::new("no response queued")))
    }
}

impl HttpTransport for StubTransport {
    fn execute(&self, request: Request<Vec<u8>>) -> TransportResult {
        self.next(request)
    }
}

impl AsyncHttpTransport for StubTransport {
    fn execute(&self, request: Request<Vec<u8>>) -> BoxFuture<'_, TransportResult> {
        let result = self.next(request);

        Box::pin(async move { result })
    }
}

fn resource() -> ResourceFile<'static, 'static> {
    ResourceFile::new(URL, "releases.md")
}

fn scratch_folder(test: &str) -> PathBuf {
    let folder =
        std::env::temp_dir().join(format!("rust-releases-io-{}-{}", std::process::id(), test));

    let _ = std::fs::remove_dir_all(&folder);

    folder
}

#[test]
fn fetches_a_document_over_a_blocking_transport() {
    let transport = StubTransport::new();
    transport.queue(200, b"# Releases");

    let client = HttpClient::new(Arc::clone(&transport)).with_timeout(Duration::from_secs(5));

    let retrieved = RustReleasesClient::fetch(&client, resource()).unwrap();

    assert_eq!(
        retrieved.retrieval_location(),
        &RetrievalLocation::Url(URL.to_string())
    );
    assert_eq!(retrieved.into_document().buffer(), b"# Releases");

    let requests = transport.requests();

    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method(), Method::GET);
    assert_eq!(requests[0].uri().to_string(), URL);
    assert_eq!(
        requests[0].headers()[USER_AGENT],
        "rust-releases (github.com/foresterre/rust-releases/issues)"
    );
    assert!(requests[0].body().is_empty());
    assert_eq!(
        Timeout::of(&requests[0]),
        Some(Duration::from_secs(5)),
        "the client should hand its timeout to the transport"
    );
}

#[tokio::test]
async fn fetches_a_document_over_an_asynchronous_transport() {
    let transport = StubTransport::new();
    transport.queue(200, b"# Releases");

    let client = HttpClient::new(Arc::clone(&transport));

    let retrieved = AsyncRustReleasesClient::fetch(&client, resource())
        .await
        .unwrap();

    assert_eq!(
        retrieved.retrieval_location(),
        &RetrievalLocation::Url(URL.to_string())
    );
    assert_eq!(retrieved.into_document().buffer(), b"# Releases");

    assert_eq!(transport.requests().len(), 1);
}

#[test]
fn without_timeout_leaves_the_timeout_to_the_transport() {
    let transport = StubTransport::new();
    transport.queue(200, b"# Releases");

    let client = HttpClient::new(Arc::clone(&transport)).without_timeout();

    RustReleasesClient::fetch(&client, resource()).unwrap();

    assert_eq!(Timeout::of(&transport.requests()[0]), None);
}

#[test]
fn user_agent_can_be_replaced() {
    let transport = StubTransport::new();
    transport.queue(200, b"# Releases");

    let client = HttpClient::new(Arc::clone(&transport)).with_user_agent("my-crate/1.0");

    RustReleasesClient::fetch(&client, resource()).unwrap();

    assert_eq!(
        transport.requests()[0].headers()[USER_AGENT],
        "my-crate/1.0"
    );
}

#[test]
fn unsuccessful_status_codes_are_reported() {
    let transport = StubTransport::new();
    transport.queue(404, b"not found");

    let client = HttpClient::new(Arc::clone(&transport));

    let error = RustReleasesClient::fetch(&client, resource()).unwrap_err();

    assert!(matches!(
        error,
        ClientError::UnexpectedStatus { status, .. } if status == 404
    ));
}

#[test]
fn empty_documents_are_reported() {
    let transport = StubTransport::new();
    transport.queue(200, b"");

    let client = HttpClient::new(Arc::clone(&transport));

    let error = RustReleasesClient::fetch(&client, resource()).unwrap_err();

    assert!(matches!(error, ClientError::Empty));
}

#[test]
fn transport_faults_are_reported() {
    let transport = StubTransport::new();
    transport.queue_error();

    let client = HttpClient::new(Arc::clone(&transport));

    let error = RustReleasesClient::fetch(&client, resource()).unwrap_err();

    assert!(matches!(error, ClientError::Transport { url, .. } if url == URL));
}

#[test]
fn cached_client_caches_a_blocking_fetch() {
    let cache_folder = scratch_folder("blocking-cache");

    let transport = StubTransport::new();
    transport.queue(200, b"# Releases");

    let client = HttpCachedClient::new(
        HttpClient::new(Arc::clone(&transport)),
        cache_folder.clone(),
        Duration::from_secs(86_400),
    );

    let retrieved = RustReleasesClient::fetch(&client, resource()).unwrap();
    assert_eq!(
        retrieved.retrieval_location(),
        &RetrievalLocation::Url(URL.to_string())
    );

    // The second fetch is served from the cache, so the transport is not called again.
    let retrieved = RustReleasesClient::fetch(&client, resource()).unwrap();
    assert_eq!(
        retrieved.retrieval_location(),
        &RetrievalLocation::Path(cache_folder.join("releases.md"))
    );
    assert_eq!(retrieved.into_document().buffer(), b"# Releases");

    assert_eq!(transport.requests().len(), 1);

    std::fs::remove_dir_all(&cache_folder).unwrap();
}

#[tokio::test]
async fn cached_client_caches_an_asynchronous_fetch() {
    let cache_folder = scratch_folder("async-cache");

    let transport = StubTransport::new();
    transport.queue(200, b"# Releases");

    let client = HttpCachedClient::new(
        HttpClient::new(Arc::clone(&transport)),
        cache_folder.clone(),
        Duration::from_secs(86_400),
    );

    AsyncRustReleasesClient::fetch(&client, resource())
        .await
        .unwrap();

    let retrieved = AsyncRustReleasesClient::fetch(&client, resource())
        .await
        .unwrap();

    assert_eq!(
        retrieved.retrieval_location(),
        &RetrievalLocation::Path(cache_folder.join("releases.md"))
    );
    assert_eq!(retrieved.into_document().buffer(), b"# Releases");

    assert_eq!(transport.requests().len(), 1);

    std::fs::remove_dir_all(&cache_folder).unwrap();
}
