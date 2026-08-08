//! `edge_transport_http_egress_resilient_port` — trait contract for
//! composing the retry/rate/breaker/cache/cassette egress chain, zero
//! implementation. Per ADR-006: the concrete implementation lives in
//! `edge-transport-http-egress-resilient`, not here.

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;

pub use crate::api::{
    ApplyDefaultsRequest, ApplyFromConfigRequest, ResilientError, ResilientLayers,
};
// Re-exported so consumers can construct an `ApplyDefaultsRequest.cassette`
// value without taking their own direct dependency on the cassette crate —
// `transport` relies on this to stay free of any of the five concern
// crates directly (see ADR-006).
pub use edge_transport_http_egress_cassette::CassetteConfig;
