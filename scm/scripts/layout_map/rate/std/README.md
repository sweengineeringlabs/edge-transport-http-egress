# edge-transport-http-egress-rate-std

The standard rate-limit middleware: a per-host token-bucket limiter (moka + tokio) that plugs into
a `reqwest-middleware` client. Its only public item is `StdRateFactory`, which implements
`edge_transport_http_egress_rate_api::RateFactory`; every concrete type is crate-private.

```rust
use edge_transport_http_egress_rate_api::{RateConfig, RateFactory};
use edge_transport_http_egress_rate_std::StdRateFactory;

let layer = StdRateFactory.build_rate_layer(RateConfig::default())?;
// reqwest_middleware::ClientBuilder::new(client).with_arc(layer)
```

```bash
cargo test -p edge-transport-http-egress-rate-std
```
