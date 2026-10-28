//! Contract for composing the retry/rate/breaker/cache/cassette egress chain.

use reqwest_middleware::ClientBuilder;

use crate::api::error::ResilientError;
use crate::api::types::{ResilientConfigRequest, ResilientDefaultsRequest};

/// Decorates a [`ClientBuilder`] with the retry/rate/breaker/cache/
/// cassette chain, in that fixed relative order (`reqwest_middleware`
/// requires every layer registered before `.build()` is called, and this
/// specific order is required — see ADR-006 for why it can't be split).
///
/// Deliberately returns a [`ClientBuilder`], not a finished
/// `Box<dyn HttpEgress>` — preserves composability so a caller can still
/// combine the result with an auth/TLS layer at the correct relative
/// position (per `transport_svc.rs`'s existing behavior, auth wraps this
/// chain outermost) rather than being forced to accept one opaque,
/// auth-less client.
pub trait ResilientLayerComposer: Send + Sync {
    /// Apply the chain using config-section-driven activation.
    ///
    /// # Errors
    /// Returns [`ResilientError::Config`] if a section fails to load or
    /// validate, or the corresponding layer-specific variant if a present
    /// layer fails to build.
    fn apply_from_config(
        &self,
        request: ResilientConfigRequest<'_>,
    ) -> Result<ClientBuilder, ResilientError>;

    /// Apply the chain using SWE-shipped defaults for retry/rate/breaker/
    /// cache, unconditionally, plus the caller-supplied cassette config.
    ///
    /// # Errors
    /// Returns the corresponding layer-specific [`ResilientError`] variant
    /// if a layer fails to build.
    fn apply_defaults(
        &self,
        request: ResilientDefaultsRequest,
    ) -> Result<ClientBuilder, ResilientError>;
}
