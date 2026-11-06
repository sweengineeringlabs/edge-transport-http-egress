# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Changed (breaking)

- Issue #30: the `edge-transport-http-egress-resilient` package is renamed `edge-transport-http-egress-resilient-std`. The Rust crate name is now `edge_transport_http_egress_resilient_std`; there is no `edge_transport_http_egress_resilient` crate any more.
- `DefaultResilientLayerComposer` is renamed `StdResilientLayerComposer`. It implements `edge_transport_http_egress_resilient_api::ResilientLayerComposer` as before. `preflight(loader)`, which was an inherent method of the implementation type, is now a method of the `ResilientLayerComposer` contract (call it on any composer), and the implementation package no longer re-exports the contract types: import them from `edge_transport_http_egress_resilient_api`.
- The composer builds each layer through the `-api`/`-std` pair of its package (`StdRetryFactory`, `StdRateFactory`, `StdBreakerFactory`, `StdCacheFactory`, `StdCassetteFactory`) and attaches it with `ClientBuilder::with_arc`.
- Migration: change the dependency and the `use` paths to `edge_transport_http_egress_resilient_std`, and `DefaultResilientLayerComposer` to `StdResilientLayerComposer`, `StdResilientLayerComposer::preflight(&loader)` to `StdResilientLayerComposer.preflight(&loader)` (import `ResilientLayerComposer`), and contract types to the `-api` crate.

### Added

- Tests for `preflight` (no section, one section, an invalid section), for `apply_from_config` and for `apply_defaults` with an unreadable cassette, and an example that composes the default chain.
