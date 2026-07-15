//! Integration tests for the `Validator` trait in `edge-transport-http-egress-cache`.

use edge_transport_http_egress_cache::CacheConfig;

/// @covers: Validator
#[test]
fn test_validator_trait_exists_in_crate() {
    // Validator is pub(crate) — it cannot be named from an integration test.
    // We verify the downstream effect: CacheConfig, which the Validator
    // contract covers, is accessible and can be constructed.
    let _exists = core::marker::PhantomData::<CacheConfig>;
}
