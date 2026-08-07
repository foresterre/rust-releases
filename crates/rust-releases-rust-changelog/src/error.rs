use crate::changelog::ChangelogError;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RustChangelogError<E> {
    #[error("Failed to fetch the Rust changelog from '{url}': {source}")]
    Fetch {
        url: String,
        #[source]
        source: E,
    },

    #[error("Failed to parse the Rust changelog fetched from '{url}': {source}")]
    Parse {
        url: String,
        #[source]
        source: ChangelogError,
    },
}
