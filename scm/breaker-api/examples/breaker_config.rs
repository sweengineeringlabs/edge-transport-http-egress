//! Parse a circuit-breaker configuration from TOML and read the values back.
//!
//! Run with `cargo run -p edge-transport-http-egress-breaker-api --example breaker_config`.
#![allow(
    clippy::expect_used,
    reason = "example code, not production library code"
)]

use edge_transport_http_egress_breaker_api::{BreakerConfig, BreakerError};

fn main() {
    let toml = r#"
        failure_threshold = 3
        half_open_after_seconds = 10
        reset_after_successes = 2
        failure_statuses = [500, 503]
    "#;
    let config = BreakerConfig::from_config(toml).expect("the example TOML is valid");
    println!(
        "breaker trips after {} failures on statuses {:?}",
        config.failure_threshold, config.failure_statuses
    );
    assert_eq!(config.failure_threshold, 3);

    // An unknown key is rejected rather than silently ignored.
    let err =
        BreakerConfig::from_config("retries = 5").expect_err("unknown field must be rejected");
    assert!(matches!(err, BreakerError::ParseFailed(_)));
    println!("unknown field rejected: {err}");
}
