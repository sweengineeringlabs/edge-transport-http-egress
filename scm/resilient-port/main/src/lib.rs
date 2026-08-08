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
