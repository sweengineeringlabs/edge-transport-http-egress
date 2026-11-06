# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- Issue #30: this package is the implementation half of the former `edge-transport-http-egress-cassette` package. Its only public item is `StdCassetteFactory`, the `edge_transport_http_egress_cassette_api::CassetteFactory` implementation. The record/replay middleware, fixture store, body scrubber and recorded interactions are crate-private. See the `-api` changelog for the migration.
- Tests for `StdCassetteFactory`, including a corrupt fixture file being refused when the layer is built.
- The `tracing` feature is kept so a consumer that enables it still resolves; no code is gated on it (it was already inert before the split).
