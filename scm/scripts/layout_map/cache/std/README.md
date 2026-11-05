# edge-transport-http-egress-cache-std

The standard HTTP-cache middleware: a moka-backed response cache with `Cache-Control`, `Vary`,
`ETag` revalidation and stale-while-revalidate, that plugs into a `reqwest-middleware` client. Its
only public item is `StdCacheFactory`, which implements
`edge_transport_http_egress_cache_api::CacheFactory`; every concrete type is crate-private.

```rust
use edge_transport_http_egress_cache_api::{CacheConfig, CacheFactory};
use edge_transport_http_egress_cache_std::StdCacheFactory;

let layer = StdCacheFactory.build_cache_layer(CacheConfig::default())?;
// reqwest_middleware::ClientBuilder::new(client).with_arc(layer)
```

```bash
cargo test -p edge-transport-http-egress-cache-std
```
