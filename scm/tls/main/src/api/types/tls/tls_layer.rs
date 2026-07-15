//! `TlsLayer` — the TLS identity layer consumers apply to a
//! `reqwest::ClientBuilder` before building.
//!
//! Rule 160: public types in api/types/. Constructor lives in
//! `core::tls_layer` (pub(crate)); `apply_to` is on the public type here.

use std::sync::Arc;

use crate::api::error::TlsConfigError;
use crate::api::traits::HttpTls;

/// TLS identity layer. Opaque handle — consumers get one from
/// `HttpTlsSvc::build_tls_layer(config)` and apply it to a
/// `reqwest::ClientBuilder` via `apply_to(..)`.
///
/// ```ignore
/// let tls = edge_transport_http_egress_tls::HttpTlsSvc::build_tls_layer(TlsConfig::None)?;
/// let client = tls.apply_to(reqwest::Client::builder())?.build()?;
/// ```
pub struct TlsLayer {
    pub(crate) provider: Arc<dyn HttpTls>,
}

impl std::fmt::Debug for TlsLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TlsLayer")
            .field("provider", &self.provider.describe())
            .finish()
    }
}

impl TlsLayer {
    /// Augment `builder` with this layer's client identity.
    /// Returns the builder unchanged when the underlying provider
    /// is the `None` (pass-through) variant.
    pub fn apply_to(
        &self,
        builder: reqwest::ClientBuilder,
    ) -> Result<reqwest::ClientBuilder, TlsConfigError> {
        match self.provider.identity()? {
            Some(identity) => Ok(builder.identity(identity)),
            None => Ok(builder),
        }
    }
}
