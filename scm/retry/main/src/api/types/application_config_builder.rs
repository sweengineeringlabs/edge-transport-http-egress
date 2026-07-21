//! `ApplicationConfigBuilder` — maps to `config/application.toml`.

use swe_edge_configbuilder::ConfigBuilderImpl;

/// Config builder corresponding to `config/application.toml`.
///
/// Wraps the external `swe_edge_configbuilder::ConfigBuilderImpl` so api/
/// never references a foreign crate type directly — construction and the
/// delegating accessors live in `core/`.
pub struct ApplicationConfigBuilder(pub(crate) ConfigBuilderImpl);
