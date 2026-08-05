use std::io;
use std::path::PathBuf;

/// An error which is returned for a fault which occurred while accessing the file system.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum IoError {
    /// Returned if a path could not be accessed.
    #[error("I/O error: {error}{}", format!(" at '{}'", .path.display()))]
    InaccessiblePath {
        /// The fault reported by the file system.
        error: io::Error,
        /// The path which could not be accessed.
        path: PathBuf,
    },

    /// Returned if a directory was expected, but a file was found instead.
    #[error("I/O error: path at '{path}' is a file, but expected a directory")]
    DirectoryPathIsFile {
        /// The path which was expected to be a directory.
        path: PathBuf,
    },

    /// Returned if a fault occurred which can not be attributed to a path.
    #[error("I/O error: {error}")]
    Auxiliary {
        /// The fault reported by the file system.
        error: io::Error,
    },
}

impl IoError {
    /// Create an [`IoError::Auxiliary`].
    pub fn auxiliary(error: io::Error) -> Self {
        Self::Auxiliary { error }
    }

    /// Create an [`IoError::InaccessiblePath`].
    pub fn inaccessible(error: io::Error, path: PathBuf) -> Self {
        Self::InaccessiblePath { error, path }
    }

    /// Create an [`IoError::DirectoryPathIsFile`].
    pub fn is_file(path: PathBuf) -> Self {
        Self::DirectoryPathIsFile { path }
    }
}
