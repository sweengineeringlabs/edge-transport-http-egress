# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- `loadbalancer` opt-in Cargo feature (default-on), config-driven via a `[loadbalancer]` `application.toml` section — mirrors `retry`/`rate`/`breaker`/`cache`/`cassette`. Not part of the zero-config `default_http_egress`/`build_default_egress` stack, since `LoadbalancerConfig`'s default has an empty backend list and fails validation. ([#18](https://github.com/sweengineeringlabs/edge-transport-http-egress/issues/18))

### Fixed

- `mod auth;` in `core/mod.rs` was compiled unconditionally, so building with the `auth` feature excluded (`--no-default-features` in any combination without `auth`) never actually worked despite `auth` being declared as an opt-in Cargo feature. Gated `mod auth;` and its `SecurityAuthMiddleware` re-export behind `#[cfg(feature = "auth")]`. ([#19](https://github.com/sweengineeringlabs/edge-transport-http-egress/issues/19))
