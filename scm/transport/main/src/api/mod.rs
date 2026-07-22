//! HTTP egress API — ports, types, and traits.

mod dto;
mod error;
mod traits;
mod types;

// Re-export public DTOs at the top level
pub use dto::{
    ConfigRequest, ConfigResponse, ConnectWebsocketRequest, ConnectWebsocketResponse, GetRequest,
    HealthCheckRequest, HttpBody, HttpRequest, HttpResponse, HttpStreamResponse, SseStream,
    SubscribeSseRequest, SubscribeSseResponse, ValidateRequest, WsChannel, WsReceiver, WsSender,
};

// Re-export public errors at the top level
pub use error::{AssemblyFailure, HttpEgressBuildError, HttpEgressError, ValidationError};

// Re-export public traits at the top level
pub use traits::{HttpEgress, HttpStream, Validator};

// Re-export public types at the top level
pub use types::{
    AlwaysValidConfig, ApplicationConfigBuilder, FormPart, HttpAuth, HttpByteStream, HttpConfig,
    HttpConfigBuilder, HttpConfigValidator, HttpEgressObject, HttpEgressResult, HttpMethod,
    HttpRequestBuilder, HttpSecurityContext, HttpTransportSvc, JsonValue, MetricsHttpEgress,
    SseEvent, ValidatableHttpConfig, ValidatorObject, WsMessage,
};
