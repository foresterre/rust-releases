use crate::changelog::{self, ReleaseDate};
use crate::error::RustChangelogError;
use rust_releases_core::StableReleases;
use rust_releases_io::{AsyncRustReleasesClient, ResourceFile, RustReleasesClient};
use std::borrow::Cow;

pub const RUST_CHANGELOG_URL: &str =
    "https://raw.githubusercontent.com/rust-lang/rust/master/RELEASES.md";

pub const RUST_CHANGELOG_RESOURCE_NAME: &str = "RELEASES.md";

#[cfg(any(feature = "ureq", feature = "reqwest"))]
const DEFAULT_CACHE_FOLDER: &str = "source_rust_changelog";

#[cfg(any(feature = "ureq", feature = "reqwest"))]
const DEFAULT_CACHE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(86_400);

#[derive(Clone, Debug)]
pub struct RustChangelog<C> {
    client: C,
    url: Cow<'static, str>,
    resource_name: Cow<'static, str>,
    today: ReleaseDate,
}

impl<C> RustChangelog<C> {
    pub fn new(client: C) -> Self {
        Self {
            client,
            url: Cow::Borrowed(RUST_CHANGELOG_URL),
            resource_name: Cow::Borrowed(RUST_CHANGELOG_RESOURCE_NAME),
            today: ReleaseDate::today(),
        }
    }

    pub fn with_url(mut self, url: impl Into<Cow<'static, str>>) -> Self {
        self.url = url.into();
        self
    }

    pub fn with_resource_name(mut self, resource_name: impl Into<Cow<'static, str>>) -> Self {
        self.resource_name = resource_name.into();
        self
    }

    pub fn with_today(mut self, today: ReleaseDate) -> Self {
        self.today = today;
        self
    }

    pub fn client(&self) -> &C {
        &self.client
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn resource_name(&self) -> &str {
        &self.resource_name
    }

    pub fn today(&self) -> ReleaseDate {
        self.today
    }

    fn collect_changelog<E>(&self, buffer: &[u8]) -> Result<StableReleases, RustChangelogError<E>> {
        changelog::stable_releases(buffer, &self.today).map_err(|source| {
            RustChangelogError::Parse {
                url: self.url.to_string(),
                source,
            }
        })
    }

    fn fetch_error<E>(&self, source: E) -> RustChangelogError<E> {
        RustChangelogError::Fetch {
            url: self.url.to_string(),
            source,
        }
    }
}

impl<C: RustReleasesClient> RustChangelog<C> {
    pub fn fetch(&self) -> Result<StableReleases, RustChangelogError<C::Error>> {
        let retrieved = self
            .client
            .fetch(ResourceFile::new(&self.url, &self.resource_name))
            .map_err(|source| self.fetch_error(source))?;

        self.collect_changelog(retrieved.into_document().buffer())
    }
}

impl<C: AsyncRustReleasesClient> RustChangelog<C> {
    pub async fn fetch_async(&self) -> Result<StableReleases, RustChangelogError<C::Error>> {
        let retrieved = self
            .client
            .fetch(ResourceFile::new(&self.url, &self.resource_name))
            .await
            .map_err(|source| self.fetch_error(source))?;

        self.collect_changelog(retrieved.into_document().buffer())
    }
}

#[cfg(feature = "ureq")]
impl RustChangelog<rust_releases_io::HttpCachedClient<rust_releases_io::UreqTransport>> {
    pub fn new_ureq_cached_client() -> Result<Self, rust_releases_io::BaseCacheDirError> {
        let cache_folder = rust_releases_io::base_cache_dir()?.join(DEFAULT_CACHE_FOLDER);

        Ok(Self::new(
            rust_releases_io::HttpCachedClient::new_with_default_client(
                cache_folder,
                DEFAULT_CACHE_TIMEOUT,
            ),
        ))
    }
}

#[cfg(feature = "reqwest")]
impl RustChangelog<rust_releases_io::HttpCachedClient<rust_releases_io::ReqwestTransport>> {
    pub fn new_reqwest_cached_client() -> Result<Self, rust_releases_io::BaseCacheDirError> {
        let cache_folder = rust_releases_io::base_cache_dir()?.join(DEFAULT_CACHE_FOLDER);

        Ok(Self::new(rust_releases_io::HttpCachedClient::new(
            rust_releases_io::HttpClient::new(rust_releases_io::ReqwestTransport::new()),
            cache_folder,
            DEFAULT_CACHE_TIMEOUT,
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn instance() -> RustChangelog<()> {
        RustChangelog::new(())
    }

    #[test]
    fn default_url() {
        assert_eq!(
            instance().url(),
            "https://raw.githubusercontent.com/rust-lang/rust/master/RELEASES.md"
        );
    }

    #[test]
    fn default_resource_name() {
        assert_eq!(instance().resource_name(), "RELEASES.md");
    }

    #[test]
    fn configured_url_and_resource_name() {
        let source = instance()
            .with_url("https://example.com/rust/RELEASES.md")
            .with_resource_name("example-RELEASES.md");

        assert_eq!(source.url(), "https://example.com/rust/RELEASES.md");
        assert_eq!(source.resource_name(), "example-RELEASES.md");
    }

    #[test]
    fn configured_today() {
        let today = ReleaseDate::parse("2021-09-01").unwrap();

        assert_eq!(instance().with_today(today).today(), today);
    }
}
