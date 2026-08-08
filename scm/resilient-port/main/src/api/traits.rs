//! `ResilientLayers` trait — the contract for composing the retry/rate/
//! breaker/cache/cassette egress chain.

use reqwest_middleware::ClientBuilder;

use crate::api::errors::ResilientError;
use crate::api::types::{ApplyDefaultsRequest, ApplyFromConfigRequest};

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
pub trait ResilientLayers: Send + Sync {
    /// Apply the chain using config-section-driven activation.
    ///
    /// # Errors
    /// Returns [`ResilientError::Config`] if a section fails to load or
    /// validate, or the corresponding layer-specific variant if a present
    /// layer fails to build.
    fn apply_from_config(
        &self,
        request: ApplyFromConfigRequest<'_>,
    ) -> Result<ClientBuilder, ResilientError>;

    /// Apply the chain using SWE-shipped defaults for retry/rate/breaker/
    /// cache, unconditionally, plus the caller-supplied cassette config.
    ///
    /// # Errors
    /// Returns the corresponding layer-specific [`ResilientError`] variant
    /// if a layer fails to build.
    fn apply_defaults(
        &self,
        request: ApplyDefaultsRequest,
    ) -> Result<ClientBuilder, ResilientError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_transport_http_egress_cassette::CassetteConfig;
    use std::sync::Arc;

    // Deliberately not backed by any of the 5 concrete concern crates —
    // proves ResilientLayers is object-safe and satisfiable using *only*
    // this crate's own dependency graph (reqwest_middleware +
    // swe-edge-configbuilder + cassette's Config DTO), matching the
    // standalone-double pattern used for every other zero-implementation
    // port in this org (e.g. swe-edge-bootstrap-runtime's RuntimeManager).
    struct NoopResilientLayers;

    impl ResilientLayers for NoopResilientLayers {
        fn apply_from_config(
            &self,
            request: ApplyFromConfigRequest<'_>,
        ) -> Result<ClientBuilder, ResilientError> {
            let _ = request.loader;
            Ok(request.builder)
        }

        fn apply_defaults(
            &self,
            request: ApplyDefaultsRequest,
        ) -> Result<ClientBuilder, ResilientError> {
            if request.cassette_name.is_empty() {
                return Err(ResilientError::Cassette(
                    "cassette_name must not be empty".to_string(),
                ));
            }
            Ok(request.builder)
        }
    }

    fn bare_builder() -> ClientBuilder {
        ClientBuilder::new(reqwest::Client::new())
    }

    #[test]
    fn test_resilient_layers_double_is_object_safe_and_runs_standalone() {
        let layers: Arc<dyn ResilientLayers> = Arc::new(NoopResilientLayers);
        let loader = match swe_edge_configbuilder::ConfigLoaderFactory::create_loader() {
            Ok(l) => l,
            Err(e) => panic!("create_loader must succeed with no config dirs configured: {e}"),
        };
        let result = layers.apply_from_config(ApplyFromConfigRequest {
            builder: bare_builder(),
            loader: &loader,
        });
        assert!(result.is_ok());
    }

    #[test]
    fn test_resilient_layers_apply_defaults_rejects_empty_cassette_name_error() {
        let layers: Arc<dyn ResilientLayers> = Arc::new(NoopResilientLayers);
        let result = layers.apply_defaults(ApplyDefaultsRequest {
            builder: bare_builder(),
            cassette: CassetteConfig::disabled(),
            cassette_name: String::new(),
        });
        let err = match result {
            Err(e) => e,
            Ok(_) => panic!("an empty cassette_name must be rejected, not silently accepted"),
        };
        assert!(matches!(err, ResilientError::Cassette(_)));
    }
}
