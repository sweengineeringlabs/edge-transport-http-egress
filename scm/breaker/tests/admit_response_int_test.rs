//! Integration tests for `AdmitResponse`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{
    Admission, AdmitRequest, BreakerConfig, CircuitBreakerNodeFactory,
};
use std::sync::Arc;

fn cfg() -> Arc<BreakerConfig> {
    Arc::new(BreakerConfig::default())
}

/// @covers: AdmitResponse
#[test]
fn test_admit_response_admission_field_is_proceed_on_fresh_node_happy() {
    let mut node = CircuitBreakerNodeFactory::create();
    let resp = node
        .admit(AdmitRequest { config: cfg() })
        .expect("infallible");
    assert_eq!(resp.admission, Admission::Proceed);
}

/// @covers: AdmitResponse
#[test]
fn test_admit_response_admission_field_is_debug_formattable_edge() {
    let mut node = CircuitBreakerNodeFactory::create();
    let resp = node
        .admit(AdmitRequest { config: cfg() })
        .expect("infallible");
    let dbg = format!("{:?}", resp.admission);
    assert_eq!(dbg, "Proceed");
}
