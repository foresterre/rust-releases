use std::io;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum IoError {
    #[error("I/O error: {error}{}", format!(" at '{}'", .path.display()))]
    InaccessiblePath { error: io::Error, path: PathBuf },

    #[error("I/O error: path at '{path}' is a file, but expected a directory")]
    DirectoryPathIsFile { path: PathBuf },

    #[error("I/O error: {error}")]
    Auxiliary { error: io::Error },
}

impl IoError {
    pub fn auxiliary(error: io::Error) -> Self {
        Self::Auxiliary { error }
    }

    pub fn inaccessible(error: io::Error, path: PathBuf) -> Self {
        Self::InaccessiblePath { error, path }
    }

    pub fn is_file(path: PathBuf) -> Self {
        Self::DirectoryPathIsFile { path }
    }
}

/// An error which is returned for a fault which occurred during processing of an HTTP request.
#[derive(Debug, thiserror::Error)]
#[error("HTTP error: {error}")]
pub struct HttpError {
    // We box the error since it can be very large.
    pub(crate) error: Box<ureq::Error>,
}
