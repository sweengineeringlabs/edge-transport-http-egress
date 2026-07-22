# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Fixed

- `mod auth;` in `core/mod.rs` was compiled unconditionally, so building with the `auth` feature excluded (`--no-default-features` in any combination without `auth`) never actually worked despite `auth` being declared as an opt-in Cargo feature. Gated `mod auth;` and its `SecurityAuthMiddleware` re-export behind `#[cfg(feature = "auth")]`. ([#19](https://github.com/sweengineeringlabs/edge-transport-http-egress/issues/19))
