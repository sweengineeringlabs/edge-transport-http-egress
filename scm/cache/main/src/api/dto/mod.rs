//! Request/response DTOs for the cache API's trait contracts.

pub(crate) mod config_validation_request;
pub(crate) mod describe_request;
pub(crate) mod describe_response;
pub(crate) mod fallback_ttl_request;
pub(crate) mod fallback_ttl_response;

pub use config_validation_request::ConfigValidationRequest;
pub use describe_request::DescribeRequest;
pub use describe_response::DescribeResponse;
pub use fallback_ttl_request::FallbackTtlRequest;
pub use fallback_ttl_response::FallbackTtlResponse;
