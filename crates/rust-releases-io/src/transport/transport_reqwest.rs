use crate::transport::timeout::Timeout;
use crate::transport::{AsyncHttpTransport, BoxFuture, TransportError, TransportResult};
use http::{Request, Response};

/// An asynchronous [`AsyncHttpTransport`], backed by `reqwest`.
#[derive(Clone, Debug, Default)]
pub struct ReqwestTransport {
    client: reqwest::Client,
}

impl ReqwestTransport {
    /// Create a new transport, using the default `reqwest` client.
    ///
    /// ```
    /// use rust_releases_io::ReqwestTransport;
    ///
    /// let _transport = ReqwestTransport::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new transport, using the given `client`.
    pub fn with_client(client: reqwest::Client) -> Self {
        Self { client }
    }

    /// The client used to execute requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }
}

impl AsyncHttpTransport for ReqwestTransport {
    fn execute(&self, request: Request<Vec<u8>>) -> BoxFuture<'_, TransportResult> {
        let client = self.client.clone();
        let timeout = Timeout::of(&request);

        Box::pin(async move {
            let mut request = reqwest::Request::try_from(request).map_err(TransportError::new)?;

            // A request without a body must not be sent with a `content-length` header.
            if request
                .body()
                .and_then(reqwest::Body::as_bytes)
                .is_some_and(<[u8]>::is_empty)
            {
                *request.body_mut() = None;
            }

            *request.timeout_mut() = timeout;

            let response = client.execute(request).await.map_err(TransportError::new)?;

            let mut builder = Response::builder()
                .status(response.status())
                .version(response.version());

            if let Some(headers) = builder.headers_mut() {
                *headers = response.headers().clone();
            }

            let body = response.bytes().await.map_err(TransportError::new)?;

            builder.body(body.to_vec()).map_err(TransportError::new)
        })
    }
}
