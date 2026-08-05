use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::{Request, Response};

pub mod timeout;
#[cfg(feature = "reqwest")]
pub mod transport_reqwest;
#[cfg(feature = "ureq")]
pub mod transport_ureq;

/// A boxed, dynamically dispatched [`Future`], as returned by the asynchronous
/// transports.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// The result produced by a transport: either a complete response, or the
/// fault reported by the underlying HTTP client.
pub type TransportResult = Result<Response<Vec<u8>>, TransportError>;

/// A blocking HTTP transport over which a client sends its requests.
///
/// Implement this trait to run the [`RustReleasesClient`] on top of a blocking
/// HTTP client of your choosing. See [`UreqTransport`] for an implementation
/// backed by `ureq`.
///
/// [`RustReleasesClient`]: crate::RustReleasesClient
/// [`UreqTransport`]: crate::UreqTransport
pub trait HttpTransport {
    /// Execute the `request`, and return the complete response.
    ///
    /// A response is to be returned regardless of its status code; it is up to
    /// the caller to determine whether a status code is acceptable.
    fn execute(&self, request: Request<Vec<u8>>) -> TransportResult;
}

/// An asynchronous HTTP transport over which a client sends its requests.
///
/// Implement this trait to run the [`AsyncRustReleasesClient`] on top of an
/// asynchronous HTTP client of your choosing. See [`ReqwestTransport`] for an
/// implementation backed by `reqwest`.
///
/// [`AsyncRustReleasesClient`]: crate::AsyncRustReleasesClient
/// [`ReqwestTransport`]: crate::ReqwestTransport
pub trait AsyncHttpTransport: Send + Sync {
    /// Execute the `request`, and return the complete response.
    ///
    /// A response is to be returned regardless of its status code; it is up to
    /// the caller to determine whether a status code is acceptable.
    fn execute(&self, request: Request<Vec<u8>>) -> BoxFuture<'_, TransportResult>;
}

/// A fault reported by the underlying HTTP client of a transport.
pub struct TransportError(Box<dyn std::error::Error + Send + Sync>);

impl TransportError {
    /// Wrap the fault reported by the underlying HTTP client.
    pub fn new<E>(source: E) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Self(source.into())
    }
}

impl fmt::Debug for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl std::error::Error for TransportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

impl<T: HttpTransport + ?Sized> HttpTransport for &T {
    fn execute(&self, request: Request<Vec<u8>>) -> TransportResult {
        (**self).execute(request)
    }
}

impl<T: HttpTransport + ?Sized> HttpTransport for Arc<T> {
    fn execute(&self, request: Request<Vec<u8>>) -> TransportResult {
        (**self).execute(request)
    }
}

impl<T: HttpTransport + ?Sized> HttpTransport for Box<T> {
    fn execute(&self, request: Request<Vec<u8>>) -> TransportResult {
        (**self).execute(request)
    }
}

impl<T: AsyncHttpTransport + ?Sized> AsyncHttpTransport for &T {
    fn execute(&self, request: Request<Vec<u8>>) -> BoxFuture<'_, TransportResult> {
        (**self).execute(request)
    }
}

impl<T: AsyncHttpTransport + ?Sized> AsyncHttpTransport for Arc<T> {
    fn execute(&self, request: Request<Vec<u8>>) -> BoxFuture<'_, TransportResult> {
        (**self).execute(request)
    }
}

impl<T: AsyncHttpTransport + ?Sized> AsyncHttpTransport for Box<T> {
    fn execute(&self, request: Request<Vec<u8>>) -> BoxFuture<'_, TransportResult> {
        (**self).execute(request)
    }
}
