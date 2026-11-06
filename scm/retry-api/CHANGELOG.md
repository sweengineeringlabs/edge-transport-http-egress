# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.0] - 2026-11-06

### Changed (breaking)

- Issue #30: the `edge-transport-http-egress-retry` package is split into `edge-transport-http-egress-retry-api` (this contract) and `edge-transport-http-egress-retry-std` (the implementation). The Rust crate names are `edge_transport_http_egress_retry_api` and `edge_transport_http_egress_retry_std`; there is no `edge_transport_http_egress_retry` crate any more.
- The contract has no `api/` source directory: modules sit at the source root, and every type is re-exported from the crate root as before.
- `RetryLayer` is now a trait (a `reqwest_middleware::Middleware` that is `Debug`), not a concrete struct, and `DecorateResponse::layer` is an `Arc<dyn RetryLayer>`: attach it with `ClientBuilder::with_arc(response.layer)`. `HttpRetrySvc` and the `ProcessorFactory` and `ValidatorFactory` structs are no longer public. Use `edge_transport_http_egress_retry_std::StdRetryFactory`, which implements the new `RetryFactory` trait: `create_processor()`, `create_validator()`, `new_config_builder()` and `new_app_config_builder()`. The retry layer is built by `Processor::decorate` on the processor it creates, as before.
- `RetryConfig`, `RetryConfigBuilder` and `ApplicationConfigBuilder` keep their schema, defaults, `from_config`, `validate`, builder methods and section registration; they now live in the contract package. `ApplicationConfigBuilder::new()` is now public (and `ApplicationConfigBuilder` implements `Default`), because the standard package has to construct it.
- The application config builder is still seeded with the application name `edge-transport-http-egress-retry`, not the `-api` or `-std` package name: configbuilder uses the name as a directory segment when it searches for configuration, so changing it would move where existing deployments' configuration is found.
- Migration: replace `HttpRetrySvc.decorate(request)` with `StdRetryFactory.create_processor().decorate(request)` (import `Processor` and `RetryFactory`), `ProcessorFactory::create()` with `StdRetryFactory.create_processor()`, `ValidatorFactory::create()` with `StdRetryFactory.create_validator()`, `HttpRetrySvc::new_config_builder()` with `StdRetryFactory.new_config_builder()` (or `RetryConfigBuilder::new()`), and `.with(layer)` with `.with_arc(layer)`.

### Added

- Tests for `ApplicationConfigBuilder` against configbuilder, and an example that builds and parses a retry configuration.

## [0.6.0] - 2026-11-05

### Changed (breaking)
- `RetryConfig` gained a new field `jitter_factor: f64` (`#[serde(default)]`, defaults to `0.0` so
  existing TOML configs keep producing byte-identical backoff durations to today's un-jittered
  formula). `RetryConfigBuilder` gained a matching `jitter_factor()` setter.
- `core::layer::RetryLayer::backoff_for` no longer computes backoff inline — it now delegates to
  `edge_transport_retry::BackoffScheduler::next_backoff`, the same scheduler
  `edge-transport-grpc-egress` uses, seeded via `edge_transport_retry_adapter::DefaultJitterRng`.
  `RetryConfig` implements the shared `edge_transport_retry_policy::BackoffPolicy` trait to supply
  it. Verified the new path is bit-identical to the old hand-rolled
  `initial_ms * multiplier.powi(attempt)` formula for every existing test case when
  `jitter_factor == 0.0`.

### Added
- Initial project structure
