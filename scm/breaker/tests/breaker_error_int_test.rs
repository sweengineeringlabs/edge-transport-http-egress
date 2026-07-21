//! Integration tests for `breaker_error` in `edge-transport-http-egress-breaker`.

use edge_transport_http_egress_breaker::BreakerError;

/// @covers: BreakerError
/// Confirms `BreakerError::ParseFailed` carries its message through to the
/// `Display` output — not just that the variant is constructable.
#[test]
fn test_breaker_error_is_accessible() {
    let err = BreakerError::ParseFailed("probe".to_string());
    let msg = err.to_string();
    assert!(
        msg.contains("probe"),
        "Display output must carry the underlying parse-failure message: {msg}"
    );
}
