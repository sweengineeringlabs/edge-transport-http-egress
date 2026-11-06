//! `edge_transport_http_egress_resilient_api` — trait contract for
//! composing the retry/rate/breaker/cache/cassette egress chain, zero
//! implementation. Per ADR-006: the concrete implementation lives in
//! `edge-transport-http-egress-resilient-std`, not here.

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod error;
mod traits;
mod types;

pub use crate::error::ResilientError;
pub use crate::traits::ResilientLayerComposer;
pub use crate::types::{ResilientConfigRequest, ResilientDefaultsRequest};
// Re-exported so consumers can construct a `ResilientDefaultsRequest.cassette`
// value without taking their own direct dependency on the cassette crate —
// `transport` relies on this to stay free of any of the five concern
// crates directly (see ADR-006).
pub use edge_transport_http_egress_cassette_api::CassetteConfig;
