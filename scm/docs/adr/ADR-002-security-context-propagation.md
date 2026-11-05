# ADR-002: Security context in outbound HTTP

**Status:** Accepted
**Date:** 2026-06-12
**Supersedes:** The earlier local `CredentialResolver` and `HttpOutbound` proposal in this ADR
**Related:** [ADR-001](ADR-001-egress-security-specialisations.md), [ADR-005](ADR-005-retry-breaker-composition-moves-out-of-transport.md), [ADR-006](ADR-006-resilient-splits-into-port-adapter-scoped-to-order-coupled-concerns.md)

## Decision

The outbound HTTP contract carries the caller's `SecurityContext` through
`HttpEgress::send_with_context`. The no-context convenience method `send` delegates to
`send_with_context` with an unauthenticated context. A local `HttpSecurityContext` newtype keeps the
foreign runtime type out of the transport contract's method signature while retaining lossless
conversion to and from `edge_security_runtime::SecurityContext`.

`send_with_context` is the required operation for `HttpEgress` implementors. Decorators such as the
metrics wrapper must forward the supplied context to the wrapped egress. The bare reqwest transport
does not interpret identity, claims, tokens, or security policy; authentication and context-derived
headers are applied by the consuming composition root and its middleware.

## Ownership

Credential resolution and auth strategies are owned by edge-security. The local egress auth crate
and its `CredentialResolver` were removed when auth migrated to edge-security's bearer crate
(commit `9c9c8b1`). This repository owns only the outbound HTTP contract and the context value it
accepts. It must not recreate credential resolution or auth policy here.

## Compatibility

`HttpEgress::send_with_context` is the primary trait method. `send` remains a convenience shim that
supplies an unauthenticated context. Implementations that need context-sensitive behavior override
`send_with_context`; decorators preserve the same context when calling their inner implementation.

The default reqwest transport continues to send the caller's `HttpRequest` as supplied. It does not
derive authorization headers from context fields. This keeps identity propagation separate from
security policy and avoids inventing a header mapping in the transport layer.

## Verification

Tests cover context construction and type identity, context-aware HTTP requests, error behavior,
and the unauthenticated `send` path. The dependency must resolve the same immutable
`edge-security-runtime` tag used by the consuming application.
