//! Integration tests for `MetricsHttpEgress`.

use edge_transport_http_egress_transport::MetricsEgress;

#[test]
fn test_metrics_http_egress_type_is_object_safe() {
    fn _check(_: &MetricsEgress) {}
}
