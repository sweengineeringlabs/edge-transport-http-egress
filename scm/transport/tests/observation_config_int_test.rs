//! Integration tests for `ObservationConfig`.

use std::sync::Arc;

use edge_transport_http_egress_transport::ObservationConfig;
use swe_observ_metrics::{create_local_metrics_backend, MetricsProvider};

#[test]
fn test_observation_config_struct_stores_provider() {
    let provider: Arc<dyn MetricsProvider> = Arc::new(create_local_metrics_backend());
    let cfg = ObservationConfig {
        provider: Arc::clone(&provider),
    };
    let snaps = cfg.provider.export();
    assert!(snaps.is_empty(), "fresh provider must have no metrics");
}
