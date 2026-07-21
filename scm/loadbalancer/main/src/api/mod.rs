//! Public traits, types, and errors.
//!
//! Submodules are private (`mod`); every item that crosses a SEA layer boundary
//! is re-exported here explicitly so sibling layers reference `crate::api::Foo`
//! rather than traversing internal module paths.

mod dto;
mod error;
mod traits;
mod types;

// Traits
pub use traits::{PoolMetrics, Processor, Validator};

// Errors
pub use error::LoadbalancerMiddlewareError;

// Request/response DTOs
pub use dto::{
    BackendCountRequest, BackendCountResponse, ConfigValidationRequest, DescribeRequest,
    DescribeResponse,
};

// Public value types (crate-owned + re-exported contract types from the
// loadbalancer library, so consumers depend only on this crate).
pub use types::{
    Backend, BackendConfig, BackendHealth, BackendId, BackendPoolInstance, LoadbalancerConfig,
    LoadbalancerLayerPoolMetrics, LoadbalancerSvcProcessor, Outcome, PoolError, Strategy,
};
