//! `edge_transport_http_egress_resilient` — real implementation of the
//! retry/rate/breaker/cache/cassette egress chain. Per ADR-006: lifted
//! from `transport`'s `saf/transport_svc.rs`, unchanged internally.

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod core;

pub use crate::core::DefaultResilientLayers;
pub use edge_transport_http_egress_resilient_port::{
    ApplyDefaultsRequest, ApplyFromConfigRequest, CassetteConfig, ResilientError, ResilientLayers,
};
