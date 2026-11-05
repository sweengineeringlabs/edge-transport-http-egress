//! Dependency coverage test for `moka`.
//! Verifies in-memory cache construction and basic lookup.

use moka::future::Cache;

/// @covers: moka
#[tokio::test]
async fn test_moka_cache_insert_and_get() {
    let cache: Cache<String, u32> = Cache::new(10);
    cache.insert("key".to_string(), 42).await;
    assert_eq!(cache.get(&"key".to_string()).await, Some(42));
}
