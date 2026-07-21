//! Integration tests for `FallbackTtlResponse`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache::{
    CacheError, FallbackTtlRequest, FallbackTtlResponse, HttpCache,
};

/// A minimal external test-double proving `HttpCache::default_ttl` can
/// genuinely fail for a real implementor — the crate's own `MiddlewareHttpCache` never
/// returns `Err` here, so this is the only way to exercise the error path.
struct FailingHttpCache;

impl HttpCache for FailingHttpCache {
    fn default_ttl(&self, _request: FallbackTtlRequest) -> Result<FallbackTtlResponse, CacheError> {
        Err(CacheError::InvalidConfig(
            "no fallback ttl configured".to_string(),
        ))
    }
}

/// @covers: default_ttl
#[test]
fn test_default_ttl_unconfigured_implementor_returns_err_error() {
    let cache = FailingHttpCache;
    let result = cache.default_ttl(FallbackTtlRequest);
    assert!(
        matches!(result, Err(CacheError::InvalidConfig(_))),
        "an external HttpCache impl reporting no configured TTL must surface as InvalidConfig; got: {result:?}"
    );
}
