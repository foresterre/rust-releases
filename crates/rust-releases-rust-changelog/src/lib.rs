#![deny(clippy::all)]
#![deny(unsafe_code)]

mod changelog;
mod error;
mod source;

pub use crate::changelog::{ChangelogError, ReleaseDate};
pub use crate::error::RustChangelogError;
pub use crate::source::{RUST_CHANGELOG_RESOURCE_NAME, RUST_CHANGELOG_URL, RustChangelog};
