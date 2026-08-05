use crate::transport::timeout::Timeout;
use crate::transport::{AsyncHttpTransport, BoxFuture, HttpTransport, TransportError};
use crate::{
    AsyncRustReleasesClient, Document, ResourceFile, RetrievalLocation, RetrievedDocument,
    RustReleasesClient,
};
use http::header::USER_AGENT;
use http::{Request, Response, StatusCode};
use std::borrow::Cow;
use std::time::Duration;

const DEFAULT_USER_AGENT: &str = "rust-releases (github.com/foresterre/rust-releases/issues)";

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(150);

/// A client to download rust releases data over HTTP.
///
/// The client is agnostic of the HTTP client used to execute its requests: the
/// requests are handed over to a transport, which may be either blocking, i.e.
/// an [`HttpTransport`], or asynchronous, i.e. an [`AsyncHttpTransport`].
///
/// A client with a blocking transport implements [`RustReleasesClient`], while
/// a client with an asynchronous transport implements
/// [`AsyncRustReleasesClient`].
#[derive(Clone, Debug)]
pub struct HttpClient<T> {
    transport: T,
    user_agent: Cow<'static, str>,
    timeout: Option<Timeout>,
}

impl<T> HttpClient<T> {
    /// Create a new [`HttpClient`], which sends its requests over the given
    /// `transport`.
    ///
    /// ```
    /// use rust_releases_io::{HttpClient, UreqTransport};
    ///
    /// let _client = HttpClient::new(UreqTransport::new());
    /// ```
    pub fn new(transport: T) -> Self {
        Self {
            transport,
            user_agent: Cow::Borrowed(DEFAULT_USER_AGENT),
            timeout: Some(Timeout::new(DEFAULT_TIMEOUT)),
        }
    }

    /// The transport over which requests are sent.
    pub fn transport(&self) -> &T {
        &self.transport
    }

    /// The timeout requested from the transport, if any.
    pub fn timeout(&self) -> Option<Duration> {
        self.timeout.map(Timeout::duration)
    }

    /// Request the transport to time out after the given `timeout`.
    ///
    /// ```
    /// use std::time::Duration;
    /// use rust_releases_io::{HttpClient, UreqTransport};
    ///
    /// let timeout = Duration::from_secs(86_400);
    ///
    /// let _client = HttpClient::new(UreqTransport::new()).with_timeout(timeout);
    /// ```
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(Timeout::new(timeout));
        self
    }

    /// Leave the timeout up to the transport.
    pub fn without_timeout(mut self) -> Self {
        self.timeout = None;
        self
    }

    /// The user agent sent with each request.
    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }

    /// Send each request with the given `user_agent`.
    pub fn with_user_agent(mut self, user_agent: impl Into<Cow<'static, str>>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    fn request(&self, url: &str) -> Result<Request<Vec<u8>>, ClientError> {
        let mut builder = Request::get(url).header(USER_AGENT, self.user_agent.as_ref());

        if let Some(timeout) = self.timeout {
            builder = builder.extension(timeout);
        }

        builder
            .body(Vec::new())
            .map_err(|source| ClientError::build_request(url, source))
    }
}

#[cfg(feature = "ureq")]
impl Default for HttpClient<crate::UreqTransport> {
    /// Create a new [`HttpClient`], which sends its requests over the
    /// [`UreqTransport`].
    ///
    /// ```
    /// use rust_releases_io::HttpClient;
    ///
    /// let _client = HttpClient::default();
    /// ```
    ///
    /// [`UreqTransport`]: crate::UreqTransport
    fn default() -> Self {
        Self::new(crate::UreqTransport::new())
    }
}

impl<T: HttpTransport> RustReleasesClient for HttpClient<T> {
    type Error = ClientError;

    fn fetch(&self, resource: ResourceFile) -> Result<RetrievedDocument, Self::Error> {
        let url = resource.url();

        let response = self
            .transport
            .execute(self.request(url)?)
            .map_err(|source| ClientError::transport(url, source))?;

        into_retrieved_document(response, url)
    }
}

impl<T: AsyncHttpTransport> AsyncRustReleasesClient for HttpClient<T> {
    type Error = ClientError;

    fn fetch<'a>(
        &'a self,
        resource: ResourceFile<'a, 'a>,
    ) -> BoxFuture<'a, Result<RetrievedDocument, Self::Error>> {
        Box::pin(async move {
            let url = resource.url();

            let response = self
                .transport
                .execute(self.request(url)?)
                .await
                .map_err(|source| ClientError::transport(url, source))?;

            into_retrieved_document(response, url)
        })
    }
}

fn into_retrieved_document(
    response: Response<Vec<u8>>,
    url: &str,
) -> Result<RetrievedDocument, ClientError> {
    let status = response.status();
    let body = response.into_body();

    if !status.is_success() {
        return Err(ClientError::UnexpectedStatus {
            url: url.to_string(),
            status,
        });
    }

    if body.is_empty() {
        return Err(ClientError::Empty);
    }

    Ok(RetrievedDocument::new(
        Document::new(body),
        RetrievalLocation::Url(url.to_string()),
    ))
}

/// A list of errors which may be produced by [`HttpClient::fetch`].
///
/// [`HttpClient::fetch`]: RustReleasesClient::fetch
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ClientError {
    /// Returned if an empty document was fetched.
    #[error("Received empty file")]
    Empty,

    /// Returned if the request to be sent to the transport could not be built.
    #[error("Failed to build request for '{url}': {source}")]
    BuildRequest {
        /// The url the request was to be sent to.
        url: String,
        /// The fault which occurred while building the request.
        source: http::Error,
    },

    /// Returned if the transport could not execute the request.
    #[error("Failed to fetch '{url}': {source}")]
    Transport {
        /// The url the request was sent to.
        url: String,
        /// The fault reported by the transport.
        source: TransportError,
    },

    /// Returned if the response had an unsuccessful status code.
    #[error("Failed to fetch '{url}': received status code '{status}'")]
    UnexpectedStatus {
        /// The url the request was sent to.
        url: String,
        /// The status code of the response.
        status: StatusCode,
    },
}

impl ClientError {
    fn build_request(url: &str, source: http::Error) -> Self {
        Self::BuildRequest {
            url: url.to_string(),
            source,
        }
    }

    fn transport(url: &str, source: TransportError) -> Self {
        Self::Transport {
            url: url.to_string(),
            source,
        }
    }
}
