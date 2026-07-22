//! Request/response DTOs for the loadbalancer API's trait contracts.

pub(crate) mod backend_count_request;
pub(crate) mod backend_count_response;
pub(crate) mod config_validation_request;
pub(crate) mod describe_request;
pub(crate) mod describe_response;

pub use backend_count_request::BackendCountRequest;
pub use backend_count_response::BackendCountResponse;
pub use config_validation_request::ConfigValidationRequest;
pub use describe_request::DescribeRequest;
pub use describe_response::DescribeResponse;
