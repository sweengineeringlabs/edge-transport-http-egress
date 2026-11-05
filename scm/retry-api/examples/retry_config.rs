//! Build a retry configuration with the builder and parse one from TOML.
//!
//! Run with `cargo run -p edge-transport-http-egress-retry-api --example retry_config`.
#![allow(
    clippy::expect_used,
    reason = "example code, not production library code"
)]

use edge_transport_http_egress_retry_api::{RetryConfig, RetryConfigBuilder, RetryError};

fn main() {
    let built = RetryConfigBuilder::new()
        .max_retries(4)
        .initial_interval_ms(200)
        .build()
        .expect("the example configuration is valid");
    println!(
        "builder: up to {} retries, first wait {}ms",
        built.max_retries, built.initial_interval_ms
    );
    assert_eq!(built.max_retries, 4);

    let toml = r#"
        max_retries = 2
        initial_interval_ms = 50
        max_interval_ms = 500
        multiplier = 3.0
        retryable_statuses = [503]
        retryable_methods = ["GET"]
    "#;
    let parsed = RetryConfig::from_config(toml).expect("the example TOML is valid");
    println!("toml: multiplier {}", parsed.multiplier);
    assert_eq!(parsed.max_retries, 2);

    // An unknown key is rejected rather than silently ignored.
    let err =
        RetryConfig::from_config("verbose = true").expect_err("unknown field must be rejected");
    assert!(matches!(err, RetryError::ParseFailed(_)));
    println!("unknown field rejected: {err}");
}
