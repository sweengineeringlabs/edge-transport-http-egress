//! `swe_edge_egress_http` — HTTP outbound domain.

// `unwrap`/`expect` are denied in production code (Cargo.toml `[lints.clippy]`)
// but are the idiomatic assertion mechanism inside the crate's inline
// `#[cfg(test)]` modules — allow them only under `cfg(test)`.
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;
mod spi;

pub use crate::api::types::default::TransportConfig;
pub use crate::api::types::metrics::{MetricsHttpEgress, ObservationConfig};
pub use crate::api::types::validator::{
    AlwaysValidConfig, HttpConfigValidator, HttpEgressObject, ValidatableHttpConfig,
    ValidatorObject,
};
pub use crate::api::types::HttpTransportSvc;
pub use crate::api::types::{
    FormPart, HttpAuth, HttpBody, HttpConfig, HttpConfigBuilder, HttpEgressResult, HttpMethod,
    HttpRequest, HttpRequestBuilder, HttpResponse, HttpStreamResponse, SseEvent, SseStream,
    WsChannel, WsMessage, WsReceiver, WsSender,
};
pub use crate::api::{HttpEgress, HttpEgressBuildError, HttpEgressError, HttpStream, Validator};
pub use edge_application::SecurityContext;

/// SAF alias for the default (reqwest-backed) HTTP outbound interface.
pub type DefaultEgress = dyn HttpEgress;
/// SAF alias for the metrics-observation HTTP outbound interface.
pub type MetricsEgress = dyn HttpEgress;
/// SAF alias for the default HTTP config validator interface.
pub type DefaultValidatorAlias = dyn Validator;
/// SAF alias for the HTTP config validator interface.
pub type HttpConfigValidatorAlias = dyn Validator;
