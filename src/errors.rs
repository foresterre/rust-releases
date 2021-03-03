use crate::source::dist_index::DistIndexError;
use crate::source::from_manifests::FromManifestsError;
use crate::source::rust_changelog::ReleasesMdError;

pub type TResult<T> = Result<T, RustReleasesError>;

/// Top level failure cases for rust-releases
#[derive(Debug, thiserror::Error)]
pub enum RustReleasesError {
    #[error("Unable to create or access RustReleases cache")]
    DlCache,

    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Network(#[from] attohttpc::Error),

    #[error("Release channel '{0}' was not found")]
    NoSuchChannel(String),

    #[error("{0}")]
    SystemTime(#[from] std::time::SystemTimeError),

    // ---------------
    // Source errors
    // ---------------
    #[error("{0}")]
    DistIndexError(#[from] DistIndexError),

    #[error("{0}")]
    FromManifestsError(#[from] FromManifestsError),

    #[error("{0}")]
    ReleasesMdError(#[from] ReleasesMdError),
}
