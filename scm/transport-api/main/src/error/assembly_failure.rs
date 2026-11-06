//! Local newtype wrapping a sibling crate's build-time error — keeps
//! [`HttpEgressBuildError`](super::HttpEgressBuildError)'s variant payloads
//! free of foreign crate types (SEA `no_foreign_type`) while preserving the
//! original error's message and source chain. Conversions from each sibling
//! crate's concrete error type, and the `Debug`/`Display`/`Error` impls,
//! live in `core/`.

/// Opaque wrapper around a middleware/config assembly failure.
pub struct AssemblyFailure(pub(crate) Box<dyn std::error::Error + Send + Sync>);
