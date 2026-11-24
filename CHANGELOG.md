# Changelog

## Unreleased

- Breaking: replace the metrics-facade dependency in transport API/std with transport-owned typed observation callbacks. Counter/histogram names, values, method labels, send/stream timing and error behavior remain unchanged; composition roots bind their metrics implementation. All middleware policies remain available.
- Align all HTTP transport and resilient layer contracts on configbuilder 0.9 through one workspace dependency. Consumers can share their application loader, including explicit files and precise I/O errors, without resolving two configuration APIs.

## [0.7.0] - 2026-11-06

### Changed (breaking)

- Issue #30: every package is split into a contract package (`-api`) and an implementation package (`-std`): `breaker`, `rate`, `cache`, `retry`, `cassette` and `transport`; `resilient-port` is renamed `resilient-api` and `resilient` is renamed `resilient-std`. Public construction goes through a factory trait in each `-api` package and a single `Std*Factory` in each `-std` package; concrete implementation types are crate-private. The per-package changelogs under `scm/` carry the migration steps, and `scm/docs/package-layout-migration-map.md` records the mapping.
- Behaviour is unchanged by the split, including the config application names and the runtime error-message prefixes. The mapping of some response statuses to errors in `HttpEgress::send` (#46) is unchanged and still needs the owner's decision.
