//! Metrics type aliases.

pub mod metrics_http_egress;
pub use metrics_http_egress::MetricsHttpEgress;
mod observation;
pub use observation::{HttpMetricKind, HttpMetricObservation, HttpMetricsObserver};
