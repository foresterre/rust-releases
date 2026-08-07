#![deny(clippy::all)]
#![deny(unsafe_code)]

mod api;
mod error;
mod paging;
mod repository;
mod source;

pub use crate::error::GithubReleasesError;
pub use crate::paging::{MaxPages, PageSize, PagingError};
pub use crate::repository::Repository;
pub use crate::source::{GITHUB_API_BASE_URL, GithubReleases};
