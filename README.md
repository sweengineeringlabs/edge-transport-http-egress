# edge-transport-http-egress

Rust workspace for outbound HTTP transport and reusable resilience middleware.

The workspace provides a bare, reqwest-backed HTTP transport plus separate retry, rate-limit,
circuit-breaker, cache, and cassette crates. Each is split into a `-api` contract package and a
`-std` implementation package. `resilient-std` assembles the resilience chain through
`resilient-api`; applications own higher-level composition such as authentication, OAuth, TLS
policy, and load balancing. The transport package does not own server listeners or connection
lifecycles.

See [the workspace overview](scm/docs/README.md) and [the architecture guide](scm/docs/architecture.md)
for package responsibilities and the composition boundary.

## HTTP metrics in 0.8

`TransportFactory::observe_http_egress` now accepts an `HttpMetricsObserver` callback.
The application binds this callback to its metrics provider; the transport no longer
depends on the observability workspace. There is no compatibility overload.

```rust,ignore
use std::sync::Arc;
use edge_transport_http_egress_transport_api::{HttpMetricKind, HttpMetricsObserver};

let observer: HttpMetricsObserver = Arc::new(move |event| {
    match event.kind {
        HttpMetricKind::Counter => { /* add event.value to event.name */ }
        HttpMetricKind::Histogram => { /* observe event.value for event.name */ }
    }
    // Pass event.labels to the provider and handle recording errors here.
});
let observed = factory.observe_http_egress(inner, observer);
```

The callback receives `edge_egress_requests_total`, `edge_egress_latency_us`, and
`edge_egress_errors_total`, retaining the `method` label. Latency is measured in
microseconds. Streaming observations cover opening the response; later body errors
do not change the opening measurements. An HTTP error status returned successfully
does not increment the transport error counter. Callbacks run synchronously and
must copy borrowed data if retaining it. Recording failures belong to the callback;
panics propagate. Retry, rate limit, circuit breaker, cache, and cassette behavior
is unchanged.
