use crate::client::errors::IoError;
use crate::{
    Document, IsStaleError, ResourceFile, RetrievalLocation, RetrievedDocument, RustReleasesClient,
};
use std::path::Path;
use std::{fs, io};

const DEFAULT_MEMORY_SIZE: usize = 4096;

/// A client to fetch resources from the local file system.
///
/// The full file path of the resource to be fetched must be given to
/// [`FsClient::fetch`].
#[derive(Debug, Default)]
pub struct FsClient;

impl RustReleasesClient for FsClient {
    type Error = FsClientError;

    fn fetch(&self, resource: ResourceFile) -> Result<RetrievedDocument, Self::Error> {
        // disadvantage of the current API is that the resource file will require the path to
        // be representable as a &str.
        let path = Path::new(resource.url);

        let file =
            fs::File::open(path).map_err(|e| IoError::inaccessible(e, path.to_path_buf()))?;
        let mut reader = io::BufReader::new(file);

        let document = read_document(&mut reader)?;

        Ok(RetrievedDocument::new(
            document,
            RetrievalLocation::Path(path.to_path_buf()),
        ))
    }
}

fn read_document(reader: &mut impl io::BufRead) -> Result<Document, FsClientError> {
    let mut buffer = Vec::with_capacity(DEFAULT_MEMORY_SIZE);

    let bytes_read = reader
        .read_to_end(&mut buffer)
        .map_err(IoError::auxiliary)?;

    if bytes_read == 0 {
        return Err(FsClientError::EmptyFile);
    }

    Ok(Document::new(buffer))
}

/// A list of errors which may be produced by [`CachedClient::fetch`].
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum FsClientError {
    /// Returned if the fetched file was empty.
    #[error("Received empty file")]
    EmptyFile,

    /// Returned in case of an `std::io::Error`.
    #[error(transparent)]
    Io(#[from] IoError),

    /// Returned in case it wasn't possible to check whether the cache file is
    /// stale or not.
    #[error(transparent)]
    IsStale(#[from] IsStaleError),
}
