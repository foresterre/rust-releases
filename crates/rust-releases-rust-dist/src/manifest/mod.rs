mod channel;
mod detail;

pub use crate::manifest::detail::Detail;

use crate::date;
use rust_releases_core::RustRelease;
use rust_releases_core::rust_release::date::Date;
use rust_releases_core::rust_release::toolchain::{
    Channel, Component, ComponentSet, Target, TargetSet, Toolchain,
};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::iter;

const RUST_PACKAGE: &str = "rust";

const RUST_STD_PACKAGE: &str = "rust-std";

const ANY_TARGET: &str = "*";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseManifest {
    channel: Channel,
    release_date: Date,
    toolchains: Vec<Toolchain>,
    // The manifest is kept as it was received, so a cache can store it
    buffer: Vec<u8>,
}

impl ReleaseManifest {
    pub fn parse(buffer: &[u8]) -> Result<Self, ReleaseManifestError> {
        let content =
            std::str::from_utf8(buffer).map_err(ReleaseManifestError::UnrecognizedText)?;

        let manifest =
            toml::from_str::<Manifest>(content).map_err(ReleaseManifestError::Deserialize)?;

        let release_date =
            date::parse(&manifest.date).ok_or_else(|| ReleaseManifestError::ReleaseDate {
                date: manifest.date.clone(),
            })?;

        let rust = manifest
            .pkg
            .get(RUST_PACKAGE)
            .ok_or(ReleaseManifestError::MissingPackage {
                package: RUST_PACKAGE,
            })?;

        let version = rust
            .version
            .as_deref()
            .ok_or(ReleaseManifestError::MissingVersion {
                package: RUST_PACKAGE,
            })?;

        let channel = channel::parse(version, &release_date)?;

        let toolchains = rust
            .target
            .iter()
            .filter(|(_, target)| target.available)
            .map(|(host, target)| toolchain(&channel, &release_date, host, target))
            .collect();

        Ok(Self {
            channel,
            release_date,
            toolchains,
            buffer: buffer.to_vec(),
        })
    }

    pub fn channel(&self) -> &Channel {
        &self.channel
    }

    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    pub fn release_date(&self) -> &Date {
        &self.release_date
    }

    pub fn toolchains(&self) -> &[Toolchain] {
        &self.toolchains
    }

    pub fn extend<V: Debug>(&self, release: RustRelease<V>, detail: Detail) -> RustRelease<V> {
        let mut release = release;

        if detail.includes_release_date() {
            release.release_date = Some(self.release_date.clone());
        }

        if detail.includes_toolchains() {
            release.toolchains.extend(self.toolchains.iter().cloned());
        }

        release
    }
}

fn toolchain(channel: &Channel, date: &Date, host: &str, target: &PackageTarget) -> Toolchain {
    let packages = || target.components.iter().chain(target.extensions.iter());

    let components = packages()
        .map(|package| Component::new(package.pkg.clone()))
        .collect::<ComponentSet>();

    let host = Target::from_target_triple_or_unknown(host);

    let targets = packages()
        .filter(|package| package.pkg == RUST_STD_PACKAGE && package.target != ANY_TARGET)
        .map(|package| Target::from_target_triple_or_unknown(&package.target))
        .chain(iter::once(host.clone()))
        .collect::<TargetSet>();

    Toolchain::new(
        channel.clone(),
        Some(date.clone()),
        host,
        components,
        targets,
    )
}

#[derive(Debug, Deserialize)]
struct Manifest {
    date: String,
    #[serde(default)]
    pkg: BTreeMap<String, Package>,
}

#[derive(Debug, Deserialize)]
struct Package {
    version: Option<String>,
    #[serde(default)]
    target: BTreeMap<String, PackageTarget>,
}

#[derive(Debug, Deserialize)]
struct PackageTarget {
    #[serde(default)]
    available: bool,
    #[serde(default)]
    components: Vec<PackageReference>,
    #[serde(default)]
    extensions: Vec<PackageReference>,
}

