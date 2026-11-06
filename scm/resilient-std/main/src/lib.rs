//! `edge_transport_http_egress_resilient_std` — real implementation of the
//! retry/rate/breaker/cache/cassette egress chain. Per ADR-006: lifted
//! from `transport`'s `saf/transport_svc.rs`, unchanged internally.

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod core;

pub use crate::core::StdResilientLayerComposer;
pub use edge_transport_http_egress_resilient_api::{
    CassetteConfig, ResilientConfigRequest, ResilientDefaultsRequest, ResilientError,
    ResilientLayerComposer,
};
