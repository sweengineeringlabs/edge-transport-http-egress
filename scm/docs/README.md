# Workspace overview

This workspace provides a bare outbound HTTP transport and reusable client-side resilience
middleware. See the [architecture guide](architecture.md) for the transport/server boundary.

## Packages

| Package | Responsibility |
| --- | --- |
| `edge-transport-http-egress-transport` | HTTP request/response contracts, `HttpEgress`, reqwest-backed outbound transport, and streaming support |
| `edge-transport-http-egress-retry` | Retry middleware and retry policy |
| `edge-transport-http-egress-rate` | Per-host client-side rate limiting |
| `edge-transport-http-egress-breaker` | Per-host circuit breaking |
| `edge-transport-http-egress-cache` | HTTP response caching |
| `edge-transport-http-egress-cassette` | Deterministic HTTP record/replay for tests |
| `edge-transport-http-egress-resilient-port` | Contract for applying the resilience chain |
| `edge-transport-http-egress-resilient` | Adapter that composes retry, rate, breaker, cache, and cassette middleware |

Applications choose and assemble their outbound stack. Authentication, OAuth, TLS policy, and
load balancing belong to that composition root; the transport crate carries the security context
used by the outbound contract but does not implement those policies. See the decisions in [`adr/`](adr/).
