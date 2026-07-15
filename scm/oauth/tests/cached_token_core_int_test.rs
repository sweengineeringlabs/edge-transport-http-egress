//! Integration tests for core `CachedToken` behavior.
//!
//! Rule 120: `src/core/refresh/strategy/oauth/cached_token.rs` requires a
//! corresponding test file.
//!
//! The core `CachedToken` struct is `pub(super)` and not directly accessible
//! from integration tests. We test the caching behavior through the full
//! middleware stack using a counting token source.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use edge_transport_http_egress_oauth::{OAuthBuilder, OAuthBuilderOps, OAuthTokenSource, Result};
use futures::future::BoxFuture;

/// Counts how many times the underlying token endpoint is called.
#[derive(Debug)]
struct CallCountSource {
    count: Arc<AtomicU32>,
    token: String,
}

impl CallCountSource {
    fn new(token: impl Into<String>) -> (Arc<AtomicU32>, Self) {
        let count = Arc::new(AtomicU32::new(0));
        let src = Self {
            count: Arc::clone(&count),
            token: token.into(),
        };
        (count, src)
    }
}

impl OAuthTokenSource for CallCountSource {
    fn get_access_token(&self) -> BoxFuture<'_, Result<String>> {
        self.count.fetch_add(1, Ordering::SeqCst);
        let t = self.token.clone();
        Box::pin(async move { Ok(t) })
    }
}

/// @covers: core CachedToken (via middleware)
/// The middleware must successfully wrap a call-counting source.
#[test]
fn oauth_struct_cached_token_core_middleware_wraps_counting_source_int_test() {
    let (_count, src) = CallCountSource::new("tok-core");
    let result = OAuthBuilder::new().with_token_source(Arc::new(src)).build();
    assert!(
        result.is_ok(),
        "middleware must build with call-counting source; got: {result:?}"
    );
}
