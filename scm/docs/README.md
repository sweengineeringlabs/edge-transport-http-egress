# edge-transport-http-egress

## WHAT

Outbound HTTP client for swe-edge — reqwest-backed transport with a full middleware stack: auth,
retry, rate limiting, circuit breaker, caching, OAuth, TLS, and cassette recording.

Key capabilities:

- **`HttpEgress`** — core trait: sends outbound HTTP requests with health checks and lazy streaming support
- **`HttpStream`** — trait for streaming response bodies (SSE, WebSocket)
- **`HttpRequest`** / **`HttpResponse`** / **`HttpStreamResponse`** — value objects: form parts, SSE events, WebSocket channels
- **`HttpConfigValidator`** — validation chain for request/response inspection before dispatch
- Workspace middleware crates: `auth`, `breaker`, `cache`, `cassette`, `oauth`, `rate`, `retry`, `tls`, `transport`
- Each middleware crate is independent and opt-in; compose as needed via reqwest-middleware

## WHY

| Problem | Solution |
|---------|----------|
| Retry, circuit-breaking, and rate limiting re-implemented per HTTP client | Each concern is an independent middleware crate; compose by adding to the reqwest-middleware stack |
| Auth token injection varies across services (Bearer, API key, OAuth) | `auth` crate provides a pluggable `TokenInjector` trait; `oauth` handles refresh token flows |
| Integration tests need to record/replay HTTP interactions | `cassette` crate captures and replays request/response pairs; deterministic test scenarios without a live backend |
| TLS certificate management scattered | `tls` crate centralises cert/key loading from config; applied as a reqwest layer |
| Diamond dep conflicts when HTTP egress types change | One crate, one tag — all consumers pin the same version; kgraph detects conflicts pre-commit |
