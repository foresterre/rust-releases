use crate::api::{InvalidPublicationDate, ReleasePage};
use crate::error::GithubReleasesError;
use crate::paging::{MaxPages, PageSize};
use crate::repository::Repository;
use rust_releases_core::StableReleases;
use rust_releases_io::{AsyncRustReleasesClient, ResourceFile, RustReleasesClient};
use std::borrow::Cow;

pub const GITHUB_API_BASE_URL: &str = "https://api.github.com";

#[cfg(any(feature = "ureq", feature = "reqwest"))]
const DEFAULT_CACHE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(86_400);

#[derive(Clone, Debug)]
pub struct GithubReleases<C> {
    client: C,
    repository: Repository,
    api_base_url: Cow<'static, str>,
    page_size: PageSize,
    max_pages: MaxPages,
}

impl<C> GithubReleases<C> {
    pub fn new(client: C) -> Self {
        Self {
            client,
            repository: Repository::default(),
            api_base_url: Cow::Borrowed(GITHUB_API_BASE_URL),
            page_size: PageSize::default(),
            max_pages: MaxPages::default(),
        }
    }

    pub fn with_repository(mut self, repository: Repository) -> Self {
        self.repository = repository;
        self
    }

    pub fn with_api_base_url(mut self, api_base_url: impl Into<Cow<'static, str>>) -> Self {
        self.api_base_url = api_base_url.into();
        self
    }

    pub fn with_page_size(mut self, page_size: PageSize) -> Self {
        self.page_size = page_size;
        self
    }

    pub fn with_max_pages(mut self, max_pages: MaxPages) -> Self {
        self.max_pages = max_pages;
        self
    }

    pub fn client(&self) -> &C {
        &self.client
    }

    pub fn repository(&self) -> &Repository {
        &self.repository
    }

    pub fn api_base_url(&self) -> &str {
        &self.api_base_url
    }

    pub fn page_size(&self) -> PageSize {
        self.page_size
    }

    pub fn max_pages(&self) -> MaxPages {
        self.max_pages
    }

    pub fn page_url(&self, page: u32) -> String {
        format!(
            "{base_url}/repos/{owner}/{name}/releases?per_page={page_size}&page={page}",
            base_url = self.api_base_url.trim_end_matches('/'),
            owner = self.repository.owner(),
            name = self.repository.name(),
            page_size = self.page_size,
        )
    }

    pub fn page_resource_name(&self, page: u32) -> String {
        let host = self
            .api_base_url
            .trim_start_matches("https://")
            .trim_start_matches("http://");

        format!(
            "github-{host}-{owner}-{name}-{page_size}-{page}.json",
            host = slug(host),
            owner = slug(self.repository.owner()),
            name = slug(self.repository.name()),
            page_size = self.page_size,
        )
    }

    fn collect_page<E>(
        &self,
        buffer: &[u8],
        page: u32,
        releases: &mut StableReleases,
    ) -> Result<Page, GithubReleasesError<E>> {
        let fetched =
            ReleasePage::parse(buffer).map_err(|source| GithubReleasesError::Deserialize {
                repository: self.repository.clone(),
                page,
                source,
            })?;

        let next = if fetched.is_full(self.page_size) {
            Page::Full
        } else {
            Page::Last
        };

        for entry in fetched {
            match entry.into_stable_release() {
                Ok(Some(release)) => releases.add(release),
                Ok(None) => continue,
                Err(InvalidPublicationDate { tag, timestamp }) => {
                    return Err(GithubReleasesError::PublicationDate {
                        repository: self.repository.clone(),
                        tag,
                        timestamp,
                    });
                }
            }
        }

        Ok(next)
    }

    fn too_many_pages<E>(&self) -> GithubReleasesError<E> {
        GithubReleasesError::TooManyPages {
            repository: self.repository.clone(),
            max_pages: self.max_pages.get(),
        }
    }

    fn fetch_error<E>(&self, page: u32, source: E) -> GithubReleasesError<E> {
        GithubReleasesError::Fetch {
            repository: self.repository.clone(),
            page,
            source,
        }
    }
}

