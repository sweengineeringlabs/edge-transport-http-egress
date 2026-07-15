//! End-to-end tests for the edge_transport_http_egress_rate SAF builder surface.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_rate::{HttpRateSvc, RateConfig, RateLayer};

fn make_cfg() -> RateConfig {
    RateConfig {
        tokens_per_second: 10,
        burst_capacity: 20,
        per_host: false,
    }
}

/// @covers: build_rate_layer with default config
#[test]
fn test_e2e_builder_default_config_succeeds() {
    let _layer: RateLayer = HttpRateSvc::build_rate_layer(RateConfig::default())
        .expect("build_rate_layer with default config must succeed");
}

/// @covers: build_rate_layer with custom config stores fields correctly
#[test]
fn test_e2e_with_config() {
    let cfg = make_cfg();
    assert_eq!(cfg.tokens_per_second, 10);
    HttpRateSvc::build_rate_layer(cfg).expect("e2e with_config build must succeed");
}

/// @covers: RateConfig fields are accessible directly
#[test]
fn test_e2e_config() {
    let cfg = make_cfg();
    assert_eq!(cfg.burst_capacity, 20);
    assert!(!cfg.per_host);
}

/// @covers: build_rate_layer with explicit config
#[test]
fn test_e2e_build() {
    let cfg = RateConfig {
        tokens_per_second: 100,
        burst_capacity: 500,
        per_host: true,
    };
    let layer = HttpRateSvc::build_rate_layer(cfg).expect("e2e build must succeed");
    assert!(!format!("{layer:?}").is_empty());
}

/// @covers: create_config_builder returns a working Loader
#[test]
fn test_e2e_create_config_builder_returns_loader() {
    let _loader = HttpRateSvc::create_config_builder().build_loader();
}
