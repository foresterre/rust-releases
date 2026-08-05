use crate::transport::timeout::Timeout;
use crate::transport::{HttpTransport, TransportError, TransportResult};
use http::{Request, Response};
use std::io::Read;
use std::time::Duration;

const DEFAULT_MEMORY_SIZE: usize = 4096;

/// A blocking [`HttpTransport`], backed by `ureq`.
#[derive(Clone, Debug)]
pub struct UreqTransport {
    agent: ureq::Agent,
}

impl UreqTransport {
    /// Create a new transport, using an agent which reads its proxy settings
    /// from the environment.
    ///
    /// ```
    /// use rust_releases_io::UreqTransport;
    ///
    /// let _transport = UreqTransport::new();
    /// ```
    pub fn new() -> Self {
        let config = ureq::Agent::config_builder()
            .proxy(ureq::Proxy::try_from_env())
            .http_status_as_error(false)
            .build();

        Self::with_agent(config.new_agent())
    }

    /// Create a new transport, using the given `agent`.
    ///
    /// Configure the agent with [`http_status_as_error`] disabled, so responses
    /// with an unsuccessful status code are handed to the client, instead of
    /// being reported as a transport fault.
    ///
    /// [`http_status_as_error`]: ureq::config::ConfigBuilder::http_status_as_error
    pub fn with_agent(agent: ureq::Agent) -> Self {
        Self { agent }
    }

    /// The agent used to execute requests.
    pub fn agent(&self) -> &ureq::Agent {
        &self.agent
    }

    fn run<B: ureq::AsSendBody>(
        &self,
        request: Request<B>,
        timeout: Option<Duration>,
    ) -> Result<ureq::http::Response<ureq::Body>, TransportError> {
        let request = match timeout {
            Some(timeout) => self
                .agent
                .configure_request(request)
                .timeout_global(Some(timeout))
                .build(),
            None => request,
        };

        self.agent.run(request).map_err(TransportError::new)
    }
}

impl Default for UreqTransport {
    /// Create a new transport, using an agent which reads its proxy settings
    /// from the environment.
    ///
    /// ```
    /// use rust_releases_io::UreqTransport;
    ///
    /// let _transport = UreqTransport::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl HttpTransport for UreqTransport {
    fn execute(&self, request: Request<Vec<u8>>) -> TransportResult {
        let timeout = Timeout::of(&request);
        let (parts, body) = request.into_parts();

        // A request without a body must not be sent with a `content-length` header.
        let response = if body.is_empty() {
            self.run(Request::from_parts(parts, ()), timeout)
        } else {
            self.run(Request::from_parts(parts, body), timeout)
        }?;

        let (parts, body) = response.into_parts();

        let mut buffer = Vec::with_capacity(DEFAULT_MEMORY_SIZE);
        body.into_reader()
            .read_to_end(&mut buffer)
            .map_err(TransportError::new)?;

        Ok(Response::from_parts(parts, buffer))
    }
}
