//! `ApplicationConfigBuilder` — maps to `config/application.toml`.

/// Config builder corresponding to `config/application.toml`'s `[application]`
/// section — the only section this file declares.
///
/// Wraps `swe_edge_configbuilder::ConfigBuilderImpl` so this crate's public
/// API never names the foreign type directly (SEA `no_foreign_type`).
pub struct ApplicationConfigBuilder(pub(crate) swe_edge_configbuilder::ConfigBuilderImpl);
