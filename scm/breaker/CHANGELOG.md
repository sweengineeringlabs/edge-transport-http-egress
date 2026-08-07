# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

### Added
- Initial project structure
