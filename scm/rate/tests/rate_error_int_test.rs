//! Integration tests for `rate_error` in `edge-transport-http-egress-rate`.

use edge_transport_http_egress_rate::RateError;

/// @covers: RateError
#[test]
fn test_rate_error_is_defined() {
    // Constructing a variant proves the type is accessible from outside the crate.
    let e = RateError::ParseFailed("bad toml".to_string());
    let msg = format!("{e}");
    assert!(
        msg.contains("config parse failed"),
        "error Display must mention the failure kind; got: {msg}"
    );
}
