# edge-transport-http-egress

Rust workspace for outbound HTTP transport and reusable resilience middleware.

The workspace provides a bare, reqwest-backed HTTP transport plus separate retry, rate-limit,
circuit-breaker, cache, and cassette crates. `resilient` assembles the resilience chain through
`resilient-port`; applications own higher-level composition such as authentication, OAuth, TLS
policy, and load balancing. The transport package does not own server listeners or connection
lifecycles.

See [the workspace overview](scm/docs/README.md) and [the architecture guide](scm/docs/architecture.md)
for package responsibilities and the composition boundary.
