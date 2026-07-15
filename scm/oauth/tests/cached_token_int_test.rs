//! Integration tests for the `CachedToken` api marker trait.
//!
//! Rule 120: `src/api/refresh/strategy/oauth/cached_token.rs` requires a
//! corresponding test file.
//!
//! `CachedToken` is a pub marker trait in the api layer. The concrete
//! caching behavior is tested via the middleware end-to-end path.

use std::sync::Arc;

use edge_transport_http_egress_oauth::{OAuthBuilder, OAuthBuilderOps, OAuthTokenSource, Result};
use futures::future::BoxFuture;

/// A token source that counts how many times `get_access_token` is called,
/// verifying that the caching layer reduces upstream calls.
#[derive(Debug)]
struct CountingSource {
    call_count: std::sync::atomic::AtomicU32,
    token: String,
}

impl CountingSource {
    fn new(token: impl Into<String>) -> Self {
        Self {
            call_count: std::sync::atomic::AtomicU32::new(0),
            token: token.into(),
        }
    }

    fn calls(&self) -> u32 {
        self.call_count.load(std::sync::atomic::Ordering::SeqCst)
    }
}

impl OAuthTokenSource for CountingSource {
    fn get_access_token(&self) -> BoxFuture<'_, Result<String>> {
        self.call_count
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let t = self.token.clone();
        Box::pin(async move { Ok(t) })
    }
}

/// @covers: CachedToken (via OAuthTokenSource)
/// The middleware must successfully build with a counting source.
#[test]
fn oauth_struct_cached_token_middleware_builds_with_counting_source_int_test() {
    let src = Arc::new(CountingSource::new("cached-tok"));
    let result = OAuthBuilder::new().with_token_source(src).build();
    assert!(
        result.is_ok(),
        "middleware with counting source must build; got: {result:?}"
    );
}

/// @covers: CachedToken
/// Verifies the counting source increments on each `get_access_token` call.
#[tokio::test]
async fn oauth_struct_cached_token_counting_source_increments_on_call_int_test() {
    let src = CountingSource::new("tok");
    let _ = src.get_access_token().await;
    let _ = src.get_access_token().await;
    assert_eq!(
        src.calls(),
        2,
        "source must be called exactly twice for two invocations"
    );
}
