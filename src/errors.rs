pub type TResult<T> = Result<T, RustReleasesError>;

#[derive(Debug, thiserror::Error)]
pub enum RustReleasesError {
    #[error("{0}")]
    DeserializeToml(#[from] toml::de::Error),

    #[error("Unable to create or access RustReleases cache")]
    DlCache,

    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Network(#[from] attohttpc::Error),

    #[error("Release channel '{0}' was not found")]
    NoSuchChannel(String),

    #[error("Unable to parse the meta manifest")]
    ParseMetaManifest,

    #[error("Unable to parse manifest date")]
    ParseManifestDate,

    #[error("Unable to parse a manifest source in the meta manifest")]
    ParseManifestSource,

    #[error("{0}")]
    ParseRustVersion(#[from] semver::SemVerError),

    #[error("{0}")]
    SystemTime(#[from] std::time::SystemTimeError),

    #[error("Unable to find Rust version in release manifest")]
    RustVersionNotFoundInManifest,
}
