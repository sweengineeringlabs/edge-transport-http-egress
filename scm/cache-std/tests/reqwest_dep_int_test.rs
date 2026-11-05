//! Dependency coverage test for `reqwest`.
//! Verifies that the reqwest types used in the cache middleware are
//! accessible and functional through the public API.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache_api::{CacheConfig, CacheFactory};
use edge_transport_http_egress_cache_std::StdCacheFactory;
use reqwest::Client;

/// @covers: reqwest
#[test]
fn std_struct_dep_reqwest_layer_attaches_to_middleware_client_int_test() {
    let cfg = CacheConfig::default();
    let layer = StdCacheFactory
        .build_cache_layer(cfg)
        .expect("build_cache_layer must succeed");
    // Verify the layer satisfies reqwest_middleware::Middleware (compile-time proof: the upcast
    // to the trait object a ClientBuilder accepts)
    let _as_middleware: std::sync::Arc<dyn reqwest_middleware::Middleware> = layer.clone();
    let _client = reqwest_middleware::ClientBuilder::new(Client::new())
        .with_arc(layer)
        .build();
}
