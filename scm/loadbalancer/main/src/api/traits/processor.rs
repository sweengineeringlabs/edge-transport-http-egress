//! `Processor` — primary processing contract for the loadbalancer middleware.

use crate::api::{DescribeRequest, DescribeResponse, LoadbalancerMiddlewareError};

/// Primary processing contract. Every loadbalancer middleware unit produced
/// by this crate implements this trait.
pub trait Processor: Send + Sync {
    /// Identify this processor in log / trace output.
    ///
    /// Returns a [`DescribeResponse`] whose `value` is the crate's canonical
    /// name (e.g. `"edge-transport-http-egress-loadbalancer"`).
    fn describe(
        &self,
        request: DescribeRequest,
    ) -> Result<DescribeResponse, LoadbalancerMiddlewareError>;
}
