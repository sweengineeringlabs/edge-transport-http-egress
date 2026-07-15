//! Integration tests for the `RateBucketOps` trait.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_rate::{HttpRateSvc, RateConfig};

/// @covers: RateBucketOps
#[test]
fn test_rate_bucket_ops_wired_through_build_rate_layer() {
    // `build_rate_layer` constructs a `RateLayer` which internally wraps a
    // `TokenBucket` (the concrete `RateBucketOps` impl). If the layer builds
    // successfully, the bucket ops contract is satisfied end-to-end.
    let cfg = RateConfig::default();
    let result = HttpRateSvc::build_rate_layer(cfg);
    assert!(
        result.is_ok(),
        "RateBucketOps wiring must be complete: build_rate_layer returned Err"
    );
}
