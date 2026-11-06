# Changelog

## [0.7.0] - 2026-11-06

### Changed (breaking)

- Issue #30: every package is split into a contract package (`-api`) and an implementation package (`-std`): `breaker`, `rate`, `cache`, `retry`, `cassette` and `transport`; `resilient-port` is renamed `resilient-api` and `resilient` is renamed `resilient-std`. Public construction goes through a factory trait in each `-api` package and a single `Std*Factory` in each `-std` package; concrete implementation types are crate-private. The per-package changelogs under `scm/` carry the migration steps, and `scm/docs/package-layout-migration-map.md` records the mapping.
- Behaviour is unchanged by the split, including the config application names and the runtime error-message prefixes. The mapping of some response statuses to errors in `HttpEgress::send` (#46) is unchanged and still needs the owner's decision.
