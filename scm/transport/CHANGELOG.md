# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Removed

- **Breaking:** the `loadbalancer` Cargo feature (see Added below) has been reverted — backend-pool selection decides *where* a request goes, unlike `retry`/`rate`/`breaker`/`cache`/`cassette`, which all operate on a request already addressed to one resolved destination. That's a routing/topology decision that doesn't belong in this crate; it now lives at the process composition root instead (`edge-bootstrap`'s `LoadBalancedHttpEgress` + `ServiceRegistry`, see `edge-bootstrap`'s ADR-004). Any consumer who adopted the default-on feature: `[loadbalancer]` sections in `application.toml` are no longer read by this crate — resolve backend topology before constructing the `HttpEgress` client instead. `breaker`'s optional pool-reporting integration (`new_with_pool`/`report_outcome`, requiring the `loadbalancer` feature) is removed for the same reason; `edge-bootstrap`'s `LoadBalancedHttpEgress::report_outcome` is the direct equivalent. See ADR-003, [#25](https://github.com/sweengineeringlabs/edge-transport-http-egress/issues/25).

### Added

- `loadbalancer` opt-in Cargo feature (default-on), config-driven via a `[loadbalancer]` `application.toml` section — mirrors `retry`/`rate`/`breaker`/`cache`/`cassette`. Not part of the zero-config `default_http_egress`/`build_default_egress` stack, since `LoadbalancerConfig`'s default has an empty backend list and fails validation. ([#18](https://github.com/sweengineeringlabs/edge-transport-http-egress/issues/18)) — **reverted, see Removed above.**

### Fixed

- `mod auth;` in `core/mod.rs` was compiled unconditionally, so building with the `auth` feature excluded (`--no-default-features` in any combination without `auth`) never actually worked despite `auth` being declared as an opt-in Cargo feature. Gated `mod auth;` and its `SecurityAuthMiddleware` re-export behind `#[cfg(feature = "auth")]`. ([#19](https://github.com/sweengineeringlabs/edge-transport-http-egress/issues/19))
