# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- `HttpTransportSvc::configure_http_builder(cb, cfg)` is now **`pub`** — the canonical `HttpConfig` → `reqwest::ClientBuilder` mapping (timeouts, user-agent, redirect policy, default headers). Additive, no behaviour change. A composition root assembling its own middleware stack applies `HttpConfig` through this single implementation instead of re-deriving it, eliminating the mapping duplication flagged in edge-bootstrap ADR-008. ([#29](https://github.com/sweengineeringlabs/edge-transport-http-egress/issues/29))
- `HttpTransportSvc::egress_from_middleware_client(client, config)` — the construction seam for a composition root: wrap an already-built `reqwest_middleware::ClientWithMiddleware` (with the caller's own TLS + resilient + auth chain) as an `HttpEgress`. ([#28](https://github.com/sweengineeringlabs/edge-transport-http-egress/issues/28))
- `HttpTransportSvc::plain_http_stream(config)` — the bare `HttpStream` primitive, the streaming counterpart of `plain_http_egress` (no middleware layers).

### Removed

- **Breaking:** `transport` is now a **bare HTTP primitive** — every top-level composition constructor is removed. Its only construction entry points are `plain_http_egress`, `plain_http_stream`, and `egress_from_middleware_client`. Composition of the TLS + `retry → rate → breaker → cache → cassette` + auth/oauth stack moves to the composition root (see `edge-bootstrap` ADR-008 / [#37](https://github.com/sweengineeringlabs/edge-bootstrap/issues/37)). **Removed symbols** (grep your codebase for these): `default_http_egress`, `default_http_egress_with_config`, `default_http_stream_outbound`, `http_egress_from_config`, `http_egress_from_config_with_auth`, `http_egress_from_config_with_tls`, `http_egress_from_config_with_oauth`, `plain_http_egress_with_oauth`, `build_default_egress`, `build_oauth_middleware`, and the `HttpEgressSvcFactory` / `HttpStreamSvcFactory` SAF factories. Migration: use `plain_http_egress` / `plain_http_stream` for a bare client, or `egress_from_middleware_client` to wrap a self-composed middleware client. ([#28](https://github.com/sweengineeringlabs/edge-transport-http-egress/issues/28), ADR-005/006)
- **Breaking:** the `auth`, `oauth`, and `tls` Cargo features and their optional `edge-security-transport-egress-http` / `-oauth` / `-tls` dependencies are removed (there are no default features now). The `SecurityAuthMiddleware` bridge is removed — a composition root that needs egress auth builds it against `edge-security` directly.
- **Breaking:** `HttpEgressBuildError` is collapsed to a single `Reqwest` variant; the `Resilient`, `Tls`, `OAuth`, and `Config` variants and their `From` impls are removed (those failures now belong to the composition root's own error types).
- **Breaking:** the `loadbalancer` Cargo feature (see Added below) has been reverted — backend-pool selection decides *where* a request goes, unlike `retry`/`rate`/`breaker`/`cache`/`cassette`, which all operate on a request already addressed to one resolved destination. That's a routing/topology decision that doesn't belong in this crate; it now lives at the process composition root instead (`edge-bootstrap`'s `LoadBalancedHttpEgress` + `ServiceRegistry`, see `edge-bootstrap`'s ADR-004). Any consumer who adopted the default-on feature: `[loadbalancer]` sections in `application.toml` are no longer read by this crate — resolve backend topology before constructing the `HttpEgress` client instead. `breaker`'s optional pool-reporting integration (`new_with_pool`/`report_outcome`, requiring the `loadbalancer` feature) is removed for the same reason; `edge-bootstrap`'s `LoadBalancedHttpEgress::report_outcome` is the direct equivalent. See ADR-003, [#25](https://github.com/sweengineeringlabs/edge-transport-http-egress/issues/25).

### Added

- `loadbalancer` opt-in Cargo feature (default-on), config-driven via a `[loadbalancer]` `application.toml` section — mirrors `retry`/`rate`/`breaker`/`cache`/`cassette`. Not part of the zero-config `default_http_egress`/`build_default_egress` stack, since `LoadbalancerConfig`'s default has an empty backend list and fails validation. ([#18](https://github.com/sweengineeringlabs/edge-transport-http-egress/issues/18)) — **reverted, see Removed above.**

### Fixed

- `mod auth;` in `core/mod.rs` was compiled unconditionally, so building with the `auth` feature excluded (`--no-default-features` in any combination without `auth`) never actually worked despite `auth` being declared as an opt-in Cargo feature. Gated `mod auth;` and its `SecurityAuthMiddleware` re-export behind `#[cfg(feature = "auth")]`. ([#19](https://github.com/sweengineeringlabs/edge-transport-http-egress/issues/19))
