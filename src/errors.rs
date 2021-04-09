use crate::source::channel_manifests::ChannelManifestsError;
use crate::source::rust_changelog::RustChangelogError;
use crate::source::rust_dist::RustDistError;
use crate::source::rust_dist_with_cli::DistIndexError;

pub type TResult<T> = Result<T, RustReleasesError>;

/// Top level failure cases for rust-releases
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
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
    ChannelManifestsError(#[from] ChannelManifestsError),

    #[error("{0}")]
    RustChangelogError(#[from] RustChangelogError),

    #[error("{0}")]
    RustDistError(#[from] RustDistError),
}