#[derive(Debug, Deserialize)]
struct PackageReference {
    pkg: String,
    target: String,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ReleaseManifestError {
    #[error(transparent)]
    UnrecognizedText(#[from] std::str::Utf8Error),

    #[error("The release manifest could not be deserialized: {0}")]
    Deserialize(#[source] toml::de::Error),

    #[error("The release date '{date}' of the release manifest could not be parsed")]
    ReleaseDate { date: String },

    #[error("The release manifest does not describe the '{package}' package")]
    MissingPackage { package: &'static str },

    #[error("The release manifest does not state the version of the '{package}' package")]
    MissingVersion { package: &'static str },

    #[error("The version '{version}' of the release manifest could not be parsed")]
    Version { version: String },
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_releases_core::Stable;

    const MANIFEST: &[u8] = br#"
manifest-version = "2"
date = "2021-06-17"

[pkg.rust]
version = "1.53.0 (53cb7b09b 2021-06-17)"

[pkg.rust.target.x86_64-unknown-linux-gnu]
available = true
url = "https://static.rust-lang.org/dist/2021-06-17/rust-1.53.0-x86_64-unknown-linux-gnu.tar.gz"

[[pkg.rust.target.x86_64-unknown-linux-gnu.components]]
pkg = "rustc"
target = "x86_64-unknown-linux-gnu"

[[pkg.rust.target.x86_64-unknown-linux-gnu.components]]
pkg = "rust-std"
target = "x86_64-unknown-linux-gnu"

[[pkg.rust.target.x86_64-unknown-linux-gnu.extensions]]
pkg = "rust-std"
target = "wasm32-unknown-unknown"

[[pkg.rust.target.x86_64-unknown-linux-gnu.extensions]]
pkg = "rust-src"
target = "*"

[pkg.rust.target.x86_64-unknown-illumos]
available = false
url = ""
"#;

    fn manifest() -> ReleaseManifest {
        ReleaseManifest::parse(MANIFEST).unwrap()
    }

    #[test]
    fn parse_the_release_date() {
        assert_eq!(manifest().release_date(), &Date::new(2021, 6, 17));
    }

    #[test]
    fn parse_the_channel() {
        assert_eq!(
            manifest().channel(),
            &Channel::Stable(Stable::new(1, 53, 0))
        );
    }

    #[test]
    fn keep_the_manifest_as_it_was_received() {
        assert_eq!(manifest().buffer(), MANIFEST);
    }

    #[test]
    fn parse_the_toolchain_of_an_available_target() {
        let manifest = manifest();
        let toolchains = manifest.toolchains();

        assert_eq!(toolchains.len(), 1);

        let toolchain = &toolchains[0];

        assert_eq!(
            toolchain.host(),
            &Target::from_target_triple_or_unknown("x86_64-unknown-linux-gnu")
        );
        assert_eq!(toolchain.channel(), &Channel::Stable(Stable::new(1, 53, 0)));
        assert_eq!(toolchain.date(), Some(&Date::new(2021, 6, 17)));
    }

    #[test]
    fn the_components_of_a_toolchain_include_its_extensions() {
        let manifest = manifest();
        let components = manifest.toolchains()[0].components().clone();

        let mut names = components
            .iter()
            .map(|component| component.name())
            .collect::<Vec<_>>();
        names.sort_unstable();

        assert_eq!(names, ["rust-src", "rust-std", "rustc"]);
    }

    #[test]
    fn the_targets_of_a_toolchain_are_the_targets_of_its_standard_library() {
        let manifest = manifest();
        let targets = manifest.toolchains()[0].targets().clone();

        assert_eq!(targets.len(), 2);
        assert!(targets.contains(&Target::from_target_triple_or_unknown(
            "x86_64-unknown-linux-gnu"
        )));
        assert!(targets.contains(&Target::from_target_triple_or_unknown(
            "wasm32-unknown-unknown"
        )));
    }

    #[test]
    fn a_toolchain_targets_its_host_even_if_the_manifest_does_not_say_so() {
        let manifest = br#"
date = "2016-04-12"

[pkg.rust]
version = "1.8.0"

[pkg.rust.target.x86_64-apple-darwin]
available = true
"#;

        let manifest = ReleaseManifest::parse(manifest).unwrap();
        let toolchain = &manifest.toolchains()[0];

        assert!(toolchain.components().is_empty());
        assert_eq!(
            toolchain.targets().iter().collect::<Vec<_>>(),
            vec![&Target::from_target_triple_or_unknown(
                "x86_64-apple-darwin"
            )]
        );
    }

    #[test]
    fn extend_a_release_with_every_field() {
        let release = RustRelease::new(Stable::new(1, 53, 0), None, []);

        let extended = manifest().extend(release, Detail::all());

        assert_eq!(extended.release_date(), Some(&Date::new(2021, 6, 17)));
        assert_eq!(extended.toolchains().len(), 1);
    }

    #[test]
    fn extend_a_release_with_its_release_date_only() {
        let release = RustRelease::new(Stable::new(1, 53, 0), None, []);

        let extended = manifest().extend(release, Detail::release_date());

        assert_eq!(extended.release_date(), Some(&Date::new(2021, 6, 17)));
        assert!(extended.toolchains().is_empty());
    }

    #[test]
    fn extend_a_release_with_its_toolchains_only() {
        let release = RustRelease::new(Stable::new(1, 53, 0), None, []);

        let extended = manifest().extend(release, Detail::toolchains());

        assert_eq!(extended.release_date(), None);
        assert_eq!(extended.toolchains().len(), 1);
    }

    #[test]
    fn extend_a_release_with_nothing() {
        let release = RustRelease::new(Stable::new(1, 53, 0), None, []);

        let extended = manifest().extend(release, Detail::default());

        assert_eq!(extended.release_date(), None);
        assert!(extended.toolchains().is_empty());
    }

    #[test]
    fn report_a_manifest_which_is_not_utf8() {
        let error = ReleaseManifest::parse(&[0xff, 0xfe]).unwrap_err();

        assert!(matches!(error, ReleaseManifestError::UnrecognizedText(_)));
    }

    #[test]
    fn report_a_manifest_which_is_not_toml() {
        let error = ReleaseManifest::parse(b"not a manifest").unwrap_err();

        assert!(matches!(error, ReleaseManifestError::Deserialize(_)));
    }

    #[test]
    fn report_a_manifest_with_an_unparsable_release_date() {
        let error = ReleaseManifest::parse(br#"date = "yesterday""#).unwrap_err();

        assert!(matches!(
            error,
            ReleaseManifestError::ReleaseDate { date } if date == "yesterday"
        ));
    }

    #[test]
    fn report_a_manifest_without_a_rust_package() {
        let manifest = br#"
date = "2021-06-17"

[pkg.cargo]
version = "1.53.0"
"#;

        let error = ReleaseManifest::parse(manifest).unwrap_err();

        assert!(matches!(
            error,
            ReleaseManifestError::MissingPackage { package: "rust" }
        ));
    }

    #[test]
    fn report_a_manifest_without_a_rust_version() {
        let manifest = br#"
date = "2021-06-17"

[pkg.rust]
[pkg.rust.target.x86_64-apple-darwin]
available = true
"#;

        let error = ReleaseManifest::parse(manifest).unwrap_err();

        assert!(matches!(
            error,
            ReleaseManifestError::MissingVersion { package: "rust" }
        ));
    }
}
