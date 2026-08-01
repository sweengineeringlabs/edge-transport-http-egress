# ADR-003: Backend-Pool Load Balancing Removed — Moves to the Composition Root

**Status:** Accepted
**Date:** 2026-08-01
**See also:** [edge-bootstrap ADR-004](https://github.com/sweengineeringlabs/edge-bootstrap/blob/dev/scm/docs/3-design/adr/004-backend-pool-load-balancing-moves-to-composition-root.md) — where this capability now lives
**Tracking:** [#25](https://github.com/sweengineeringlabs/edge-transport-http-egress/issues/25), [edge-bootstrap#24](https://github.com/sweengineeringlabs/edge-bootstrap/issues/24)

---

## Mandate

Remove the `loadbalancer` Cargo feature (added in an earlier commit, shipped default-on in v0.4.6) from `transport` and the pool-reporting integration it enabled in `breaker`. Delete the `scm/loadbalancer` crate. This crate no longer owns backend-pool selection for outbound calls.

## Why

`transport`'s middleware stack — `auth`/`retry`/`rate`/`breaker`/`cache`/`cassette` — all operate on a request already addressed to one resolved destination: they decorate *how* a call is made (retried, rate-limited, circuit-broken, cached, recorded), never *where* it goes. `loadbalancer` was the exception: it read a static `[loadbalancer]` config section, built a `BackendPoolInstance`, and rewrote every outgoing request's URL to a selected backend. That's a routing/topology decision, and this crate — a per-call HTTP transport library with no visibility into deployment topology — is the wrong place to make it. The composition root (`edge-bootstrap`, which already wires ingress + egress + lifecycle for a deployable process) is.

## What changed

- `transport`: `loadbalancer` feature removed from `Cargo.toml` (dependency, feature flag, `default = [...]` entry). `[loadbalancer]` config loading (`preflight`, `with_optional_layers`) and the `HttpEgressBuildError::Loadbalancer` variant removed from `transport_svc.rs` and the error types. Loadbalancer-specific tests removed (`tests/loadbalancer_dep_int_test.rs` deleted; loadbalancer-specific cases removed from `tests/http_egress_from_config_int_test.rs` and `tests/coverage_int_test.rs`).
- `breaker`: `loadbalancer` feature, `swe-edge-loadbalancer` dependency, `BreakerLayerBreakerMetrics::new_with_pool`, `HttpBreakerSvcProcessor::build_breaker_layer_with_pool`, and the pool-reporting branch in `BreakerLayerBreakerMetrics::handle` all removed. `breaker`'s own circuit-tripping (`admit`/`record`, and the `HostBreaker::is_open`/`is_half_open`/`is_closed` state-inspection trait) is untouched and stays — that's still a valid transport-layer concern, operating on one host, independent of any pool.
- `scm/loadbalancer` crate: deleted outright, not extracted for reuse. It was a `reqwest_middleware::Middleware`, built to plug into a `ClientBuilder` chain — not consumable by `edge-bootstrap`, which only ever sees `Arc<dyn HttpEgress>` trait objects. `edge-bootstrap`'s `LoadBalancedHttpEgress` calls `swe-edge-loadbalancer`'s egress subdomain (`BackendPool`/`Strategy`/`Outcome`) directly instead — same underlying library, different (and now correctly-placed) caller.

## Outcome-reporting parity

`breaker`'s removed pool integration let a circuit trip evict a backend from rotation and a recovery restore it (`report_outcome` against the pool). This was not dropped silently: `edge-bootstrap`'s `LoadBalancedHttpEgress::report_outcome(backend_id, outcome)` is the direct equivalent, landed and tested (`edge-bootstrap#24`) *before* this removal shipped — see that issue's Sequencing section and edge-bootstrap ADR-004.

## Consequences

- **Breaking change** for any consumer who adopted the default-on `loadbalancer` feature between v0.4.6 and this change: `[loadbalancer]` sections in `application.toml` are no longer read by `transport`. Backend topology must be resolved before constructing the `HttpEgress` client handed to the consumer's runtime (e.g. via `edge-bootstrap`'s `RuntimeBuilder`/`ServiceRegistry`).
- `cargo test --workspace` (default features) has zero references to `swe-edge-loadbalancer` in `transport` or `breaker`.
- See `CHANGELOG.md` for the consumer-facing summary.
