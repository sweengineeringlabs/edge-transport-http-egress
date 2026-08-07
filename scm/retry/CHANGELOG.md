# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
