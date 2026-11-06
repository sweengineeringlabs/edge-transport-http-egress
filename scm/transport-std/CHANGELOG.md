# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- Issue #30: this package is the implementation half of the former `edge-transport-http-egress-transport` package. Its only public item is `StdTransportFactory`, the `edge_transport_http_egress_transport_api::TransportFactory` implementation. The reqwest-backed `HttpEgress` and `HttpStream` (with WebSocket support behind the `websocket` feature), the metrics-observing decorator and the default validator are crate-private. See the `-api` changelog for the migration.
- Tests for `StdTransportFactory`: config validation (accepted and rejected), `configure_http_builder` with valid and invalid default headers, and building a plain egress and stream that carry the given config.
