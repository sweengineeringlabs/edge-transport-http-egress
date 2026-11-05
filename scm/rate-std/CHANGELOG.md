# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- Issue #30: this package is the implementation half of the former `edge-transport-http-egress-rate` package. Its only public item is `StdRateFactory`, the `edge_transport_http_egress_rate_api::RateFactory` implementation. The per-host token-bucket limiter, its moka/tokio state and the reqwest-middleware glue are crate-private. See the `-api` changelog for the migration.
- The `tracing` feature is kept so a consumer that enables it still resolves; no code is gated on it (it was already inert before the split).
