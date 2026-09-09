use crate::index::DistIndexError;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RustDistError<E> {
    #[error("Failed to obtain the Rust distribution index: {0}")]
    Download(#[source] E),

    #[error("Failed to parse the Rust distribution index: {0}")]
    Parse(#[source] DistIndexError),
}
