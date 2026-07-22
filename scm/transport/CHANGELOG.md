# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- `loadbalancer` opt-in Cargo feature (default-on), config-driven via a `[loadbalancer]` `application.toml` section — mirrors `retry`/`rate`/`breaker`/`cache`/`cassette`. Not part of the zero-config `default_http_egress`/`build_default_egress` stack, since `LoadbalancerConfig`'s default has an empty backend list and fails validation. ([#18](https://github.com/sweengineeringlabs/edge-transport-http-egress/issues/18))
