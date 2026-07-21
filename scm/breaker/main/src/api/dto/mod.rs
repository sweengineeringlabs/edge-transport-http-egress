//! Request/response DTOs for the breaker API's trait contracts.

pub(crate) mod admit_request;
pub(crate) mod admit_response;
pub(crate) mod closed_state_request;
pub(crate) mod closed_state_response;
pub(crate) mod config_validation_request;
pub(crate) mod describe_request;
pub(crate) mod describe_response;
pub(crate) mod failure_threshold_request;
pub(crate) mod failure_threshold_response;
pub(crate) mod half_open_state_request;
pub(crate) mod half_open_state_response;
pub(crate) mod open_state_request;
pub(crate) mod open_state_response;
pub(crate) mod record_request;
pub(crate) mod record_response;

pub use admit_request::AdmitRequest;
pub use admit_response::AdmitResponse;
pub use closed_state_request::ClosedStateRequest;
pub use closed_state_response::ClosedStateResponse;
pub use config_validation_request::ConfigValidationRequest;
pub use describe_request::DescribeRequest;
pub use describe_response::DescribeResponse;
pub use failure_threshold_request::FailureThresholdRequest;
pub use failure_threshold_response::FailureThresholdResponse;
pub use half_open_state_request::HalfOpenStateRequest;
pub use half_open_state_response::HalfOpenStateResponse;
pub use open_state_request::OpenStateRequest;
pub use open_state_response::OpenStateResponse;
pub use record_request::RecordRequest;
pub use record_response::RecordResponse;
