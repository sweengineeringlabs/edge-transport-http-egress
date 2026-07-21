/// R121 filesystem counterpart for `core::error_classifier::RetryErrorClassifier`.
///
/// Known, accepted `no_orphan_types` gap: `RetryErrorClassifier` is a pure
/// internal helper (static classification logic over
/// `reqwest_middleware::Error`, never returned from `saf/`, never a trait
/// object), so this placeholder never appears in a trait method signature.
/// Backing it with a new `pub trait` in api/ would misrepresent an
/// implementation detail as a public contract — mirrors the
/// `saf_no_inherent_impl` gap accepted for stateful SAF factories elsewhere
/// in this codebase.
///
/// Known, accepted `core_api_module_correspondence` gap: `core/error_classifier.rs`
/// is a loose file directly under `core/` (no domain subdirectory), so its
/// literal mirror path would be `api/error_classifier.rs` — but
/// `api_no_loose_rs_files` unconditionally forbids any loose `.rs` file at
/// the `api/` root (`mod.rs` excepted). Nesting this placeholder here
/// (`api/error/error_classifier.rs`) satisfies `api_no_loose_rs_files` and
/// `filename_matches_type`, but the correspondence check still wants the
/// exact flat path — a genuine conflict between two rules with no layout
/// that satisfies both, left unresolved and documented rather than hacked
/// around.
pub type ErrorClassifier = ();
