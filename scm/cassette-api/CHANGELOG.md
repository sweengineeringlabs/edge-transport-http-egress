# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.0] - 2026-11-06

### Changed (breaking)

- Issue #30: the `edge-transport-http-egress-cassette` package is split into `edge-transport-http-egress-cassette-api` (this contract) and `edge-transport-http-egress-cassette-std` (the implementation). The Rust crate names are `edge_transport_http_egress_cassette_api` and `edge_transport_http_egress_cassette_std`; there is no `edge_transport_http_egress_cassette` crate any more.
- The contract has no `api/` source directory: modules sit at the source root, and every type is re-exported from the crate root as before.
- `CassetteLayer` is now a trait (a `reqwest_middleware::Middleware` that is also an `HttpCassette` and `Debug`), not a concrete struct. Attach a layer with `ClientBuilder::with_arc`. `CassetteLayerBuilder`, `HttpCassetteSvc`, `HttpCassetteFactory`, `ProcessorFactory` and `ValidatorFactory` are no longer public. Use `edge_transport_http_egress_cassette_std::StdCassetteFactory`, which implements the new `CassetteFactory` trait: `create_processor()`, `create_validator()`, `create_config_builder()` and `build_cassette_layer(config, cassette_name)`.
- `CassetteConfig` and `CassetteConfigBuilder` keep their schema, defaults, `from_config`, `disabled`, `swe_default`, builder methods and section registration; they now live in the contract package.
- The config builder is still seeded with the application name `edge-transport-http-egress-cassette`, not the `-api` or `-std` package name: configbuilder uses the name as a directory segment when it searches for configuration, so changing it would move where existing deployments' configuration is found.
- Migration: replace `HttpCassetteSvc::build_cassette_layer(config, name)` with `StdCassetteFactory.build_cassette_layer(config, name)` (import `CassetteFactory`), `CassetteLayerBuilder::new().with_config(config).with_cassette_name(name).build_layer()` with the same call, `ProcessorFactory::create()` with `StdCassetteFactory.create_processor()`, `ValidatorFactory::create()` with `StdCassetteFactory.create_validator()`, `HttpCassetteSvc::create_config_builder()` with `StdCassetteFactory.create_config_builder()`, `HttpCassetteFactory::from_layer(layer)` with the layer itself (it is an `HttpCassette`), and `.with(layer)` with `.with_arc(layer)`.
- A builder with no cassette name used to fail at `build_layer()`; the factory takes the name as an argument, so that state can no longer be expressed.

## [0.6.0] - 2026-11-05

### Added
- Initial project structure
