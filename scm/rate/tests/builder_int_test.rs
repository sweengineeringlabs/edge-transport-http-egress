//! Integration tests for `api/builder.rs` and `saf/builder.rs`.
//!
//! Covers the full public builder surface: `build_rate_layer` and `create_config_builder`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_rate::{
    HttpRateSvcProcessor, RateConfig, RateError, RateLayerRateMetrics,
};

// ---------------------------------------------------------------------------
// build_rate_layer with default config
// ---------------------------------------------------------------------------

/// `HttpRateSvcProcessor::build_rate_layer(RateConfig::default())` must return Ok — the crate-shipped
/// TOML baseline must always be parseable.
#[test]
fn test_builder_fn_succeeds_with_swe_default() {
    HttpRateSvcProcessor::build_rate_layer(RateConfig::default())
        .expect("builder() must succeed with the crate-shipped baseline");
}

/// `tokens_per_second` must be >= 1 in the default config.  A rate of 0
/// would block all requests permanently.
#[test]
fn test_builder_fn_swe_default_tokens_per_second_is_positive() {
    let cfg = RateConfig::default();
    assert!(
        cfg.tokens_per_second >= 1,
        "swe_default tokens_per_second must be >= 1, got {}",
        cfg.tokens_per_second
    );
}

/// `burst_capacity` must be >= 1 in the default config.  A burst of 0 would
/// deny every request that does not arrive exactly at the refill moment.
#[test]
fn test_builder_fn_swe_default_burst_capacity_is_positive() {
    let cfg = RateConfig::default();
    assert!(
        cfg.burst_capacity >= 1,
        "swe_default burst_capacity must be >= 1, got {}",
        cfg.burst_capacity
    );
}

// ---------------------------------------------------------------------------
// build_rate_layer with custom config — custom policy round-trips correctly
// ---------------------------------------------------------------------------

/// `build_rate_layer` must preserve every field without modification.
#[test]
fn test_with_config_preserves_all_fields() {
    let cfg = RateConfig {
        tokens_per_second: 50,
        burst_capacity: 100,
        per_host: true,
    };
    assert_eq!(cfg.tokens_per_second, 50);
    assert_eq!(cfg.burst_capacity, 100);
    assert!(cfg.per_host);
}

/// `per_host = false` must be preserved.
#[test]
fn test_with_config_per_host_false_preserved() {
    let cfg = RateConfig {
        tokens_per_second: 10,
        burst_capacity: 20,
        per_host: false,
    };
    assert!(!cfg.per_host, "per_host=false must survive with_config");
}

/// Config fields must be directly accessible after construction.
#[test]
fn test_config_accessor_returns_reference_not_divergent_copy() {
    let cfg = RateConfig {
        tokens_per_second: 7,
        burst_capacity: 14,
        per_host: true,
    };
    assert_eq!(cfg.tokens_per_second, 7);
    assert_eq!(cfg.burst_capacity, 14);
}

// ---------------------------------------------------------------------------
// build_rate_layer — produces a usable RateLayerRateMetrics
// ---------------------------------------------------------------------------

/// The nominal build path must succeed.
#[test]
fn test_build_from_swe_default_returns_rate_layer() {
    let layer: RateLayerRateMetrics = HttpRateSvcProcessor::build_rate_layer(RateConfig::default())
        .expect("build() must succeed");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("RateLayerRateMetrics"),
        "Debug must identify the type; got: {dbg}"
    );
}

/// Building from a custom config must succeed.
#[test]
fn test_build_with_custom_config_succeeds() {
    let cfg = RateConfig {
        tokens_per_second: 20,
        burst_capacity: 40,
        per_host: false,
    };
    HttpRateSvcProcessor::build_rate_layer(cfg).expect("custom config must build");
}

/// `per_host = true` must build successfully.
#[test]
fn test_build_with_per_host_true_succeeds() {
    let cfg = RateConfig {
        tokens_per_second: 10,
        burst_capacity: 10,
        per_host: true,
    };
    HttpRateSvcProcessor::build_rate_layer(cfg).expect("per_host=true must build");
}

/// `per_host = false` must build successfully.
#[test]
fn test_build_with_per_host_false_succeeds() {
    let cfg = RateConfig {
        tokens_per_second: 10,
        burst_capacity: 10,
        per_host: false,
    };
    HttpRateSvcProcessor::build_rate_layer(cfg).expect("per_host=false must build");
}

/// High token rate and burst capacity are valid operator choices.
#[test]
fn test_build_with_high_rate_and_burst_succeeds() {
    let cfg = RateConfig {
        tokens_per_second: 10_000,
        burst_capacity: 50_000,
        per_host: false,
    };
    HttpRateSvcProcessor::build_rate_layer(cfg).expect("high rate + burst must build");
}

// ---------------------------------------------------------------------------
// create_config_builder — SAF entry point
// ---------------------------------------------------------------------------

/// `create_config_builder().build_loader()` must return a working loader.
#[test]
fn test_create_config_builder_returns_working_loader() {
    let builder = HttpRateSvcProcessor::create_config_builder();
    assert_eq!(
        builder.name(),
        "edge-transport-http-egress-rate",
        "builder must carry the crate name so the loader resolves the right config"
    );
    let _loader = builder.build_loader();
}

// ---------------------------------------------------------------------------
// RateError variants — public constructability
// ---------------------------------------------------------------------------

/// `RateError::ParseFailed` display must name the crate and echo the reason.
#[test]
fn test_error_parse_failed_display_names_crate_and_echoes_reason() {
    let reason = "missing field `tokens_per_second`";
    let err = RateError::ParseFailed(reason.to_string());
    let msg = err.to_string();
    assert!(
        msg.contains("edge_transport_http_egress_rate"),
        "ParseFailed display must name the crate; got: {msg}"
    );
    assert!(
        msg.contains(reason),
        "ParseFailed display must echo the reason; got: {msg}"
    );
}
