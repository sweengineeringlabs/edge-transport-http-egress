//! `Processor` — primary trait for the rate crate.

use crate::api::{DescribeRequest, DescribeResponse, RateError};

/// Primary trait for this crate (service_type = "processor").
pub trait Processor: Send + Sync {
    /// Identify this processor in log / trace output.
    fn describe(&self, request: DescribeRequest) -> Result<DescribeResponse, RateError>;
}
