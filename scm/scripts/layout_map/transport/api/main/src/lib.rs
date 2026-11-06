//! edge_transport_http_egress_transport_api — the HTTP egress transport contract.
//!
//! This crate holds what a consumer programs against: the [`HttpEgress`], [`HttpStream`] and
//! [`Validator`] traits, the request/response DTOs and value types ([`HttpRequest`],
//! [`HttpResponse`], [`HttpConfig`] and their builders), the streaming types, the errors, and the
//! [`TransportFactory`] that builds an outbound transport. The standard reqwest-backed
//! implementation is the `edge-transport-http-egress-transport-std` package.

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
// `unwrap`/`expect` are denied in production code (Cargo.toml `[lints.clippy]`)
// but are the idiomatic assertion mechanism in inline `#[cfg(test)]` modules.
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod dto;
mod error;
mod traits;
mod types;

pub use crate::dto::{
    ConfigRequest, ConfigResponse, ConnectWebsocketRequest, ConnectWebsocketResponse, GetRequest,
    HealthCheckRequest, HttpBody, HttpRequest, HttpResponse, HttpStreamResponse, SseStream,
    SubscribeSseRequest, SubscribeSseResponse, ValidateRequest, WsChannel, WsReceiver, WsSender,
};
pub use crate::error::{AssemblyFailure, HttpEgressBuildError, HttpEgressError, ValidationError};
pub use crate::traits::{
    HttpEgress, HttpEgressFuture, HttpStream, TransportFactory, Validator,
};
pub use crate::types::{
    AlwaysValidConfig, ApplicationConfigBuilder, FormPart, HttpAuth, HttpByteStream, HttpConfig,
    HttpConfigBuilder, HttpConfigValidator, HttpEgressObject, HttpEgressResult, HttpMethod,
    HttpRequestBuilder, HttpSecurityContext, JsonValue, MetricsHttpEgress, SseEvent,
    ValidatableHttpConfig, ValidatorObject, WsMessage,
};
pub use edge_security_runtime::SecurityContext;
