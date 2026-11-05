# edge-transport-http-egress-retry-std

The standard retry middleware: idempotent-method exponential backoff with jitter for a
`reqwest-middleware` client. Its only public item is `StdRetryFactory`, which implements
`edge_transport_http_egress_retry_api::RetryFactory`; every concrete type is crate-private.

```rust
use edge_transport_http_egress_retry_api::{DecorateRequest, RetryConfig, RetryFactory};
use edge_transport_http_egress_retry_std::StdRetryFactory;

let decorated = StdRetryFactory
    .create_processor()
    .decorate(DecorateRequest { config: RetryConfig::default() })?;
// reqwest_middleware::ClientBuilder::new(client).with_arc(decorated.layer)
```

```bash
cargo test -p edge-transport-http-egress-retry-std
```
