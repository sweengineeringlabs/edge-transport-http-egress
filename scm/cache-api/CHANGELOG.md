# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed (breaking)

- The config builder is seeded with the package name `edge-transport-http-egress-cache-std` instead of the pre-split `edge-transport-http-egress-cache`, and diagnostic messages carry the new crate names (`edge_transport_http_egress_cache_api` / `_std`). configbuilder uses the name as a directory segment when it searches for configuration, so a configuration directory named after `edge-transport-http-egress-cache` must be renamed. The pre-split name is not kept.

## [0.7.0] - 2026-11-06

### Changed (breaking)

- Issue #30: the `edge-transport-http-egress-cache` package is split into `edge-transport-http-egress-cache-api` (this contract) and `edge-transport-http-egress-cache-std` (the implementation). The Rust crate names are `edge_transport_http_egress_cache_api` and `edge_transport_http_egress_cache_std`; there is no `edge_transport_http_egress_cache` crate any more.
- The contract has no `api/` source directory: modules sit at the source root, and every type is re-exported from the crate root as before.
- `MiddlewareHttpCache`, `HttpCacheSvcProcessor` and the `ProcessorSvcFactory`, `ValidatorSvcFactory` and `HttpCacheSvcFactory` structs are no longer public. Use `edge_transport_http_egress_cache_std::StdCacheFactory`, which implements the new `CacheFactory` trait: `create_processor()`, `create_validator()`, `create_config_builder()` and `build_cache_layer(config)`. `build_cache_layer` returns `Arc<dyn CacheLayer>`; `CacheLayer` is a `reqwest_middleware::Middleware` that also implements `HttpCache`, so attach it with `ClientBuilder::with_arc(layer)` and read the fallback TTL from it directly.
- `CacheConfig` keeps its schema, `Default`, `from_config` and section registration; they now live in the contract package.
- The config builder is still seeded with the application name `edge-transport-http-egress-cache`, not the `-std` package name: configbuilder uses the name as a directory segment when it searches for configuration, so changing it would move where existing deployments' configuration is found.
- Migration: replace `HttpCacheSvcProcessor::build_cache_layer(cfg)` with `StdCacheFactory.build_cache_layer(cfg)` (import `CacheFactory`), `ProcessorSvcFactory::create()` with `StdCacheFactory.create_processor()`, `ValidatorSvcFactory::create()` with `StdCacheFactory.create_validator()`, and drop `HttpCacheSvcFactory::from_layer(layer)` (the layer is already an `HttpCache`).

### Added

- Tests for every `StdCacheFactory` construction path, and an example that parses a cache configuration.

## [0.6.0] - 2026-11-05

### Added
- Initial project structure
