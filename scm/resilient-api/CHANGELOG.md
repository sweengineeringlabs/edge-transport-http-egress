# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

## [0.7.0] - 2026-11-06

### Changed (breaking)

- Issue #30: the `edge-transport-http-egress-resilient-port` package is renamed `edge-transport-http-egress-resilient-api`. The Rust crate name is now `edge_transport_http_egress_resilient_api`; there is no `edge_transport_http_egress_resilient_port` crate any more. `ResilientLayerComposer` gains a required `preflight(&self, loader)` method (the visibility summary of which layers a config would activate), which was previously an inherent method of the implementation type. The rest of the contract is unchanged: `ResilientLayerComposer`, `ResilientConfigRequest`, `ResilientDefaultsRequest`, `ResilientError` and the `CassetteConfig` re-export. The package has no `api/` source directory: modules sit at the source root.
- It depends on `edge-transport-http-egress-cassette-api` for `CassetteConfig`.
- Migration: change the dependency and the `use` paths from `edge_transport_http_egress_resilient_port` to `edge_transport_http_egress_resilient_api`.
