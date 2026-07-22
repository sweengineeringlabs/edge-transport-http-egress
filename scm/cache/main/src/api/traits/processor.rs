//! `Processor` — primary trait for the cache crate (service_type = "processor").

use crate::api::{CacheError, DescribeRequest, DescribeResponse};

/// Primary processing trait for this crate.
pub trait Processor: Send + Sync {
    /// Identify this processor in log / trace output.
    fn describe(&self, request: DescribeRequest) -> Result<DescribeResponse, CacheError>;
}
