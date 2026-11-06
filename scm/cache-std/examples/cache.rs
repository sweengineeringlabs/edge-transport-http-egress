//! Minimal usage: build the cache layer with the default config.

use edge_transport_http_egress_cache_api::{CacheConfig, CacheFactory};
use edge_transport_http_egress_cache_std::StdCacheFactory;

fn main() {
    match StdCacheFactory.build_cache_layer(CacheConfig::default()) {
        Ok(_) => println!("edge_transport_http_egress_cache_std layer built"),
        Err(e) => println!("edge_transport_http_egress_cache_std: {e}"),
    }
}
