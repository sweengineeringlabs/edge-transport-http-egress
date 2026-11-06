# edge-transport-http-egress-breaker-std

The standard circuit-breaker middleware: a per-host breaker (moka + tokio) that plugs into a
`reqwest-middleware` client. Its only public item is `StdBreakerFactory`, which implements
`edge_transport_http_egress_breaker_api::BreakerFactory`; every concrete type is crate-private.

```rust
use edge_transport_http_egress_breaker_api::{BreakerConfig, BreakerFactory};
use edge_transport_http_egress_breaker_std::StdBreakerFactory;

let layer = StdBreakerFactory.build_breaker_layer(BreakerConfig::default())?;
// reqwest_middleware::ClientBuilder::new(client).with_arc(layer)
```

```bash
cargo test -p edge-transport-http-egress-breaker-std
```
