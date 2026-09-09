use rust_releases_io::ClientError;
use rust_releases_rust_dist::{AwsError, CachedDistError};
use std::path::PathBuf;

pub type DistError = CachedDistError<AwsError>;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum GeneratorError {
    #[error("Failed to read '{}': {source}", path.display())]
    Read {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to write '{}': {source}", path.display())]
    Write {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to collect the stable releases from '{url}': {source}")]
    Changelog {
        url: String,
        #[source]
        source: rust_releases_rust_changelog::RustChangelogError<ClientError>,
    },

    #[error("Failed to set up the Rust distribution client: {0}")]
    DistSetup(#[source] rust_releases_rust_dist::AwsSetupError),

    #[error("Failed to collect the {channel} releases of the Rust distribution bucket: {source}")]
    Dist {
        channel: &'static str,
        #[source]
        source: Box<DistError>,
    },

    #[error("Failed to obtain the release manifest of a stable release: {0}")]
    DistManifest(#[source] Box<DistError>),

    #[error("The release date of Rust {version} is not known")]
    UnknownReleaseDate { version: String },

    #[error("The release manifest of Rust {version} holds a host which cannot be parsed")]
    UnknownHost { version: String },
}
