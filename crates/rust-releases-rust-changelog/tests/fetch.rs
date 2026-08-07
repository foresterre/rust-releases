use http::{Request, Response, StatusCode};
use rust_releases_core::Stable;
use rust_releases_io::{
    AsyncHttpTransport, BoxFuture, ClientError, FsClient, HttpCachedClient, HttpClient,
    HttpTransport, TransportError, TransportResult,
};
use rust_releases_rust_changelog::{
    ChangelogError, ReleaseDate, RustChangelog, RustChangelogError,
};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[test]
fn fetch_the_stable_releases_of_rust() {
    let rust_lang = FakeRustLang::new();
    rust_lang.serve_changelog("RELEASES.md");

    let client = HttpClient::new(rust_lang.clone()).with_timeout(Duration::from_secs(30));
    let source = RustChangelog::new(client).with_today(ReleaseDate::parse("2021-09-01").unwrap());

    let releases = source.fetch().unwrap();

    assert_eq!(releases.len(), 72);
    assert_eq!(
        releases.iter().next().unwrap().version(),
        &Stable::new(0, 11, 0)
    );
    assert_eq!(
        releases.iter().last().unwrap().version(),
        &Stable::new(1, 50, 0)
    );

    assert_eq!(
        rust_lang.requested_urls(),
        ["https://raw.githubusercontent.com/rust-lang/rust/master/RELEASES.md"]
    );
}

#[tokio::test]
async fn fetch_the_stable_releases_of_rust_asynchronously() {
    let rust_lang = FakeRustLang::new();
    rust_lang.serve_changelog("RELEASES.md");

    let client = HttpClient::new(rust_lang.clone());
    let source = RustChangelog::new(client).with_today(ReleaseDate::parse("2021-09-01").unwrap());

    let releases = source.fetch_async().await.unwrap();

    assert_eq!(releases.len(), 72);
    assert_eq!(
        releases.iter().last().unwrap().version(),
        &Stable::new(1, 50, 0)
    );
}

