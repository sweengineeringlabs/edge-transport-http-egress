# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed (breaking)

- Issue #30: the `edge-transport-http-egress-breaker` package is split into `edge-transport-http-egress-breaker-api` (this contract) and `edge-transport-http-egress-breaker-std` (the implementation). The Rust crate names are `edge_transport_http_egress_breaker_api` and `edge_transport_http_egress_breaker_std`; there is no `edge_transport_http_egress_breaker` crate any more.
- The contract has no `api/` source directory: modules sit at the source root, and every type is re-exported from the crate root as before.
- `BreakerLayerBreakerMetrics`, `HttpBreakerSvcProcessor` and the `ProcessorFactory`, `ValidatorFactory` and `BreakerMetricsFactory` structs are no longer public. Use `edge_transport_http_egress_breaker_std::StdBreakerFactory`, which implements the new `BreakerFactory` trait: `create_processor()`, `create_validator()`, `create_config_builder()` and `build_breaker_layer(config)`. `build_breaker_layer` returns `Arc<dyn BreakerLayer>`; `BreakerLayer` is a `reqwest_middleware::Middleware` that also implements `BreakerMetrics`, so attach it with `ClientBuilder::with_arc(layer)` and read the threshold from it directly.
- `BreakerConfig` keeps its schema, `Default`, `from_config`, section registration and conversion to the shared policy config; they now live in the contract package.
- Migration: replace `HttpBreakerSvcProcessor::build_breaker_layer(cfg)` with `StdBreakerFactory.build_breaker_layer(cfg)` (import `BreakerFactory`), `ProcessorFactory::create()` with `StdBreakerFactory.create_processor()`, `ValidatorFactory::create()` with `StdBreakerFactory.create_validator()`, and drop `BreakerMetricsFactory::from_layer(layer)` (the layer is already a `BreakerMetrics`).

### Fixed

- `StdBreakerFactory::create_config_builder` seeds the config builder with the application name `edge-transport-http-egress-breaker`, the name of the pre-split package, not the `-std` package name. configbuilder uses the name as a directory segment when it searches for configuration, so the first split commit (which seeded `edge-transport-http-egress-breaker-std`) would have moved where existing deployments' configuration is found.

### Added

- Tests for `BreakerConfig`'s conversion into the policy config and for every `StdBreakerFactory` construction path, and an example that parses a breaker configuration.

## [0.6.0] - 2026-11-05

### Changed (breaking)
- The circuit-state machine (`DefaultHostBreaker`, `CircuitBreakerNode`/`HostBreaker` traits,
  `Admission`, `Outcome`, `AdmitRequest`/`AdmitResponse`, `RecordRequest`→`RecordOutcomeRequest`,
  `RecordResponse`→`RecordOutcomeResponse`, and the `is_open`/`is_half_open`/`is_closed`
  introspection DTOs) moved to `edge-transport-breaker`/`edge-transport-breaker-policy`.
  `edge-transport-grpc-egress`'s implementation was chosen canonical in the Phase 1 design review
  (a stateless `&self` design, more composable across two different callers than this crate's
  `&mut self` one) — so this crate's own implementation and its dedicated introspection DTOs are
  deleted, not kept as duplicates. `BreakerConfig` gained `impl From<BreakerConfig> for
  edge_transport_breaker_policy::BreakerConfig` (mapping `half_open_after_seconds`→
  `cool_down_seconds`, `reset_after_successes`→`half_open_probe_count`) since `AdmitRequest`/
  `RecordOutcomeRequest` now take the shared config type. `BreakerError` keeps its local
  `CircuitOpen{host}` variant (the shared crate's error type has no equivalent — it's specific to
  this crate's own middleware rejection behavior).
- 3 orphaned dead files (`api/host/breaker/*`, `api/default/http_breaker.rs`, never wired into
  `api/mod.rs`'s module tree) deleted as unrelated cleanup surfaced during this review.

### Fixed
- Errors from the shared breaker transition implementation now propagate through reqwest middleware instead of panicking under an assumption that the transition can never fail.

### Added
- Initial project structure
