//! `From<ForeignError>` conversions for [`HttpEgressBuildError`] — the
//! declaration (and the [`AssemblyFailure`] newtype it wraps every variant in)
//! lives in `api/`.

use crate::api::{AssemblyFailure, HttpEgressBuildError};

#[cfg(feature = "retry")]
impl From<edge_transport_http_egress_retry::RetryError> for HttpEgressBuildError {
    fn from(e: edge_transport_http_egress_retry::RetryError) -> Self {
        Self::Retry(AssemblyFailure(Box::new(e)))
    }
}

#[cfg(feature = "rate")]
impl From<edge_transport_http_egress_rate::RateError> for HttpEgressBuildError {
    fn from(e: edge_transport_http_egress_rate::RateError) -> Self {
        Self::Rate(AssemblyFailure(Box::new(e)))
    }
}

#[cfg(feature = "breaker")]
impl From<edge_transport_http_egress_breaker::BreakerError> for HttpEgressBuildError {
    fn from(e: edge_transport_http_egress_breaker::BreakerError) -> Self {
        Self::Breaker(AssemblyFailure(Box::new(e)))
    }
}

#[cfg(feature = "cache")]
impl From<edge_transport_http_egress_cache::CacheError> for HttpEgressBuildError {
    fn from(e: edge_transport_http_egress_cache::CacheError) -> Self {
        Self::Cache(AssemblyFailure(Box::new(e)))
    }
}

#[cfg(feature = "cassette")]
impl From<edge_transport_http_egress_cassette::CassetteError> for HttpEgressBuildError {
    fn from(e: edge_transport_http_egress_cassette::CassetteError) -> Self {
        Self::Cassette(AssemblyFailure(Box::new(e)))
    }
}

#[cfg(feature = "loadbalancer")]
impl From<edge_transport_http_egress_loadbalancer::LoadbalancerMiddlewareError>
    for HttpEgressBuildError
{
    fn from(e: edge_transport_http_egress_loadbalancer::LoadbalancerMiddlewareError) -> Self {
        Self::Loadbalancer(AssemblyFailure(Box::new(e)))
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
