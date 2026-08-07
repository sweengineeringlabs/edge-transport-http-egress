# ADR-005: Retry and Breaker Composition Moves Out of the Transport Crate

**Status:** Proposed
**Date:** 2026-08-07
**Amends:** [ADR-003](ADR-003-loadbalancer-moves-to-composition-root.md) — supersedes its treatment
of `retry`/`breaker` specifically; the `loadbalancer` conclusion there is unaffected.
**See also:** [ADR-004](ADR-004-extract-retry-breaker-to-shared-crates.md) (this repo),
[edge-transport-grpc-egress ADR-004](https://github.com/sweengineeringlabs/edge-transport-grpc-egress/blob/dev/scm/docs/adr/ADR-004-remove-resilience-config-from-transport-primitive.md)
**Tracking:** [#26](https://github.com/sweengineeringlabs/edge-transport-http-egress/issues/26)

---

## Mandate

This crate's `transport` crate must have **zero knowledge** of `retry`/`breaker` — no Cargo
dependency, no feature flags gated on them, no error variants tied to their types. All
retry/breaker composition currently living in `transport` moves to a new crate,
`edge-transport-http-egress-resilient`, mirroring the separation `edge-transport-grpc-egress`
already has via its own `resilient` crate.

## Why

An impact analysis for ADR-004's extraction work found `transport` does not merely depend on
retry/breaker — it *is* their composition root:

- `transport/Cargo.toml` declares `retry`/`breaker` as its own Cargo features
  (`retry = ["dep:edge-transport-http-egress-retry"]`, same for `breaker`).
- `main/src/saf/transport_svc.rs` calls `edge_transport_http_egress_retry::HttpRetrySvc::decorate`
  and `edge_transport_http_egress_breaker::HttpBreakerSvcProcessor::build_breaker_layer` directly
  to assemble the client.
- `HttpEgressBuildError` — this crate's own public build-error primitive — has
  `#[cfg(feature = "retry")] impl From<RetryError>` and the equivalent for `BreakerError`
  (`core/error/http_egress_build_error.rs`).

That's two responsibilities fused into one crate: "be an HTTP transport client" and "know how to
make an HTTP transport client resilient." Duplicating the underlying retry/breaker *logic* across
`edge-transport-http-egress` and `edge-transport-grpc-egress` (ADR-004's finding) was a direct
consequence of transport crates each independently owning this composition responsibility instead
of a decorator layer owning it uniformly. Fixing the logic duplication without also fixing this
containment problem would leave the root cause in place.

`edge-transport-grpc-egress` already has the right shape for this: a separate
`edge-transport-grpc-egress-resilient` crate does the actual `GrpcRetryClient`/`GrpcBreakerClient`
composition, not the base `transport` crate. This repo has no equivalent split today — retry/breaker
composition has always lived directly inside `transport`.

## What changes

- New crate: `edge-transport-http-egress-resilient`. Depends on `transport` (not the reverse),
  plus `edge-transport-retry`/`edge-transport-breaker` (or this repo's own `retry`/`breaker`
  crates, until ADR-004's extraction lands). Owns the composition logic moved from
  `transport_svc.rs`, and its own error type instead of `HttpEgressBuildError::Retry`/`::Breaker`.
- `transport/Cargo.toml`: `retry`/`breaker` Cargo features and their `dep:` entries removed.
- `main/src/saf/transport_svc.rs`: retry/breaker decoration calls removed.
- `core/error/http_egress_build_error.rs`: `From<RetryError>`/`From<BreakerError>` impls (and the
  corresponding `HttpEgressBuildError` variants) removed.

## Consequences

- **Breaking change** for any consumer using `transport`'s `retry`/`breaker` Cargo features
  directly — they must depend on `edge-transport-http-egress-resilient` instead.
- `transport`'s own public error surface shrinks (`HttpEgressBuildError` loses two variants).
- Sequencing: this depends on ADR-004's shared-crate extraction landing first (or can proceed in
  parallel against this repo's own `retry`/`breaker` crates, then be re-pointed once the shared
  crates exist — either order works, but both must land before `transport` is fully retry/breaker
  -agnostic).
