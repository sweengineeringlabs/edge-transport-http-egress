# Workspace overview

This workspace provides a bare outbound HTTP transport and reusable client-side resilience
middleware. See the [architecture guide](architecture.md) for the transport/server boundary.

## Packages

| Package | Responsibility |
| --- | --- |
| `edge-transport-http-egress-transport-api` | HTTP request/response contracts, `HttpEgress`, `HttpStream` and `TransportFactory` |
| `edge-transport-http-egress-transport-std` | Reqwest-backed outbound transport and streaming support (`StdTransportFactory`) |
| `edge-transport-http-egress-retry-api` / `-retry-std` | Retry contract; retry middleware and retry policy |
| `edge-transport-http-egress-rate-api` / `-rate-std` | Rate-limit contract; per-host client-side rate limiting |
| `edge-transport-http-egress-breaker-api` / `-breaker-std` | Circuit-breaker contract; per-host circuit breaking |
| `edge-transport-http-egress-cache-api` / `-cache-std` | Cache contract; HTTP response caching |
| `edge-transport-http-egress-cassette-api` / `-cassette-std` | Record/replay contract; deterministic HTTP record/replay for tests |
| `edge-transport-http-egress-resilient-api` | Contract for applying the resilience chain |
| `edge-transport-http-egress-resilient-std` | Adapter that composes retry, rate, breaker, cache, and cassette middleware |

Applications choose and assemble their outbound stack. Authentication, OAuth, TLS policy, and
load balancing belong to that composition root; the transport crate carries the security context
used by the outbound contract but does not implement those policies. See the decisions in [`adr/`](adr/).
