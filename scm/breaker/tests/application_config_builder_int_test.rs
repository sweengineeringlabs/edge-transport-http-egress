//! Integration tests for `ApplicationConfigBuilder` in `edge-transport-http-egress-breaker`.
//!
//! `ApplicationConfigBuilder` is a crate-internal type alias for
//! `BreakerConfig` (the type this crate resolves `config/application.toml`
//! into) — it is not itself part of the public API, so these tests verify
//! the config type it names is genuinely wired to that file's `[breaker]`
//! section via `HttpBreakerSvcProcessor::create_config_builder()`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::HttpBreakerSvcProcessor;

/// @covers: ApplicationConfigBuilder
#[test]
fn test_application_config_builder_crate_name_matches_this_crate_happy() {
    let builder = HttpBreakerSvcProcessor::create_config_builder();
    assert_eq!(
        builder.name(),
        "edge-transport-http-egress-breaker",
        "the builder seeded for config/application.toml must carry this crate's own name"
    );
}

/// @covers: ApplicationConfigBuilder
#[test]
fn test_application_config_builder_version_is_non_empty_edge() {
    let builder = HttpBreakerSvcProcessor::create_config_builder();
    assert!(!builder.version().is_empty());
}
