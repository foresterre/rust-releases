use crate::strategy::dist_index::DistIndexError;
use crate::strategy::from_manifests::FromManifestsError;
use crate::strategy::releases_md::ReleasesMdError;

pub type TResult<T> = Result<T, RustReleasesError>;

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
    // strategy errors
    // ---------------
    #[error("{0}")]
    DistIndexStrategyError(#[from] DistIndexError),

    #[error("{0}")]
    FromManifestsError(#[from] FromManifestsError),

    #[error("{0}")]
    ReleasesMdStrategyError(#[from] ReleasesMdError),
}
