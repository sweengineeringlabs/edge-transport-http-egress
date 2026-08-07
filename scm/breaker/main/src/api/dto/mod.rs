//! Request/response DTOs for the breaker API's trait contracts.

pub(crate) mod config_validation_request;
pub(crate) mod describe_request;
pub(crate) mod describe_response;
pub(crate) mod failure_threshold_request;
pub(crate) mod failure_threshold_response;

pub use config_validation_request::ConfigValidationRequest;
pub use describe_request::DescribeRequest;
pub use describe_response::DescribeResponse;
pub use failure_threshold_request::FailureThresholdRequest;
pub use failure_threshold_response::FailureThresholdResponse;

// Moved to edge-transport-breaker-policy (ADR-004/ADR-003): the circuit-state DTOs are
// protocol-agnostic. This crate's canonical implementation (edge-transport-grpc-egress's) won
// the Phase 1 design review, so these local definitions are deleted, not kept as duplicates.
pub use edge_transport_breaker_policy::{
    AdmitRequest, AdmitResponse, RecordOutcomeRequest, RecordOutcomeResponse,
};
