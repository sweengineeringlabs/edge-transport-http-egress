//! Parse an HTTP-cache configuration from TOML and read the values back.
//!
//! Run with `cargo run -p edge-transport-http-egress-cache-api --example cache_config`.
#![allow(
    clippy::expect_used,
    reason = "example code, not production library code"
)]

use edge_transport_http_egress_cache_api::{CacheConfig, CacheError};

fn main() {
    let toml = r#"
        default_ttl_seconds = 120
        max_entries = 500
        respect_cache_control = false
        cache_private = true
    "#;
    let config = CacheConfig::from_config(toml).expect("the example TOML is valid");
    println!(
        "cache keeps {} entries, falls back to a {}s TTL (respect Cache-Control: {})",
        config.max_entries, config.default_ttl_seconds, config.respect_cache_control
    );
    assert_eq!(config.default_ttl_seconds, 120);

    // An unknown key is rejected rather than silently ignored.
    let err = CacheConfig::from_config("retries = 5").expect_err("unknown field must be rejected");
    assert!(matches!(err, CacheError::ParseFailed(_)));
    println!("unknown field rejected: {err}");
}
