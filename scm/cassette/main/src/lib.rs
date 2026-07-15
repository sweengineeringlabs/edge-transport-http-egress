//! `edge_transport_http_egress_cassette` — VCR-style HTTP record/replay middleware.
//!
//! Intercepts outbound HTTP requests made through a
//! `reqwest_middleware::ClientWithMiddleware` stack and either records the real
//! response to a YAML fixture file or replays a previously recorded one.
//! The result: integration tests that run offline, deterministically, and fast —
//! without hitting real upstreams in CI.
//!
//! # How it works
//!
//! On every request the middleware computes a **match key** from configurable
//! request components (method, URL, body hash). That key is looked up in the
//! cassette fixture file for the current test.
//!
//! | Mode | On hit | On miss |
//! |---|---|---|
//! | `"replay"` (CI default) | Return recorded response | **Fail loudly** |
//! | `"record"` | Hit upstream, overwrite recording | Hit upstream, save new recording |
//! | `"auto"` (local dev) | Return recorded response | Hit upstream, save new recording |
//! | `"disabled"` (production) | — | Pass through, no file I/O |
//!
//! # Credential scrubbing
//!
//! Before writing a response to disk, the middleware strips every header listed
//! in `scrub_headers` (e.g. `authorization`, `x-api-key`). Cassette files are
//! safe to commit to git — they never contain credentials.
//!
//! # Typical workflow
//!
//! ## 1. Record (once, on a developer machine)
//!
//! Add a `[cassette]` section to the test's config in `"auto"` or
//! `"record"` mode and run the tests with real credentials:
//!
//! ```toml
//! # tests/config/cassette.toml  (override for this test binary)
//! mode          = "auto"
//! cassette_dir  = "tests/cassettes"
//! match_on      = ["method", "url", "body_hash"]
//! scrub_headers = ["authorization", "x-api-key"]
//! scrub_body_paths = []
//! ```
//!
//! The first run hits the real upstream and writes YAML fixture files under
//! `tests/cassettes/<cassette_name>.yaml`. Commit those files.
//!
//! ## 2. Replay (CI and subsequent local runs)
//!
//! Switch to `"replay"` mode (the SWE default). Tests run entirely from the
//! fixture files — no network, no credentials required. Any request that
//! doesn't match a recorded interaction fails the test loudly, keeping the
//! cassette honest.
//!
//! ## 3. Re-record when the API changes
//!
//! Delete the affected cassette file and re-run in `"auto"` mode. The new
//! interaction is recorded and the file is recreated.
//!
//! # Quick start
//!
//! ```rust,no_run
//! use edge_transport_http_egress_cassette::{HttpCassetteSvc, CassetteConfig};
//!
//! # fn main() -> Result<(), edge_transport_http_egress_cassette::CassetteError> {
//! // SWE default: replay mode, tests/cassettes/, scrubs auth headers.
//! let cassette = HttpCassetteSvc::build_cassette_layer(CassetteConfig::default(), "my_test")?;
//!
//! let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
//!     .with(cassette)
//!     .build();
//! # Ok(())
//! # }
//! ```
//!
//! For production stacks where record/replay is not wanted:
//!
//! ```rust,no_run
//! use edge_transport_http_egress_cassette::{HttpCassetteSvc, CassetteConfig};
//!
//! # fn main() -> Result<(), edge_transport_http_egress_cassette::CassetteError> {
//! let cassette = HttpCassetteSvc::build_cassette_layer(CassetteConfig::disabled(), "unused")?;
//! # Ok(())
//! # }
//! ```
//!
//! # Non-deterministic request fields
//!
//! If the request body contains fields that change on every call (trace IDs,
//! timestamps, UUIDs) they will cause match-key misses on replay. Two options:
//!
//! - **`scrub_body_paths`** — zero out the offending JSON paths before hashing,
//!   so the key is stable even though the raw body changes.
//! - **Drop `"body_hash"` from `match_on`** — match on method + URL only.
//!   Simpler, but a single cassette entry is reused for all bodies sent to that
//!   endpoint.

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use crate::api::types::cassette::cassette_config_builder::CassetteConfigBuilder;
pub use crate::api::types::cassette::cassette_layer::CassetteLayer;
pub use crate::api::types::cassette::cassette_layer_builder::CassetteLayerBuilder;
pub use crate::api::{
    CassetteConfig, CassetteError, HttpCassette, HttpCassetteSvc, Processor, Validator,
};
