use rust_releases_core::{semver, Channel};
use rust_releases_io::IoError;

/// A result type which binds the `ChannelManifestsError` to the error type.
pub type ChannelManifestsResult<T> = Result<T, ChannelManifestsError>;

/// Top level failure cases for rust-releases-channel-manifests source crate
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ChannelManifestsError {
    /// Returned when the channel is not available, that is, is unimplemented.
    #[error("Channel {0} is not available for the 'ChannelManifests' source type")]
    ChannelNotAvailable(Channel),

    /// Returned when a retrieved manifest is not decodable as TOML.
    #[error("{0}")]
    DeserializeToml(#[from] toml::de::Error),

    /// Returned when the manifest date can not be parsed.
    #[error("Unable to parse manifest date")]
    ParseManifestDate,

    /// Returned when the manifest source can not be parsed from the top level meta manifest.
    #[error("Unable to parse a manifest source in the meta manifest")]
    ParseManifestSource,

    /// Returned when the top level meta manifest can not be parsed.
    #[error("Unable to parse the meta manifest")]
    ParseMetaManifest,

    /// Returned when there is an issue with a semver version.
    #[error("{0}")]
    ParseRustVersion(#[from] semver::Error),

    /// Returned in case of an I/O error.
    #[error("{0}")]
    RustReleasesIoError(#[from] IoError),

    /// Returned when the Rust version can not be found in a release manifest.
    #[error("Unable to find Rust version in release manifest")]
    RustVersionNotFoundInManifest,
}
