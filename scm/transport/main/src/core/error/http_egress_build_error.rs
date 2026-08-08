//! `From<ForeignError>` conversions for [`HttpEgressBuildError`] — the
//! declaration (and the [`AssemblyFailure`] newtype it wraps every variant in)
//! lives in `api/`.

use crate::api::{AssemblyFailure, HttpEgressBuildError};

impl From<edge_transport_http_egress_resilient::ResilientError> for HttpEgressBuildError {
    fn from(e: edge_transport_http_egress_resilient::ResilientError) -> Self {
        Self::Resilient(AssemblyFailure(Box::new(e)))
    }
}

#[cfg(feature = "tls")]
impl From<edge_security_transport_egress_http_tls::TlsConfigError> for HttpEgressBuildError {
    fn from(e: edge_security_transport_egress_http_tls::TlsConfigError) -> Self {
        Self::Tls(AssemblyFailure(Box::new(e)))
    }
}

#[cfg(feature = "oauth")]
impl From<edge_security_transport_egress_http_oauth::OAuthError> for HttpEgressBuildError {
    fn from(e: edge_security_transport_egress_http_oauth::OAuthError) -> Self {
        Self::OAuth(AssemblyFailure(Box::new(e)))
    }
}

impl From<reqwest::Error> for HttpEgressBuildError {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(AssemblyFailure(Box::new(e)))
    }
}

impl From<swe_edge_configbuilder::ConfigError> for HttpEgressBuildError {
    fn from(e: swe_edge_configbuilder::ConfigError) -> Self {
        Self::Config(AssemblyFailure(Box::new(e)))
    }
}
