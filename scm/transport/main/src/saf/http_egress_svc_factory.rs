//! Composition site for [`HttpEgress`] — one file per trait keeps wiring
//! focused.

use crate::api::{HttpEgress, HttpEgressBuildError, HttpTransportSvc};

/// Factory for the default [`HttpEgress`].
pub struct HttpEgressSvcFactory;

impl HttpEgressSvcFactory {
    /// Construct the SWE-default [`HttpEgress`] (see
    /// [`HttpTransportSvc::default_http_egress`]).
    pub fn create() -> Result<Box<dyn HttpEgress>, HttpEgressBuildError> {
        HttpTransportSvc::default_http_egress()
    }
}
