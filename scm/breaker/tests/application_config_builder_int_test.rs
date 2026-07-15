//! Integration tests for `ApplicationConfigBuilder` in `edge-transport-http-egress-breaker`.

use edge_transport_http_egress_breaker::HttpBreakerSvc;

/// @covers: ApplicationConfigBuilder
/// Verifies that `create_config_builder()` returns a builder seeded with the
/// crate name — proving `ApplicationConfigBuilder` is wired into the public API.
#[test]
fn test_application_config_builder_exists() {
    let builder = HttpBreakerSvc::create_config_builder();
    assert_eq!(
        builder.name(),
        "edge-transport-http-egress-breaker",
        "ApplicationConfigBuilder must carry the crate name"
    );
}
