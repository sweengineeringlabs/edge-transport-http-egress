//! Request/response DTOs for the rate API's trait contracts.

pub(crate) mod config_validation_request;
pub(crate) mod describe_request;
pub(crate) mod describe_response;
pub(crate) mod rate_limit_request;
pub(crate) mod rate_limit_response;

pub use config_validation_request::ConfigValidationRequest;
pub use describe_request::DescribeRequest;
pub use describe_response::DescribeResponse;
pub use rate_limit_request::RateLimitRequest;
pub use rate_limit_response::RateLimitResponse;
