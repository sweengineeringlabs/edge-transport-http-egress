//! Fluent builder for [`CassetteConfig`].
//!
//! Rule 91: structs with 5+ fields require a builder. `CassetteConfig` has
//! 5 fields (`mode`, `cassette_dir`, `match_on`, `scrub_headers`, `scrub_body_paths`).
//!
//! The impl block (fluent setters, `build_config`) lives in
//! `core/cassette/cassette_config_builder.rs` — api/ is a pure declaration layer.
//!
//! Known, accepted `no_orphan_types` gap: this type is a construction-time
//! ergonomic helper, not a trait-contract type, so it never appears in any
//! api/traits/ method signature. `pub_types_in_api_only` forces it to be
//! declared here regardless (no other layer is a legal declaration site for
//! a `pub` type), and it is genuinely load-bearing (re-exported from lib.rs,
//! exercised directly in `tests/cassette_config_builder_int_test.rs` and via
//! `validator_int_test.rs`) — deleting it is not an option. Mirrors the
//! `saf_no_inherent_impl` gap accepted for stateful SAF factories elsewhere
//! in this codebase: a genuine, understood tool-rule tension left
//! unresolved rather than hacked around.

/// Fluent builder for [`CassetteConfig`](crate::api::CassetteConfig).
///
/// All fields are optional — unset fields fall back to the `CassetteConfig::default()` values.
pub struct CassetteConfigBuilder {
    pub(crate) mode: Option<String>,
    pub(crate) cassette_dir: Option<String>,
    pub(crate) match_on: Option<Vec<String>>,
    pub(crate) scrub_headers: Option<Vec<String>>,
    pub(crate) scrub_body_paths: Option<Vec<String>>,
}
