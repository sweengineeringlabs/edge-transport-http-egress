//! Integration tests for `cache_error` in `edge-transport-http-egress-cache`.

use edge_transport_http_egress_cache::CacheError;

/// @covers: CacheError
#[test]
fn test_cache_error_is_accessible() {
    // Verify CacheError is part of the public API by instantiating a PhantomData
    // marker — if the type is not exported this file will not compile.
    let _exists = core::marker::PhantomData::<CacheError>;
}
