use std::path::PathBuf;

/// A [`Document`] with added information about the its retrieval.
#[derive(Debug, Eq, PartialEq)]
pub struct RetrievedDocument {
    document: Document,
    retrieval_location: RetrievalLocation,
}

impl RetrievedDocument {
    /// Create a new RetrievedDocument from the Document and its retrieval location.
    pub fn new(document: Document, retrieval_location: RetrievalLocation) -> Self {
        Self {
            document,
            retrieval_location,
        }
    }

    /// Transforms the document into a buffer of bytes.
    pub fn into_document(self) -> Document {
        self.document
    }

    /// Where the document data came from.
    pub fn retrieval_location(&self) -> &RetrievalLocation {
        &self.retrieval_location
    }
}

/// A `Document` represents a resource which can be used as an input to construct a `ReleaseIndex`.
#[derive(Debug, Eq, PartialEq)]
pub struct Document {
    buffer: Vec<u8>,
}

impl Document {
    /// Create a new document from a raw buffer of bytes.
    pub fn new(buffer: Vec<u8>) -> Self {
        Self { buffer }
    }

    /// Read-only access to the underlying buffer.
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    /// Consumes `self` to expose the underlying buffer of bytes.
    pub fn into_buffer(self) -> Vec<u8> {
        self.buffer
    }
}

/// Location a [`Document`] was retrieved from.
#[derive(Debug, Eq, PartialEq)]
pub enum RetrievalLocation {
    /// A document retrieved from a remote URL.
    RemoteUrl(String),
    /// A document retrieved from a cache.
    Cache(PathBuf),
}
