# ADR-004: Extract Retry and Circuit-Breaker into Shared, Protocol-Agnostic Crates

**Status:** Proposed
**Date:** 2026-08-07
**See also:** [edge-transport-grpc-egress ADR-003](https://github.com/sweengineeringlabs/edge-transport-grpc-egress/blob/dev/scm/docs/adr/ADR-003-extract-retry-breaker-to-shared-crates.md) — sibling record in the gRPC egress repo
**Tracking:** none yet — file follow-up issues in this repo and `edge-transport-grpc-egress` before implementation starts

---

## Mandate

Extract this crate's `retry` (`edge-transport-http-egress-retry`) and `breaker`
(`edge-transport-http-egress-breaker`) crates' policy/backoff/circuit-state logic into new
standalone, protocol-agnostic crates — `edge-transport-retry` and `edge-transport-breaker` —
consumed by both this repo and `edge-transport-grpc-egress`. This repo's `retry`/`breaker` crates
become thin adapters wiring the shared policy engine to `HttpEgress`, the same shape as how
`edge-loadbalancer` is consumed today (ADR-003) rather than reimplemented per protocol.

## Why

`edge-transport-http-egress-retry` and `edge-transport-grpc-egress-retry` are two fully
independent implementations of the same concern — backoff/jitter, attempt bookkeeping, retry
policy configuration — with no shared dependency between them. Same for
`edge-transport-http-egress-breaker` and `edge-transport-grpc-egress-breaker` (circuit state
machine: open/half-open/closed, failure-rate tracking). Retry and circuit-breaking do not depend
on wire format — both operate on "did this attempt succeed or fail," a judgment already reduced to
a boolean/enum outcome by the time either decorator runs.

Duplicating that judgment logic per protocol has already produced drift: `edge-transport-grpc-egress`
has `resilient`/`auth-bearer` crates with no `http`-side equivalent, and a policy fix (e.g. a
backoff-jitter bug) made in one implementation has no path to the other short of a human noticing
and porting it by hand.

The one genuinely protocol-specific piece is classifying whether a given error is
retryable/trip-worthy at all — that depends on HTTP status codes vs. gRPC status codes. That stays
local to each protocol's adapter crate as a small `ErrorClassifier` implementation; only the
policy/backoff/state-machine core moves out.

## What changes

- New repos: `edge-transport-retry`, `edge-transport-breaker` — protocol-agnostic policy/backoff/
  circuit-state engines, with no dependency on `HttpEgress`/`GrpcEgress` or any wire-format type.
- `edge-transport-http-egress-retry`/`-breaker` (this repo) and
  `edge-transport-grpc-egress-retry`/`-breaker` are retargeted to depend on the new shared crates,
  keeping only their protocol-specific `ErrorClassifier` and the decorator wiring
  (`reqwest_middleware::Middleware` / `tower::Layer` around `HttpEgress`/`GrpcEgress`).
- This repo's public API — the decorator types consumers construct — does not change; only what's
  inside them does.

## Consequences

- A retry/breaker policy fix or improvement lands once, in the shared crate, and both protocols
  pick it up on their next dependency bump — no more manual porting between implementations.
- Adds a new cross-repo dependency edge: both egress repos now depend on
  `edge-transport-retry`/`edge-transport-breaker`, the same shape as the existing
  `edge-loadbalancer` dependency each already carries elsewhere.
- Feature-parity gaps (`resilient`/`auth-bearer` existing only on the gRPC side) are not
  automatically closed by this ADR — that's a separate follow-up once the shared core exists.
- **Implementation not yet started.** This ADR records the decision and naming ahead of the
  extraction work; see the Naming rationale below. Track via a new issue in this repo and in
  `edge-transport-grpc-egress` before work begins.

## Naming rationale

`edge-transport-retry`/`edge-transport-breaker` — not `edge-retry`/`edge-breaker` — because these
decorators specifically wrap a transport call (`HttpEgress`/`GrpcEgress`), not an arbitrary
operation: that's a transport-layer resilience concern. This is a deliberate departure from crates
like `edge-application`/`edge-security`, which span layers beyond transport and are *consumed by*
transport rather than *belonging to* it — a distinction worth naming explicitly rather than
generalizing from "shared crates don't get a `transport` prefix."
