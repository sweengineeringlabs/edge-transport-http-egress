# ADR-001: Egress HTTP security ownership

**Status:** Superseded
**Date:** 2026-06-12
**Superseded by:** [ADR-002: Security context in outbound HTTP](ADR-002-security-context-propagation.md)

## Original decision

The original decision kept outbound auth, OAuth token refresh, and client TLS specialisations in
this repository. That ownership was reversed as the shared edge-security crates became compatible
with the egress transport. The local `auth`, `oauth`, and `tls` crates are no longer workspace
members.

## Current ownership

- Edge-security owns credential resolution, auth strategies, OAuth, and security policy.
- The consuming composition root assembles those policies with the HTTP client and resilience
  middleware.
- This repository owns the outbound HTTP contract, the reqwest-backed transport, and
  `HttpSecurityContext`, which carries `edge_security_runtime::SecurityContext` through
  `HttpEgress::send_with_context`.
- The bare transport does not derive authorization headers or interpret identity claims.

See [ADR-002](ADR-002-security-context-propagation.md) for the current context contract and
[ADR-005](ADR-005-retry-breaker-composition-moves-out-of-transport.md) / [ADR-006](ADR-006-resilient-splits-into-port-adapter-scoped-to-order-coupled-concerns.md)
for middleware ownership.
