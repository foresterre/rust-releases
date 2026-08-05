use crate::document::RetrievedDocument;
use crate::transport::BoxFuture;

pub mod cached_client;
pub mod errors;
pub mod fs_client;
pub mod remote_client;

/// Fetch a document, given a `resource` description.
pub trait RustReleasesClient {
    /// The type of error returned by the client implementation.
    type Error;

    /// Fetch the document described by the `resource` file.
    fn fetch(&self, resource: ResourceFile) -> Result<RetrievedDocument, Self::Error>;
}

/// Fetch a document asynchronously, given a `resource` description.
///
/// The asynchronous counterpart of [`RustReleasesClient`]. Clients which are
/// generic over their transport implement this trait if, and only if, their
/// transport is an [`AsyncHttpTransport`].
///
/// [`AsyncHttpTransport`]: crate::AsyncHttpTransport
pub trait AsyncRustReleasesClient {
    /// The type of error returned by the client implementation.
    type Error;

    /// Fetch the document described by the `resource` file.
    fn fetch<'a>(
        &'a self,
        resource: ResourceFile<'a, 'a>,
    ) -> BoxFuture<'a, Result<RetrievedDocument, Self::Error>>;
}

/// A resource which can be fetched, named and stored.
#[derive(Clone, Debug)]
pub struct ResourceFile<'url, 'name> {
    // Where the remote resource is located.
    url: &'url str,
    /// What the resource is to be named.
    name: &'name str,
}

impl<'url, 'name> ResourceFile<'url, 'name> {
    /// Create a new resource file.
    ///
    /// The `url` should point to the file to be fetched.
    /// The `name` should refer to name of this resource. It is recommended that
    /// each separate resource has a unique name.
    pub fn new(url: &'url str, name: &'name str) -> Self {
        Self { url, name }
    }

    /// The `url` points to the file to be fetched.
    pub fn url(&self) -> &'url str {
        self.url
    }

    /// The `name` is the identifier of this resource and is recommended to
    /// be unique per resource.
    pub fn name(&self) -> &'name str {
        self.name
    }
}
