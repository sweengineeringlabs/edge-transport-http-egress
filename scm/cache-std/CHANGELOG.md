# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- Issue #30: this package is the implementation half of the former `edge-transport-http-egress-cache` package. Its only public item is `StdCacheFactory`, the `edge_transport_http_egress_cache_api::CacheFactory` implementation. The moka-backed cache, its stale-while-revalidate refresh and the reqwest-middleware glue are crate-private. See the `-api` changelog for the migration.
- The `tracing` feature is kept and still gates the same spans.
