//! Minimal usage: build the cache layer with the default config.

fn main() {
    match edge_transport_http_egress_cache::HttpCacheSvc::build_cache_layer(
        edge_transport_http_egress_cache::CacheConfig::default(),
    ) {
        Ok(_) => println!("edge_transport_http_egress_cache layer built"),
        Err(e) => println!("edge_transport_http_egress_cache: {e}"),
    }
}
