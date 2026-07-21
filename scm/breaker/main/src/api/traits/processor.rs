//! `Processor` — primary trait for the breaker crate.

use crate::api::{BreakerError, DescribeRequest, DescribeResponse};

/// Primary trait for this crate (service_type = "processor").
pub trait Processor: Send + Sync {
    /// Identify this processor in log / trace output.
    fn describe(&self, request: DescribeRequest) -> Result<DescribeResponse, BreakerError>;
}
