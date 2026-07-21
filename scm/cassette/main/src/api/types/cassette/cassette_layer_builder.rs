//! Fluent builder for [`CassetteLayer`].
//!
//! Rule 91: structs with 5+ fields require a builder.
//!
//! The impl block (fluent setters, `build_layer`) lives in
//! `core/cassette/cassette_layer_builder.rs` — api/ is a pure declaration layer.
//!
//! Known, accepted `no_orphan_types` gap: this type is a construction-time
//! ergonomic helper, not a trait-contract type, so it never appears in any
//! api/traits/ method signature. `pub_types_in_api_only` forces it to be
//! declared here regardless (no other layer is a legal declaration site for
//! a `pub` type), and it is genuinely load-bearing (re-exported from lib.rs,
//! exercised directly in `tests/cassette_layer_builder_int_test.rs`) —
//! deleting it is not an option. Mirrors the `saf_no_inherent_impl` gap
//! accepted for stateful SAF factories elsewhere in this codebase: a
//! genuine, understood tool-rule tension left unresolved rather than
//! hacked around.

use crate::api::CassetteConfig;

/// Fluent builder for [`CassetteLayer`](crate::api::CassetteLayer).
///
/// Required: `cassette_name`. Optional: `config` (defaults to `CassetteConfig::default()`).
pub struct CassetteLayerBuilder {
    pub(crate) config: Option<CassetteConfig>,
    pub(crate) cassette_name: Option<String>,
}
