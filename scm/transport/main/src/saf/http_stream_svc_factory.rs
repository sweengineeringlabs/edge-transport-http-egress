//! Composition site for [`HttpStream`] — one file per trait keeps wiring
//! focused.

use crate::api::{HttpEgressBuildError, HttpStream, HttpTransportSvc};

/// Factory for the default [`HttpStream`].
pub struct HttpStreamSvcFactory;

impl HttpStreamSvcFactory {
    /// Construct the SWE-default [`HttpStream`] (see
    /// [`HttpTransportSvc::default_http_stream_outbound`]).
    pub fn create() -> Result<Box<dyn HttpStream>, HttpEgressBuildError> {
        HttpTransportSvc::default_http_stream_outbound()
    }
}
