# edge-transport-http-egress-transport-std

The standard HTTP egress transport: a reqwest-backed `HttpEgress` and `HttpStream` (with optional
WebSocket support behind the `websocket` feature) and a metrics-observing decorator. Its only public
item is `StdTransportFactory`, which implements
`edge_transport_http_egress_transport_api::TransportFactory`; every concrete type is crate-private.

```rust
use edge_transport_http_egress_transport_api::{HttpConfig, TransportFactory};
use edge_transport_http_egress_transport_std::StdTransportFactory;

let egress = StdTransportFactory.plain_http_egress(HttpConfig::default())?;
```

```bash
cargo test -p edge-transport-http-egress-transport-std
```
