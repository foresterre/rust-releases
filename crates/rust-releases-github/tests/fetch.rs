use http::{Request, Response, StatusCode};
use rust_releases_github::{GithubReleases, GithubReleasesError, MaxPages, PageSize, Repository};
use rust_releases_io::{
    AsyncHttpTransport, BoxFuture, ClientError, HttpCachedClient, HttpClient, HttpTransport,
    TransportError, TransportResult,
};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[test]
fn fetch_the_stable_releases_of_rust() {
    let github = FakeGithub::new();
    github.serve_page(1, "rust-lang-rust-page-1.json");
    github.serve_page(2, "rust-lang-rust-page-2.json");
    github.serve_page(3, "rust-lang-rust-page-3.json");

    let client = HttpClient::new(github.clone()).with_timeout(Duration::from_secs(30));
    let source = GithubReleases::new(client).with_page_size(PageSize::new(5).unwrap());

    let releases = source.fetch().unwrap();

    let released = releases
        .iter()
        .map(|release| {
            (
                release.version().version.to_string(),
                release.release_date().unwrap().ymd().to_string(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        released,
        [
            ("0.10.0".to_string(), "2020-09-10".to_string()),
            ("0.12.0".to_string(), "2020-09-10".to_string()),
            ("1.0.0".to_string(), "2020-09-10".to_string()),
            ("1.95.0".to_string(), "2026-04-16".to_string()),
            ("1.96.0".to_string(), "2026-05-28".to_string()),
            ("1.96.1".to_string(), "2026-07-05".to_string()),
            ("1.97.0".to_string(), "2026-07-09".to_string()),
            ("1.97.1".to_string(), "2026-07-16".to_string()),
        ]
    );

    assert_eq!(github.requested_pages(), [1, 2, 3]);
}

#[tokio::test]
async fn fetch_the_stable_releases_of_rust_asynchronously() {
    let github = FakeGithub::new();
    github.serve_page(1, "rust-lang-rust-page-1.json");
    github.serve_page(2, "rust-lang-rust-page-2.json");
    github.serve_page(3, "rust-lang-rust-page-3.json");

    let client = HttpClient::new(github.clone());
    let source = GithubReleases::new(client).with_page_size(PageSize::new(5).unwrap());

    let releases = source.fetch_async().await.unwrap();

    let versions = releases
        .iter()
        .map(|release| release.version().version.to_string())
        .collect::<Vec<_>>();

    assert_eq!(
        versions,
        [
            "0.10.0", "0.12.0", "1.0.0", "1.95.0", "1.96.0", "1.96.1", "1.97.0", "1.97.1"
        ]
    );

    assert_eq!(github.requested_pages(), [1, 2, 3]);
}

#[test]
fn fetch_the_stable_releases_of_rust_through_a_cache() {
    let cache_folder = std::env::temp_dir().join(format!(
        "rust-releases-github-{}-cached-fetch",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&cache_folder);

    let github = FakeGithub::new();
    github.serve_page(1, "rust-lang-rust-page-1.json");
    github.serve_page(2, "rust-lang-rust-page-2.json");
    github.serve_page(3, "rust-lang-rust-page-3.json");

    let client = HttpCachedClient::new(
        HttpClient::new(github.clone()),
        cache_folder.clone(),
        Duration::from_secs(86_400),
    );
    let source = GithubReleases::new(client).with_page_size(PageSize::new(5).unwrap());

    let first = source.fetch().unwrap();
    let second = source.fetch().unwrap();

    assert_eq!(first, second);
    assert_eq!(github.requested_pages(), [1, 2, 3]);
    assert!(
        cache_folder
            .join("github-api-github-com-rust-lang-rust-5-1.json")
            .exists()
    );

    std::fs::remove_dir_all(&cache_folder).unwrap();
}

#[test]
fn fetch_the_stable_releases_of_a_fork_hosted_on_a_github_enterprise_instance() {
    let github = FakeGithub::new();
    github.serve_page(1, "rust-lang-rust-page-1.json");

    let client = HttpClient::new(github.clone());
    let source = GithubReleases::new(client)
        .with_repository(Repository::new("foresterre", "rust"))
        .with_api_base_url("https://github.example.com/api/v3")
        .with_page_size(PageSize::new(6).unwrap());

    let releases = source.fetch().unwrap();

    assert_eq!(releases.len(), 5);
    assert_eq!(
        github.requested_urls(),
        ["https://github.example.com/api/v3/repos/foresterre/rust/releases?per_page=6&page=1"]
    );
}

#[test]
fn releases_carry_no_toolchains() {
    let github = FakeGithub::new();
    github.serve_page(1, "rust-lang-rust-page-1.json");

    let client = HttpClient::new(github.clone());
    let source = GithubReleases::new(client).with_page_size(PageSize::new(6).unwrap());

    let releases = source.fetch().unwrap();

    assert!(
        releases
            .iter()
            .all(|release| release.toolchains().is_empty())
    );
}

#[test]
fn tags_which_are_not_stable_versions_are_excluded() {
    let github = FakeGithub::new();
    github.serve_page(1, "rust-lang-rust-page-2.json");

    let client = HttpClient::new(github.clone());
    let source = GithubReleases::new(client).with_page_size(PageSize::new(6).unwrap());

    let releases = source.fetch().unwrap();

    let versions = releases
        .iter()
        .map(|release| release.version().version.to_string())
        .collect::<Vec<_>>();

    assert_eq!(versions, ["0.10.0", "0.12.0", "1.0.0"]);
}

#[test]
fn drafts_and_prereleases_are_excluded() {
    let github = FakeGithub::new();
    github.serve_page(1, "drafts-and-prereleases.json");

    let client = HttpClient::new(github.clone());
    let source = GithubReleases::new(client).with_page_size(PageSize::new(5).unwrap());

    let releases = source.fetch().unwrap();

    let versions = releases
        .iter()
        .map(|release| release.version().version.to_string())
        .collect::<Vec<_>>();

    assert_eq!(versions, ["1.97.1"]);
}

#[test]
fn releases_without_a_publication_date_have_no_release_date() {
    let github = FakeGithub::new();
    github.serve_page(1, "without-publication-date.json");

    let client = HttpClient::new(github.clone());
    let source = GithubReleases::new(client).with_page_size(PageSize::new(5).unwrap());

    let releases = source.fetch().unwrap();

    assert_eq!(releases.len(), 1);
    assert_eq!(releases.iter().next().unwrap().release_date(), None);
}

#[test]
fn pagination_stops_at_the_first_page_which_is_not_full() {
    let github = FakeGithub::new();
    github.serve_page(1, "rust-lang-rust-page-1.json");
    github.serve_page(2, "rust-lang-rust-page-2.json");

    let client = HttpClient::new(github.clone());
    let source = GithubReleases::new(client).with_page_size(PageSize::new(6).unwrap());

    let releases = source.fetch().unwrap();

    assert_eq!(releases.len(), 5);
    assert_eq!(github.requested_pages(), [1]);
}

#[test]
fn invalid_publication_dates_are_reported() {
    let github = FakeGithub::new();
    github.serve_page(1, "invalid-publication-date.json");

    let client = HttpClient::new(github.clone());
    let source = GithubReleases::new(client).with_page_size(PageSize::new(5).unwrap());

    let error = source.fetch().unwrap_err();

    assert!(matches!(
        &error,
        GithubReleasesError::PublicationDate { repository, tag, timestamp }
            if repository == &Repository::RUST_LANG_RUST
                && tag == "1.97.1"
                && timestamp == "the day before yesterday"
    ));

    assert_eq!(
        error.to_string(),
        "Failed to parse the publication date 'the day before yesterday' of release '1.97.1' of 'rust-lang/rust'"
    );
}

#[test]
fn malformed_pages_are_reported() {
    let github = FakeGithub::new();
    github.serve_page(1, "rust-lang-rust-page-1.json");
    github.serve_page(2, "malformed.json");

    let client = HttpClient::new(github.clone());
    let source = GithubReleases::new(client).with_page_size(PageSize::new(5).unwrap());

    let error = source.fetch().unwrap_err();

    assert!(matches!(
        &error,
        GithubReleasesError::Deserialize { repository, page: 2, .. }
            if repository == &Repository::RUST_LANG_RUST
    ));
}

#[test]
fn client_faults_are_reported() {
    let github = FakeGithub::new();
    github.serve_page(1, "rust-lang-rust-page-1.json");
    github.serve_fault(2);

    let client = HttpClient::new(github.clone());
    let source = GithubReleases::new(client).with_page_size(PageSize::new(5).unwrap());

    let error = source.fetch().unwrap_err();

    assert!(matches!(
        &error,
        GithubReleasesError::Fetch {
            page: 2,
            source: ClientError::Transport { .. },
            ..
        }
    ));
}

#[test]
fn unsuccessful_responses_are_reported() {
    let github = FakeGithub::new();
    github.serve_status(1, 403);

    let client = HttpClient::new(github.clone());
    let source = GithubReleases::new(client).with_page_size(PageSize::new(5).unwrap());

    let error = source.fetch().unwrap_err();

    assert!(matches!(
        &error,
        GithubReleasesError::Fetch {
            page: 1,
            source: ClientError::UnexpectedStatus { status, .. },
            ..
        } if status == &StatusCode::FORBIDDEN
    ));
}

#[test]
fn empty_responses_are_reported() {
    let github = FakeGithub::new();
    github.serve_body(1, "");

    let client = HttpClient::new(github.clone());
    let source = GithubReleases::new(client).with_page_size(PageSize::new(5).unwrap());

    let error = source.fetch().unwrap_err();

    assert!(matches!(
        &error,
        GithubReleasesError::Fetch {
            page: 1,
            source: ClientError::Empty,
            ..
        }
    ));
}

#[test]
fn exceeding_the_configured_page_limit_is_reported() {
    let github = FakeGithub::new();
    github.serve_page(1, "rust-lang-rust-page-1.json");
    github.serve_page(2, "rust-lang-rust-page-2.json");
    github.serve_page(3, "rust-lang-rust-page-3.json");

    let client = HttpClient::new(github.clone());
    let source = GithubReleases::new(client)
        .with_page_size(PageSize::new(5).unwrap())
        .with_max_pages(MaxPages::new(2).unwrap());

    let error = source.fetch().unwrap_err();

    assert!(matches!(
        &error,
        GithubReleasesError::TooManyPages { repository, max_pages: 2 }
            if repository == &Repository::RUST_LANG_RUST
    ));

    assert_eq!(github.requested_pages(), [1, 2]);
}

#[derive(Clone, Default)]
struct FakeGithub {
    state: Arc<State>,
}

#[derive(Default)]
struct State {
    requests: Mutex<Vec<Request<Vec<u8>>>>,
    replies: Mutex<HashMap<u32, Reply>>,
}

enum Reply {
    Response(StatusCode, Vec<u8>),
    Fault,
}

impl FakeGithub {
    fn new() -> Self {
        Self::default()
    }

    fn serve_page(&self, page: u32, fixture: &str) {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join(fixture);

        let body = std::fs::read(&path)
            .unwrap_or_else(|error| panic!("unable to read '{}': {error}", path.display()));

        self.reply(page, Reply::Response(StatusCode::OK, body));
    }

    fn serve_body(&self, page: u32, body: &str) {
        self.reply(
            page,
            Reply::Response(StatusCode::OK, body.as_bytes().to_vec()),
        );
    }

    fn serve_status(&self, page: u32, status: u16) {
        let status = StatusCode::from_u16(status).expect("valid status code");

        self.reply(page, Reply::Response(status, b"{}".to_vec()));
    }

    fn serve_fault(&self, page: u32) {
        self.reply(page, Reply::Fault);
    }

    fn reply(&self, page: u32, reply: Reply) {
        self.state.replies.lock().unwrap().insert(page, reply);
    }

    fn requested_pages(&self) -> Vec<u32> {
        self.state
            .requests
            .lock()
            .unwrap()
            .iter()
            .map(requested_page)
            .collect()
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
        let page = requested_page(&request);

        self.state.requests.lock().unwrap().push(request);

        match self.state.replies.lock().unwrap().get(&page) {
            Some(Reply::Response(status, body)) => Ok(Response::builder()
                .status(status)
                .body(body.clone())
                .expect("valid response")),
            Some(Reply::Fault) => Err(TransportError::new("the network is down")),
            None => Err(TransportError::new(format!(
                "no fixture was registered for page {page}"
            ))),
        }
    }
}

impl HttpTransport for FakeGithub {
    fn execute(&self, request: Request<Vec<u8>>) -> TransportResult {
        self.respond(request)
    }
}

impl AsyncHttpTransport for FakeGithub {
    fn execute(&self, request: Request<Vec<u8>>) -> BoxFuture<'_, TransportResult> {
        let result = self.respond(request);

        Box::pin(async move { result })
    }
}

fn requested_page(request: &Request<Vec<u8>>) -> u32 {
    request
        .uri()
        .query()
        .and_then(|query| {
            query
                .split('&')
                .find_map(|parameter| parameter.strip_prefix("page="))
        })
        .and_then(|page| page.parse().ok())
        .unwrap_or_else(|| panic!("request without a page: {}", request.uri()))
}