impl<C: RustReleasesClient> GithubReleases<C> {
    pub fn fetch(&self) -> Result<StableReleases, GithubReleasesError<C::Error>> {
        let mut releases = StableReleases::empty();

        for page in 1..=self.max_pages.get() {
            let url = self.page_url(page);
            let name = self.page_resource_name(page);

            let retrieved = self
                .client
                .fetch(ResourceFile::new(&url, &name))
                .map_err(|source| self.fetch_error(page, source))?;

            let buffer = retrieved.into_document().into_buffer();

            if let Page::Last = self.collect_page(&buffer, page, &mut releases)? {
                return Ok(releases);
            }
        }

        Err(self.too_many_pages())
    }
}

impl<C: AsyncRustReleasesClient> GithubReleases<C> {
    pub async fn fetch_async(&self) -> Result<StableReleases, GithubReleasesError<C::Error>> {
        let mut releases = StableReleases::empty();

        for page in 1..=self.max_pages.get() {
            let url = self.page_url(page);
            let name = self.page_resource_name(page);

            let retrieved = self
                .client
                .fetch(ResourceFile::new(&url, &name))
                .await
                .map_err(|source| self.fetch_error(page, source))?;

            let buffer = retrieved.into_document().into_buffer();

            if let Page::Last = self.collect_page(&buffer, page, &mut releases)? {
                return Ok(releases);
            }
        }

        Err(self.too_many_pages())
    }
}

#[cfg(feature = "ureq")]
impl GithubReleases<rust_releases_io::HttpCachedClient<rust_releases_io::UreqTransport>> {
    pub fn new_ureq_cached_client() -> Result<Self, rust_releases_io::BaseCacheDirError> {
        let cache_folder = rust_releases_io::base_cache_dir()?;

        Ok(Self::new(
            rust_releases_io::HttpCachedClient::new_with_default_client(
                cache_folder,
                DEFAULT_CACHE_TIMEOUT,
            ),
        ))
    }
}

#[cfg(feature = "reqwest")]
impl GithubReleases<rust_releases_io::HttpCachedClient<rust_releases_io::ReqwestTransport>> {
    pub fn new_reqwest_cached_client() -> Result<Self, rust_releases_io::BaseCacheDirError> {
        let cache_folder = rust_releases_io::base_cache_dir()?;

        Ok(Self::new(rust_releases_io::HttpCachedClient::new(
            rust_releases_io::HttpClient::new(rust_releases_io::ReqwestTransport::new()),
            cache_folder,
            DEFAULT_CACHE_TIMEOUT,
        )))
    }
}

enum Page {
    Full,
    Last,
}

fn slug(value: &str) -> String {
    value
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn instance() -> GithubReleases<()> {
        GithubReleases::new(())
    }

    #[test]
    fn default_page_url() {
        assert_eq!(
            instance().page_url(1),
            "https://api.github.com/repos/rust-lang/rust/releases?per_page=100&page=1"
        );
    }

    #[test]
    fn page_url_of_a_configured_repository() {
        let source = instance()
            .with_repository(Repository::new("foresterre", "rust-releases"))
            .with_page_size(PageSize::new(5).unwrap());

        assert_eq!(
            source.page_url(3),
            "https://api.github.com/repos/foresterre/rust-releases/releases?per_page=5&page=3"
        );
    }

    #[test]
    fn page_url_of_a_configured_api_base_url() {
        let source = instance().with_api_base_url("https://github.example.com/api/v3/");

        assert_eq!(
            source.page_url(1),
            "https://github.example.com/api/v3/repos/rust-lang/rust/releases?per_page=100&page=1"
        );
    }

    #[test]
    fn default_page_resource_name() {
        assert_eq!(
            instance().page_resource_name(2),
            "github-api-github-com-rust-lang-rust-100-2.json"
        );
    }

    #[test]
    fn page_resource_names_are_unique_per_setting() {
        let names = [
            instance().page_resource_name(1),
            instance().page_resource_name(2),
            instance()
                .with_page_size(PageSize::MIN)
                .page_resource_name(1),
            instance()
                .with_repository(Repository::new("foresterre", "rust-releases"))
                .page_resource_name(1),
            instance()
                .with_api_base_url("https://github.example.com/api/v3")
                .page_resource_name(1),
        ];

        let unique = names.iter().collect::<std::collections::HashSet<_>>();

        assert_eq!(unique.len(), names.len());
    }

    #[test]
    fn page_resource_names_may_not_escape_the_cache_folder() {
        let source = instance().with_repository(Repository::new("../../etc", "rust"));
        let name = source.page_resource_name(1);

        assert_eq!(name, "github-api-github-com-------etc-rust-100-1.json");
        assert!(!name.contains(std::path::is_separator));
    }
}
