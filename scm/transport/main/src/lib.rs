//! `edge_transport_http_egress` — HTTP outbound domain.

// `unwrap`/`expect` are denied in production code (Cargo.toml `[lints.clippy]`)
// but are the idiomatic assertion mechanism inside the crate's inline
// `#[cfg(test)]` modules — allow them only under `cfg(test)`.
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use crate::api::{
    AlwaysValidConfig, ApplicationConfigBuilder, AssemblyFailure, ConfigRequest, ConfigResponse,
    ConnectWebsocketRequest, ConnectWebsocketResponse, FormPart, GetRequest, HealthCheckRequest,
    HttpAuth, HttpBody, HttpByteStream, HttpConfig, HttpConfigBuilder, HttpConfigValidator,
    HttpEgress, HttpEgressBuildError, HttpEgressError, HttpEgressObject, HttpEgressResult,
    HttpMethod, HttpRequest, HttpRequestBuilder, HttpResponse, HttpSecurityContext, HttpStream,
    HttpStreamResponse, HttpTransportSvc, JsonValue, MetricsHttpEgress, SseEvent, SseStream,
    SubscribeSseRequest, SubscribeSseResponse, ValidatableHttpConfig, ValidateRequest,
    ValidationError, Validator, ValidatorObject, WsChannel, WsMessage, WsReceiver, WsSender,
};
pub use crate::saf::{HttpEgressSvcFactory, HttpStreamSvcFactory, ValidatorSvcFactory};
pub use edge_application::SecurityContext;
