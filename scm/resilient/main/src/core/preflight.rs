//! `DefaultResilientLayers::preflight` — dry-run visibility into which of
//! the five composed concerns would activate, without building any of
//! them. Moved here from `transport`'s `saf/transport_svc.rs::preflight`,
//! since it inspects each concern's own config type directly (the same
//! reason the composition logic itself moved) — `transport` no longer
//! depends on any of the five concern crates at all, so it cannot know
//! their config shapes to report on them.

use swe_edge_configbuilder::{ConfigError, FeatureRegistry, FeatureSummary, SectionLoaderImpl};

use crate::core::DefaultResilientLayers;

impl DefaultResilientLayers {
    /// Load every optional `[section]` this crate composes into a
    /// [`FeatureRegistry`] and return a [`FeatureSummary`] of what would
    /// activate — without building any middleware. Log this at startup so
    /// operators see exactly which of retry/rate/breaker/cache/cassette
    /// are on (and why); it is the visibility guardrail against silent
    /// config-driven activation.
    ///
    /// # Errors
    /// Returns [`ConfigError`] if a present section fails to parse or
    /// validate.
    pub fn preflight(loader: &SectionLoaderImpl) -> Result<FeatureSummary, ConfigError> {
        let mut registry = FeatureRegistry::new();
        registry.load::<edge_transport_http_egress_retry::RetryConfig>(loader)?;
        registry.load::<edge_transport_http_egress_rate::RateConfig>(loader)?;
        registry.load::<edge_transport_http_egress_breaker::BreakerConfig>(loader)?;
        registry.load::<edge_transport_http_egress_cache::CacheConfig>(loader)?;
        registry.load::<edge_transport_http_egress_cassette::CassetteConfig>(loader)?;
        Ok(registry.summary())
    }
}
