//! Value objects for the loadbalancer middleware API.

pub(crate) mod application_config_builder;
pub mod loadbalancer;

pub use loadbalancer::loadbalancer_layer_pool_metrics::LoadbalancerLayerPoolMetrics;
pub use loadbalancer::loadbalancer_svc_processor::LoadbalancerSvcProcessor;

// Contract types from the loadbalancer library — re-exported so consumers
// only need to depend on this crate.
pub use swe_edge_loadbalancer::{
    Backend, BackendConfig, BackendHealth, BackendId, BackendPoolInstance, LoadbalancerConfig,
    LoadbalancerError as PoolError, Outcome, Strategy,
};
