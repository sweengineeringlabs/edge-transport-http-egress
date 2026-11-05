//! Dependency coverage test for the `http` crate.
//! @covers: http
//!
//! Rule 95: `http` is used in `src/` (Cache-Control header handling) and must
//! have integration coverage with an explicit `use http::...` import.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use http::{header::CACHE_CONTROL, HeaderMap, HeaderName, HeaderValue};

/// @covers: http
/// Constructs an `http::HeaderMap`, inserts a `Cache-Control` header, and
/// reads it back — this is the exact pattern the cache middleware uses to
/// inspect response headers.
#[test]
fn std_struct_http_dep_header_map_insert_and_get_int_test() {
    let mut map = HeaderMap::new();
    map.insert(CACHE_CONTROL, HeaderValue::from_static("max-age=60"));
    let got = map.get(CACHE_CONTROL);
    assert!(
        got.is_some(),
        "http::HeaderMap must return the inserted Cache-Control header"
    );
    assert_eq!(
        got.unwrap().as_bytes(),
        b"max-age=60",
        "http::HeaderMap must round-trip the Cache-Control value"
    );
}

/// @covers: http
/// Verifies that `http::HeaderName` parses correctly — the cache layer
/// uses header names for Vary matching.
#[test]
fn std_struct_http_dep_header_name_parse_int_test() {
    let name: HeaderName = "x-custom-header"
        .parse()
        .expect("valid header name must parse");
    assert_eq!(
        name.as_str(),
        "x-custom-header",
        "http::HeaderName must round-trip a lowercase custom name"
    );
}
