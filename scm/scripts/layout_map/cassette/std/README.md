# edge-transport-http-egress-cassette-std

The standard VCR-style record/replay middleware for deterministic end-to-end tests: it records
HTTP interactions to a YAML fixture file (headers and body paths scrubbed) and replays them,
plugging into a `reqwest-middleware` client. Its only public item is `StdCassetteFactory`, which
implements `edge_transport_http_egress_cassette_api::CassetteFactory`; every concrete type is
crate-private.

```rust
use edge_transport_http_egress_cassette_api::{CassetteConfig, CassetteFactory};
use edge_transport_http_egress_cassette_std::StdCassetteFactory;

let layer = StdCassetteFactory.build_cassette_layer(CassetteConfig::default(), "my_cassette")?;
// reqwest_middleware::ClientBuilder::new(client).with_arc(layer)
```

```bash
cargo test -p edge-transport-http-egress-cassette-std
```
