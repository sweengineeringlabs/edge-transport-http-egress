//! `Processor` — primary processing trait (service_type = "processor").

use crate::api::{CassetteError, DescribeRequest, DescribeResponse};

/// Primary processing trait for this crate (service_type = "processor").
pub trait Processor: Send + Sync {
    /// Identify this processor in log / trace output.
    fn describe(&self, request: DescribeRequest) -> Result<DescribeResponse, CassetteError>;
}
