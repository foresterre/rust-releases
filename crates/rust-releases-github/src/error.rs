use crate::repository::Repository;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum GithubReleasesError<E> {
    // Requests are unauthenticated, so GitHub rate limits them to 60 per hour per IP. Exceeding the
    // limit is reported as a `GithubReleasesError::Fetch`. To send a token in the future, well need
    // to extend the transport to accept additional request headers.
    #[error("Failed to fetch page {page} of the releases of '{repository}': {source}")]
    Fetch {
        repository: Repository,
        page: u32,
        #[source]
        source: E,
    },

    #[error("Failed to deserialize page {page} of the releases of '{repository}': {source}")]
    Deserialize {
        repository: Repository,
        page: u32,
        #[source]
        source: serde_json::Error,
    },

    #[error(
        "Failed to parse the publication date '{timestamp}' of release '{tag}' of '{repository}'"
    )]
    PublicationDate {
        repository: Repository,
        tag: String,
        timestamp: String,
    },

    #[error(
        "Failed to fetch the releases of '{repository}': more than the {max_pages} configured pages of releases are available"
    )]
    TooManyPages {
        repository: Repository,
        max_pages: u32,
    },
}
