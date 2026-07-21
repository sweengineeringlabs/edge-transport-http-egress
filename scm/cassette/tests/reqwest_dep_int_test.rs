//! Dependency coverage test for `reqwest`.
//! @covers: reqwest
//!
//! Rule 95: `reqwest` is used in `src/` and must have integration coverage
//! with an explicit `use reqwest::...` import.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use reqwest::Client;

/// @covers: reqwest
/// Verifies `reqwest::Client` is accessible and can build a real request whose
/// method and URL round-trip — the foundation the cassette middleware wraps and
/// derives its match key from.
#[test]
fn cassette_struct_dep_reqwest_client_builds_int_test() {
    let client = Client::new();
    let req = client
        .get("https://api.example.test/foo")
        .build()
        .expect("building a GET request must succeed");
    assert_eq!(req.method().as_str(), "GET");
    assert_eq!(req.url().as_str(), "https://api.example.test/foo");
}

/// @covers: reqwest
/// Verifies `reqwest::Method` variants are accessible — the cassette middleware
/// stores the request method as part of the match key.
#[test]
fn cassette_struct_dep_reqwest_method_variants_int_test() {
    assert_eq!(reqwest::Method::GET.as_str(), "GET");
    assert_eq!(reqwest::Method::POST.as_str(), "POST");
}
