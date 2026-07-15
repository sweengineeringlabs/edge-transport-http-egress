//! Integration tests for `CachedEntryBuilder`.

use edge_transport_http_egress_cache::CachedEntryBuilder;
use std::time::{Duration, Instant};

/// @covers: CachedEntryBuilder::build
#[test]
fn cache_struct_cached_entry_builder_build_with_required_fields_succeeds_int_test() {
    let result = CachedEntryBuilder::new()
        .with_status(200)
        .with_body(b"hello".to_vec())
        .with_expires_at(Instant::now() + Duration::from_secs(60))
        .build();
    assert!(result.is_ok(), "all required fields must produce Ok");
}

/// @covers: CachedEntryBuilder::build
#[test]
fn cache_struct_cached_entry_builder_build_missing_status_fails_int_test() {
    let result = CachedEntryBuilder::new()
        .with_body(b"hello".to_vec())
        .with_expires_at(Instant::now() + Duration::from_secs(60))
        .build();
    assert!(result.is_err(), "missing status must produce Err");
}

/// @covers: CachedEntryBuilder::with_etag
#[test]
fn cache_struct_cached_entry_builder_with_etag_sets_field_int_test() {
    let result = CachedEntryBuilder::new()
        .with_status(200)
        .with_body(vec![])
        .with_expires_at(Instant::now() + Duration::from_secs(1))
        .with_etag("\"abc123\"")
        .build();
    assert!(result.is_ok());
}

/// @covers: CachedEntryBuilder::with_stale_while_revalidate
#[test]
fn cache_struct_cached_entry_builder_with_swr_sets_field_int_test() {
    let result = CachedEntryBuilder::new()
        .with_status(200)
        .with_body(vec![])
        .with_expires_at(Instant::now() + Duration::from_secs(1))
        .with_stale_while_revalidate(Duration::from_secs(30))
        .build();
    assert!(result.is_ok());
}
