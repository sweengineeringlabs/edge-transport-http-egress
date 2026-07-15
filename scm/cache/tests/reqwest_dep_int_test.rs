//! Dependency coverage test for `reqwest`.
//! Verifies that the reqwest types used in the cache middleware are
//! accessible and functional through the public API.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache::{CacheConfig, HttpCacheSvc};
use reqwest::Client;

/// @covers: reqwest
#[test]
fn cache_struct_dep_reqwest_layer_attaches_to_middleware_client_int_test() {
    let cfg = CacheConfig::default();
    let layer = HttpCacheSvc::build_cache_layer(cfg).expect("build_cache_layer must succeed");
    // Verify the layer satisfies reqwest_middleware::Middleware (compile-time proof)
    fn assert_middleware<T: reqwest_middleware::Middleware>() {}
    assert_middleware::<edge_transport_http_egress_cache::CacheLayer>();
    let _client = reqwest_middleware::ClientBuilder::new(Client::new())
        .with(layer)
        .build();
}
