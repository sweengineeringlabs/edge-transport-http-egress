//! Local newtype wrapping the caller's security context (egress).

/// The caller's security context, propagated to
/// [`HttpEgress::send_with_context`](crate::api::traits::HttpEgress::send_with_context).
///
/// Wraps `edge_application::SecurityContext` so the trait method's parameter
/// never names the foreign type directly (SEA `no_foreign_type`). Construct
/// via [`HttpSecurityContext::from`] (or `.into()`) from an existing
/// `edge_application::SecurityContext`.
pub struct HttpSecurityContext(pub(crate) edge_application::SecurityContext);
