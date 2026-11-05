//! Parse a rate-limit configuration from TOML and read the values back.
//!
//! Run with `cargo run -p edge-transport-http-egress-rate-api --example rate_config`.
#![allow(
    clippy::expect_used,
    reason = "example code, not production library code"
)]

use edge_transport_http_egress_rate_api::{RateConfig, RateError};

fn main() {
    let toml = r#"
        tokens_per_second = 25
        burst_capacity = 50
        per_host = false
    "#;
    let config = RateConfig::from_config(toml).expect("the example TOML is valid");
    println!(
        "limiter refills {} tokens/s up to {} (per host: {})",
        config.tokens_per_second, config.burst_capacity, config.per_host
    );
    assert_eq!(config.tokens_per_second, 25);

    // An unknown key is rejected rather than silently ignored.
    let err = RateConfig::from_config("retries = 5").expect_err("unknown field must be rejected");
    assert!(matches!(err, RateError::ParseFailed(_)));
    println!("unknown field rejected: {err}");
}
