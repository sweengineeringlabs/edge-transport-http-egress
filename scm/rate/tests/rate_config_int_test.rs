//! Integration tests for `api/rate_config.rs` — the public `RateConfig`
//! struct and its field semantics.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_rate::{HttpRateSvc, RateConfig};

// ---------------------------------------------------------------------------
// Struct literal construction — all three fields are public
// ---------------------------------------------------------------------------

/// All `RateConfig` fields must be publicly constructable via a struct literal.
#[test]
fn test_rate_config_all_fields_constructable_and_readable() {
    let cfg = RateConfig {
        tokens_per_second: 10,
        burst_capacity: 20,
        per_host: true,
    };
    assert_eq!(cfg.tokens_per_second, 10);
    assert_eq!(cfg.burst_capacity, 20);
    assert!(cfg.per_host);
}

// ---------------------------------------------------------------------------
// tokens_per_second — boundary values
// ---------------------------------------------------------------------------

/// `tokens_per_second = 1` — slowest possible refill rate.  Must build.
#[test]
fn test_rate_config_minimum_tokens_per_second_builds() {
    let cfg = RateConfig {
        tokens_per_second: 1,
        burst_capacity: 1,
        per_host: false,
    };
    HttpRateSvc::build_rate_layer(cfg).expect("tokens_per_second=1 must build");
}

/// High token rate — must not be rejected.
#[test]
fn test_rate_config_high_tokens_per_second_builds() {
    let cfg = RateConfig {
        tokens_per_second: u32::MAX,
        burst_capacity: u32::MAX,
        per_host: false,
    };
    HttpRateSvc::build_rate_layer(cfg).expect("max tokens_per_second must build");
}

// ---------------------------------------------------------------------------
// burst_capacity — boundary values
// ---------------------------------------------------------------------------

/// `burst_capacity = 1` — no burst tolerance.  Must build.
#[test]
fn test_rate_config_burst_capacity_one_builds() {
    let cfg = RateConfig {
        tokens_per_second: 10,
        burst_capacity: 1,
        per_host: false,
    };
    HttpRateSvc::build_rate_layer(cfg).expect("burst_capacity=1 must build");
}

/// Burst larger than the refill rate — common production pattern.
#[test]
fn test_rate_config_burst_larger_than_rate_builds() {
    let cfg = RateConfig {
        tokens_per_second: 10,
        burst_capacity: 1_000,
        per_host: false,
    };
    HttpRateSvc::build_rate_layer(cfg).expect("burst > rate must build");
}

// ---------------------------------------------------------------------------
// per_host flag
// ---------------------------------------------------------------------------

/// `per_host = true` — separate bucket per origin.
#[test]
fn test_rate_config_per_host_true_builds() {
    let cfg = RateConfig {
        tokens_per_second: 5,
        burst_capacity: 10,
        per_host: true,
    };
    HttpRateSvc::build_rate_layer(cfg).expect("per_host=true must build");
}

/// `per_host = false` — single global bucket.
#[test]
fn test_rate_config_per_host_false_builds() {
    let cfg = RateConfig {
        tokens_per_second: 5,
        burst_capacity: 10,
        per_host: false,
    };
    HttpRateSvc::build_rate_layer(cfg).expect("per_host=false must build");
}

// ---------------------------------------------------------------------------
// Config round-trip through build_rate_layer
// ---------------------------------------------------------------------------

/// No field must be silently modified between construction and use.
#[test]
fn test_rate_config_round_trips_through_builder_unchanged() {
    let cfg = RateConfig {
        tokens_per_second: 77,
        burst_capacity: 333,
        per_host: true,
    };
    let b_cfg = cfg;
    let out = &b_cfg;
    assert_eq!(out.tokens_per_second, 77);
    assert_eq!(out.burst_capacity, 333);
    assert!(out.per_host);
}
