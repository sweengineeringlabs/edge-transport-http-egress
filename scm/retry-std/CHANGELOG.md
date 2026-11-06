# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

## [0.7.0] - 2026-11-06

### Added

- Issue #30: this package is the implementation half of the former `edge-transport-http-egress-retry` package. Its only public item is `StdRetryFactory`, the `edge_transport_http_egress_retry_api::RetryFactory` implementation. The retry middleware (exponential backoff with jitter, method and status filtering) and the processor that builds it are crate-private. See the `-api` changelog for the migration.
- Tests for `StdRetryFactory`, for the processor that turns a config into a layer, and for the shared backoff scheduler driven by `RetryConfig`.
- The `tracing` feature is kept so a consumer that enables it still resolves; no code is gated on it (it was already inert before the split).
