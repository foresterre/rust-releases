use rust_releases_core::{semver, Channel};
use rust_releases_io::IoError;

/// A result type which binds the `ChannelManifestsError` to the error type.
pub type ChannelManifestsResult<T> = Result<T, ChannelManifestsError>;

/// Top level failure cases for rust-releases-channel-manifests source crate
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ChannelManifestsError {
    #[error("Channel {0} is not available for the 'ChannelManifests' source type")]
    ChannelNotAvailable(Channel),

    #[error("{0}")]
    DeserializeToml(#[from] toml::de::Error),

    #[error("Unable to parse manifest date")]
    ParseManifestDate,

    #[error("Unable to parse a manifest source in the meta manifest")]
    ParseManifestSource,

    #[error("Unable to parse the meta manifest")]
    ParseMetaManifest,

    #[error("{0}")]
    ParseRustVersion(#[from] semver::SemVerError),

    #[error("{0}")]
    RustReleasesIoError(#[from] IoError),

    #[error("Unable to find Rust version in release manifest")]
    RustVersionNotFoundInManifest,
}
