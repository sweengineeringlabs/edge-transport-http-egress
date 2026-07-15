//! edge_transport_http_egress_tls — client-side mTLS identity for reqwest.
//!
//! Sibling crate to `edge_transport_http_egress_auth`. Different integration
//! surface: this crate augments a `reqwest::ClientBuilder` with
//! a client identity (PKCS12 or PEM) *before* the TLS handshake,
//! whereas `edge_transport_http_egress_auth` attaches HTTP headers *after* the
//! handshake. Both "auth" semantically; different layers
//! mechanically.
//!
//! ## Usage
//!
//! ```ignore
//! use edge_transport_http_egress_tls::{build_tls_layer, TlsConfig};
//! let tls = build_tls_layer(TlsConfig::default())?;
//! let client = tls.apply_to(reqwest::Client::builder())?.build()?;
//! ```

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use crate::api::{
    HttpTls, HttpTlsSvc, Provider, TlsConfig, TlsConfigError, TlsError, TlsLayer, Validator,
};
pub use saf::{describe_tls_provider, validate_tls_config};
