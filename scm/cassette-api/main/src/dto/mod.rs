//! Request/response DTOs for the cassette API's trait contracts.

pub(crate) mod cassette_mode_request;
pub(crate) mod cassette_mode_response;
pub(crate) mod config_validation_request;
pub(crate) mod describe_request;
pub(crate) mod describe_response;

pub use cassette_mode_request::CassetteModeRequest;
pub use cassette_mode_response::CassetteModeResponse;
pub use config_validation_request::ConfigValidationRequest;
pub use describe_request::DescribeRequest;
pub use describe_response::DescribeResponse;
