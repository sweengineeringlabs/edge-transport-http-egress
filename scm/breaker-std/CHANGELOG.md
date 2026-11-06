# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

## [0.7.0] - 2026-11-06

### Added

- Issue #30: this package is the implementation half of the former `edge-transport-http-egress-breaker` package. Its only public item is `StdBreakerFactory`, the `edge_transport_http_egress_breaker_api::BreakerFactory` implementation. The per-host breaker, its moka/tokio state and the reqwest-middleware glue are crate-private. See the `-api` changelog for the migration.
- The `tracing` feature is kept so a consumer that enables it still resolves; no code is gated on it (it was already inert before the split).
