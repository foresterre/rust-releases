use http::Request;
use std::fmt;
use std::time::Duration;

/// How long a request may take, before it times out.
///
/// A client communicates the timeout to its transport by adding this value to
/// the [`http::Request::extensions`] of the request it hands over.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Timeout(Duration);

impl Timeout {
    /// Create a new timeout, which lasts for the given `duration`.
    pub const fn new(duration: Duration) -> Self {
        Self(duration)
    }

    /// The duration of the timeout.
    pub const fn duration(self) -> Duration {
        self.0
    }

    /// The timeout requested by the `request`, if any.
    pub fn of<B>(request: &Request<B>) -> Option<Duration> {
        request
            .extensions()
            .get::<Self>()
            .map(|timeout| timeout.duration())
    }
}

impl From<Duration> for Timeout {
    fn from(duration: Duration) -> Self {
        Self::new(duration)
    }
}

impl fmt::Display for Timeout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
