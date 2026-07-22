//! Integration tests for `cache_error` in `edge-transport-http-egress-cache`.

use edge_transport_http_egress_cache::CacheError;

/// @covers: CacheError
#[test]
fn test_cache_error_is_accessible() {
    // Construct a real variant and prove its Display carries the supplied
    // reason verbatim — a stub that dropped the payload would fail this.
    let err = CacheError::ParseFailed("missing field `max_entries`".to_string());
    let msg = err.to_string();
    assert!(
        msg.contains("missing field `max_entries`"),
        "CacheError::ParseFailed Display must echo the reason; got: {msg}"
    );
}
