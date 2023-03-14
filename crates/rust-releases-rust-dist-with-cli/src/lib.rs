#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(unsafe_code)]
#![allow(clippy::upper_case_acronyms)]
//! Please, see the [`rust-releases`] for additional documentation on how this crate can be used.
//!
//! [`rust-releases`]: https://docs.rs/rust-releases

use rust_releases_core::{semver, Release, ReleaseIndex, Source};
use rust_releases_io::Document;
use std::collections::BTreeSet;
use std::fs;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

pub(crate) mod errors;

pub use crate::errors::{RustDistWithCLIError, RustDistWithCLIResult};

/// A [`Source`] which parses Rust release data from the AWS S3 index.
///
/// The data files required as input must be obtained separately (i.e. [`FetchResources`] is not
/// implemented for [`RustDistWithCLI`]). You can download the input data files by using the [`aws`]
/// cli utility and running: `aws --no-sign-request s3 ls static-rust-lang-org/dist/ > rust_dist_with_cli.txt`
///
/// You may then load the source by creating the [`RustDistWithCLI`] and calling the `build_index` method
/// from the `Source` trait.
///
/// ```rust,no_run
/// use rust_releases_core::Source;
/// use rust_releases_rust_dist_with_cli::RustDistWithCLI;
///
/// let source = RustDistWithCLI::from_path("rust_dist_with_cli.txt");
/// let index = source.build_index().expect("Unable to build a release index");
/// ```
///
/// Alternatively you can look at [`RustDist`] which also uses the AWS S3 index, but obtains the
/// input data differently. The [`RustDist`] source does include a [`FetchResources`] implementation.
///
/// [`Source`]: rust_releases_core::Source
/// [`FetchResources`]: rust_releases_core::FetchResources
/// [`RustDistWithCLI`]: crate::RustDistWithCLI
/// [`RustDist`]: https://docs.rs/rust-releases-rust-dist/0.15.0/rust_releases_rust_dist/struct.RustDist.html
/// [`aws`]: https://aws.amazon.com/cli/

pub struct RustDistWithCLI {
    path: PathBuf,
}

impl RustDistWithCLI {
    /// Creates a `RustDistWithCLI` from a path.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl Source for RustDistWithCLI {
    type Error = RustDistWithCLIError;

    fn build_index(&self) -> Result<ReleaseIndex, Self::Error> {
        let file = fs::read(&self.path)?;
        let document = Document::new(file);
        let content = std::str::from_utf8(document.buffer())?;

        // NB: poor man's parsing for stable releases only
        let versions = content
            .lines()
            .filter(|s| !s.trim().starts_with("PRE"))
            .filter_map(|line| {
                line.split_ascii_whitespace()
                    .nth(3)
                    .filter(|s| s.starts_with("rust-1"))
            })
            .filter_map(|s| s.split('-').nth(1))
            .flat_map(|s| semver::Version::parse(s).map(Release::new_stable))
            .collect::<BTreeSet<_>>();

        Ok(ReleaseIndex::from_iter(versions))
    }
}

#[cfg(test)]
mod tests {
    use crate::{ReleaseIndex, RustDistWithCLI};
    use rust_releases_core::semver;

    #[test]
    fn strategy_dist_index() {
        let expected_version = semver::Version::parse("1.50.0").unwrap();

        let path = [
            env!("CARGO_MANIFEST_DIR"),
            "/../../resources/rust_dist_with_cli/dist.txt",
        ]
        .join("");

        let strategy = RustDistWithCLI::from_path(path);
        let index = ReleaseIndex::from_source(strategy).unwrap();

        assert!(index.releases().len() > 50);
        assert_eq!(index.releases()[0].version(), &expected_version);
    }
}