#[test]
fn fetch_the_stable_releases_of_rust_through_a_cache() {
    let cache_folder = std::env::temp_dir().join(format!(
        "rust-releases-rust-changelog-{}-cached-fetch",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&cache_folder);

    let rust_lang = FakeRustLang::new();
    rust_lang.serve_changelog("RELEASES.md");

    let client = HttpCachedClient::new(
        HttpClient::new(rust_lang.clone()),
        cache_folder.clone(),
        Duration::from_secs(86_400),
    );
    let source = RustChangelog::new(client).with_today(ReleaseDate::parse("2021-09-01").unwrap());

    let first = source.fetch().unwrap();
    let second = source.fetch().unwrap();

    assert_eq!(first, second);
    assert_eq!(rust_lang.requested_urls().len(), 1);
    assert!(cache_folder.join("RELEASES.md").exists());

    std::fs::remove_dir_all(&cache_folder).unwrap();
}

#[test]
fn fetch_the_stable_releases_from_a_local_copy_of_the_changelog() {
    let changelog = changelog_path("RELEASES.md");

    let source = RustChangelog::new(FsClient)
        .with_url(changelog.display().to_string())
        .with_today(ReleaseDate::parse("2021-09-01").unwrap());

    let releases = source.fetch().unwrap();

    assert_eq!(releases.len(), 72);
    assert_eq!(
        releases.iter().last().unwrap().version(),
        &Stable::new(1, 50, 0)
    );
}

#[test]
fn versions_which_are_not_released_yet_are_excluded() {
    let rust_lang = FakeRustLang::new();
    rust_lang.serve_changelog("RELEASES_with_unreleased.md");

    let client = HttpClient::new(rust_lang.clone());
    let source = RustChangelog::new(client).with_today(ReleaseDate::parse("2021-09-01").unwrap());

    let releases = source.fetch().unwrap();

    assert_eq!(
        releases.iter().last().unwrap().version(),
        &Stable::new(1, 54, 0)
    );
}

#[test]
fn versions_are_included_once_their_release_date_has_come() {
    let rust_lang = FakeRustLang::new();
    rust_lang.serve_changelog("RELEASES_with_unreleased.md");

    let client = HttpClient::new(rust_lang.clone());
    let source = RustChangelog::new(client).with_today(ReleaseDate::parse("2021-09-09").unwrap());

    let releases = source.fetch().unwrap();

    assert_eq!(
        releases.iter().last().unwrap().version(),
        &Stable::new(1, 55, 0)
    );
}

#[test]
fn fetch_a_changelog_from_a_configured_url() {
    let rust_lang = FakeRustLang::new();
    rust_lang.serve_body("Version 1.50.0 (2021-02-11)\n");

    let client = HttpClient::new(rust_lang.clone());
    let source = RustChangelog::new(client)
        .with_url("https://example.com/rust/RELEASES.md")
        .with_resource_name("example-RELEASES.md");

    let releases = source.fetch().unwrap();

    assert_eq!(releases.len(), 1);
    assert_eq!(
        rust_lang.requested_urls(),
        ["https://example.com/rust/RELEASES.md"]
    );
}

#[test]
fn pre_release_and_two_component_versions_are_excluded() {
    let rust_lang = FakeRustLang::new();
    rust_lang.serve_body(
        "Version 0.12 (2014-10-09)\nVersion 1.0.0-alpha (2015-01-09)\nVersion 1.0.0 (2015-05-15)\n",
    );

    let client = HttpClient::new(rust_lang.clone());
    let source = RustChangelog::new(client);

    let releases = source.fetch().unwrap();

    let versions = releases
        .iter()
        .map(|release| release.version().version.to_string())
        .collect::<Vec<_>>();

    assert_eq!(versions, ["1.0.0"]);
}

#[test]
fn entries_without_a_release_date_are_reported() {
    let rust_lang = FakeRustLang::new();
    rust_lang.serve_body("Version 1.50.0\n");

    let client = HttpClient::new(rust_lang.clone());
    let source = RustChangelog::new(client);

    let error = source.fetch().unwrap_err();

    assert!(matches!(
        &error,
        RustChangelogError::Parse {
            url,
            source: ChangelogError::NoDateInChangelogItem,
        } if url == "https://raw.githubusercontent.com/rust-lang/rust/master/RELEASES.md"
    ));
}

#[test]
fn entries_with_an_unparsable_release_date_are_reported() {
    let rust_lang = FakeRustLang::new();
    rust_lang.serve_body("Version 1.50.0 (yesterday)\n");

    let client = HttpClient::new(rust_lang.clone());
    let source = RustChangelog::new(client);

    let error = source.fetch().unwrap_err();

    assert!(matches!(
        &error,
        RustChangelogError::Parse {
            source: ChangelogError::TimeParseError(item, _),
            ..
        } if item == "yesterday"
    ));
}

#[test]
fn changelogs_which_are_not_utf8_are_reported() {
    let rust_lang = FakeRustLang::new();
    rust_lang.serve_bytes(&[0xff, 0xfe]);

    let client = HttpClient::new(rust_lang.clone());
    let source = RustChangelog::new(client);

    let error = source.fetch().unwrap_err();

    assert!(matches!(
        &error,
        RustChangelogError::Parse {
            source: ChangelogError::UnrecognizedText(_),
            ..
        }
    ));
}

#[test]
fn client_faults_are_reported() {
    let rust_lang = FakeRustLang::new();
    rust_lang.serve_fault();

    let client = HttpClient::new(rust_lang.clone());
    let source = RustChangelog::new(client);

    let error = source.fetch().unwrap_err();

    assert!(matches!(
        &error,
        RustChangelogError::Fetch {
            source: ClientError::Transport { .. },
            ..
        }
    ));
}

#[test]
fn unsuccessful_responses_are_reported() {
    let rust_lang = FakeRustLang::new();
    rust_lang.serve_status(404);

    let client = HttpClient::new(rust_lang.clone());
    let source = RustChangelog::new(client);

    let error = source.fetch().unwrap_err();

    assert!(matches!(
        &error,
        RustChangelogError::Fetch {
            source: ClientError::UnexpectedStatus { status, .. },
            ..
        } if status == &StatusCode::NOT_FOUND
    ));
}

fn changelog_path(changelog: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../resources/rust_changelog")
        .join(changelog)
}

#[derive(Clone, Default)]
struct FakeRustLang {
    state: Arc<State>,
}

#[derive(Default)]
struct State {
    requests: Mutex<Vec<Request<Vec<u8>>>>,
    reply: Mutex<Option<Reply>>,
}

enum Reply {
    Response(StatusCode, Vec<u8>),
    Fault,
}

impl FakeRustLang {
    fn new() -> Self {
        Self::default()
    }

    fn serve_changelog(&self, changelog: &str) {
        let path = changelog_path(changelog);

        let body = std::fs::read(&path)
            .unwrap_or_else(|error| panic!("unable to read '{}': {error}", path.display()));

        self.reply(Reply::Response(StatusCode::OK, body));
    }

    fn serve_body(&self, body: &str) {
        self.serve_bytes(body.as_bytes());
    }

    fn serve_bytes(&self, body: &[u8]) {
        self.reply(Reply::Response(StatusCode::OK, body.to_vec()));
    }

    fn serve_status(&self, status: u16) {
        let status = StatusCode::from_u16(status).expect("valid status code");

        self.reply(Reply::Response(status, b"not found".to_vec()));
    }

    fn serve_fault(&self) {
        self.reply(Reply::Fault);
    }

    fn reply(&self, reply: Reply) {
        *self.state.reply.lock().unwrap() = Some(reply);
    }

    fn requested_urls(&self) -> Vec<String> {
        self.state
            .requests
            .lock()
            .unwrap()
            .iter()
            .map(|request| request.uri().to_string())
            .collect()
    }

    fn respond(&self, request: Request<Vec<u8>>) -> TransportResult {
        self.state.requests.lock().unwrap().push(request);

        match self.state.reply.lock().unwrap().as_ref() {
            Some(Reply::Response(status, body)) => Ok(Response::builder()
                .status(status)
                .body(body.clone())
                .expect("valid response")),
            Some(Reply::Fault) => Err(TransportError::new("the network is down")),
            None => Err(TransportError::new("no changelog was registered")),
        }
    }
}

impl HttpTransport for FakeRustLang {
    fn execute(&self, request: Request<Vec<u8>>) -> TransportResult {
        self.respond(request)
    }
}

impl AsyncHttpTransport for FakeRustLang {
    fn execute(&self, request: Request<Vec<u8>>) -> BoxFuture<'_, TransportResult> {
        let result = self.respond(request);

        Box::pin(async move { result })
    }
}
