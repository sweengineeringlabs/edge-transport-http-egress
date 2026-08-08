# ADR-006: `resilient` Splits Into a Port/Adapter Pair, Scoped to the Order-Coupled Concerns Only

**Status:** Proposed
**Date:** 2026-08-08
**Amends:** [ADR-005](ADR-005-retry-breaker-composition-moves-out-of-transport.md) — refines its
"new crate" into a port/adapter pair and narrows its scope; ADR-005's core mandate (`transport`
gains zero knowledge of composed concerns) is unaffected.
**See also:** [ADR-004](ADR-004-extract-retry-breaker-to-shared-crates.md) (this repo),
[edge-transport-grpc-egress ADR-004](https://github.com/sweengineeringlabs/edge-transport-grpc-egress/blob/dev/scm/docs/adr/ADR-004-remove-resilience-config-from-transport-primitive.md)
**Tracking:** [#27](https://github.com/sweengineeringlabs/edge-transport-http-egress/issues/27)
(supersedes #26 for implementation scope), consumer-side: [edge-bootstrap#36](https://github.com/sweengineeringlabs/edge-bootstrap/issues/36)

---

## Mandate

`edge-transport-http-egress-resilient` splits into two crates, mirroring the port/adapter pattern
`edge-security` already uses for this exact class of pluggable egress concern:

- **`edge-transport-http-egress-resilient-port`** — trait contract, zero implementation.
- **`edge-transport-http-egress-resilient`** — real implementation (unsuffixed name, matching
  `edge-security-transport-egress-http-oauth`'s naming convention, not
  `edge-security-transport-egress-http-oauth-port`'s).

Scope is **retry, rate, breaker, cache, cassette only** — not all 8 concerns `#27` names. `auth`,
`oauth`, and `tls` stay exactly as they are today: separately caller-supplied, unaffected by this
migration. `#27` itself already says as much: *"auth/oauth/tls — not ours to classify either way;
already externally owned by edge-security, consumed as caller-supplied strategy objects, not
something this migration creates or extracts."* This ADR makes that boundary explicit rather than
implicit, and gives a structural reason for it (below), not just a jurisdictional one.

## Why

### The port/adapter split matches an existing, working precedent

Checked `edge-security`'s actual on-disk structure, not just its exports: `port/transport/http/egress/{bearer,oauth,tls}`
(trait contracts, suffixed `-port` in `Cargo.toml`'s `name`) and
`adapter/transport/http/egress/{bearer,oauth,tls}` (real implementations, unsuffixed). Confirmed
`transport/Cargo.toml` here depends directly on the **unsuffixed** adapter packages
(`edge-security-transport-egress-http-oauth`, `edge-security-transport-egress-http-tls`), not the
`-port` ones — the port crate is an internal contract the adapter's own implementation is built
against (e.g. `bearer`'s `Validator` trait, used by its own `HttpEgressAuthStrategy` impl), not
necessarily the only thing external consumers see. `resilient` should follow the same shape:
real composition code lives in the adapter, a narrower contract lives in the port, and consumers
(`transport` today, `edge-bootstrap` after `#27`/`#36` land) depend on the adapter.

### Why bundle retry/rate/breaker/cache/cassette together, but not with auth/oauth/tls

`auth`/`oauth`/`tls` are already independently pluggable today — `transport_svc.rs` exposes them as
three **separate** functions (`http_egress_from_config_with_auth`/`_with_tls`/`_with_oauth`), never
merged into one. They have no ordering dependency on each other.

Retry/rate/breaker/cache/cassette are different: `reqwest_middleware::ClientBuilder` requires every
layer registered **before** `.build()` is ever called, in a specific relative order —
`with_optional_layers`'s actual sequence today is `retry → rate → breaker → cache → cassette`
(verified in `transport_svc.rs` directly). `#27`'s own empirical research already found that
splitting retry/breaker out while leaving rate/cache/cassette behind would invert this order — a
real regression (breaker would stop being re-checked per retry attempt). That's a structural reason
these five must stay assembled as one unit; auth/oauth/tls have no equivalent constraint forcing
them to bundle with anything.

One caveat found while reading `transport_svc.rs` in full: auth is *not* fully independent of the
resilient chain's assembly *order*, even though it's a separate concern. `http_egress_from_config_with_auth`
adds `SecurityAuthMiddleware` **before** calling `with_optional_layers` — since `reqwest_middleware`'s
first-added layer is outermost, auth ends up wrapping the entire resilient chain. Whoever does final
assembly (see "What changes" below) must preserve that relative placement; it isn't `resilient`'s
job to know about auth, but it is the composer's job to place `resilient`'s output correctly
relative to auth.

## What changes

- **`edge-transport-http-egress-resilient-port`** (new): trait contract for "decorate a
  `ClientBuilder` with the resilient chain" — something in the shape of
  `fn apply(&self, builder: ClientBuilder, config: ResilientConfig) -> Result<ClientBuilder, ResilientError>`.
  Zero implementation, zero dependency on `reqwest_middleware` internals beyond the type signature.
- **`edge-transport-http-egress-resilient`** (new): implements the port trait. Contains exactly
  what `with_optional_layers`'s retry/rate/breaker/cache/cassette branches do today — lifted
  verbatim (same config-section-driven activation, same ordering), not reimplemented. Depends on
  `edge-transport-http-egress-{retry,rate,breaker,cache,cassette}` (retargeted to the shared
  `edge-transport-retry`/`edge-transport-breaker` crates per ADR-004, once that lands).
  **Exposes a `ClientBuilder → ClientBuilder` decorator, not a "return a finished `Box<dyn HttpEgress>`"
  function** — this is the key difference from `#27`'s current all-in-one-crate proposal.
  Preserving builder-level composability means a caller can still combine `resilient`'s output with
  auth/tls at the correct relative position, rather than being forced to accept a single opaque,
  auth-less client the way today's `default_http_egress()` is.
- **`transport/Cargo.toml`**: `retry`/`rate`/`breaker`/`cache`/`cassette` Cargo features and their
  `dep:` entries removed, per ADR-005's original mandate.
- **`main/src/saf/transport_svc.rs`**: `with_optional_layers`/`build_default_egress`'s
  composition logic moves to `edge-transport-http-egress-resilient`. `plain_http_egress`,
  `configure_http_builder`, `create_config_builder`, `validate_http_config`, `validate` stay —
  these operate on `transport`'s own `HttpConfig` domain type or produce the bare primitive, not on
  any of the 5 relocated concerns.

### Open question, not resolved here

Where do the *top-level* combining functions land — `http_egress_from_config_with_auth`/`_with_tls`/
`_with_oauth`, `default_http_egress`/`default_http_egress_with_config`, `plain_http_egress_with_oauth`?
These are themselves the "assemble everything into one finished client" step, combining
`resilient`'s decorator with auth/tls/oauth in the correct order. Per `#27`'s stated goal that
`transport`'s only construction entry point becomes `plain_http_egress`, they can't stay in
`transport`. Two options, not decided here:

1. `edge-transport-http-egress-resilient` also exposes convenience top-level functions (mirroring
   today's shape: "give me a fully assembled default client, optionally with auth") for simple
   consumers, *alongside* the lower-level `ClientBuilder` decorator for composition roots that want
   more control.
2. They move entirely into `edge-bootstrap`'s `RuntimeBuilder`, which already does exactly this
   kind of top-level assembly for HTTP/gRPC servers, egress clients, observability, and lifecycle —
   consistent with `edge-bootstrap`'s own position that composition-root work belongs at the
   composition root, not scattered into egress-side crates.

Leaning toward option 1 for the convenience surface (so non-`edge-bootstrap` consumers of this repo
aren't forced into a full composition-root just to get a working default client) plus option 2 for
`edge-bootstrap` specifically (which wants the `ClientBuilder`-level primitive so it can compose
its *own* defaults, not inherit this crate's). Not settled — flagging before implementation starts,
per the same "cheaper to change now" reasoning as the port/adapter split itself.

## Consequences

- Same breaking-change profile as ADR-005 for any direct consumer of `transport`'s `retry`/`rate`/
  `breaker`/`cache`/`cassette` features — they migrate to depend on
  `edge-transport-http-egress-resilient` instead.
- `edge-bootstrap`'s own migration (`#36`) gets simpler in one respect (only 5 concerns to reason
  about for the new crate, not 8) and stays open in another (the top-level convenience functions'
  landing spot affects exactly what `#36`'s 10 call sites retarget to).
- gRPC's existing `resilient` crate does not have this split today (single implementation crate, no
  separate port trait) — out of scope here to retrofit it, but worth the same treatment eventually
  for consistency, tracked separately if pursued.
