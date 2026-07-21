//! Integration tests for `cassette_error` in `edge-transport-http-egress-cassette`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cassette::CassetteError;

/// @covers: CassetteError
/// Confirms `CassetteError::ParseFailed` is publicly constructible and its
/// Display echoes the supplied reason — the contract callers depend on.
#[test]
fn test_cassette_error_is_defined() {
    let err = CassetteError::ParseFailed("bad mode 'xyz'".to_string());
    assert!(
        err.to_string().contains("bad mode 'xyz'"),
        "ParseFailed Display must echo the supplied reason; got: {err}"
    );
}
