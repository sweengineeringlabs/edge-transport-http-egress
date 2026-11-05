# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed (breaking)

- Issue #30: the `edge-transport-http-egress-rate` package is split into `edge-transport-http-egress-rate-api` (this contract) and `edge-transport-http-egress-rate-std` (the implementation). The Rust crate names are `edge_transport_http_egress_rate_api` and `edge_transport_http_egress_rate_std`; there is no `edge_transport_http_egress_rate` crate any more.
- The contract has no `api/` source directory: modules sit at the source root, and every type is re-exported from the crate root as before.
- `RateLayerRateMetrics`, `HttpRateSvcProcessor` and the `ProcessorFactory`, `ValidatorFactory` and `RateMetricsFactory` structs are no longer public. Use `edge_transport_http_egress_rate_std::StdRateFactory`, which implements the new `RateFactory` trait: `create_processor()`, `create_validator()`, `create_config_builder()` and `build_rate_layer(config)`. `build_rate_layer` returns `Arc<dyn RateLayer>`; `RateLayer` is a `reqwest_middleware::Middleware` that also implements `RateMetrics`, so attach it with `ClientBuilder::with_arc(layer)` and read the token rate from it directly.
- `RateConfig` keeps its schema, `Default`, `from_config` and section registration; they now live in the contract package.
- The config builder is still seeded with the application name `edge-transport-http-egress-rate`, not the `-std` package name: configbuilder uses the name as a directory segment when it searches for configuration, so changing it would move where existing deployments' configuration is found.
- Migration: replace `HttpRateSvcProcessor::build_rate_layer(cfg)` with `StdRateFactory.build_rate_layer(cfg)` (import `RateFactory`), `ProcessorFactory::create()` with `StdRateFactory.create_processor()`, `ValidatorFactory::create()` with `StdRateFactory.create_validator()`, and drop `RateMetricsFactory::from_layer(layer)` (the layer is already a `RateMetrics`).

### Added

- Tests for every `StdRateFactory` construction path, and an example that parses a rate-limit configuration.

## [0.6.0] - 2026-11-05

### Added
- Initial project structure
