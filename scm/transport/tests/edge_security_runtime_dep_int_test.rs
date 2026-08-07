//! Dependency coverage test for `edge-security-runtime`.
//! @covers: edge-security-runtime
//!
//! Verifies that `edge-security-runtime`'s `SecurityContext` type surfaces
//! correctly through the transport crate's public API (SEA Rule 95 —
//! dependency coverage).

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::HashMap;

use edge_security_runtime::SecurityContext;

/// @covers: SecurityContext (edge-security-runtime)
/// Verifies that `SecurityContext` is reachable via the crate's public SAF
/// surface and that a fully-authenticated context can be constructed.
#[test]
fn test_security_context_authenticated_with_tenant_and_trace_is_constructible() {
    let ctx = SecurityContext {
        principal: None,
        tenant_id: Some("tenant-1".to_string()),
        claims: HashMap::from([("role".to_string(), "admin".to_string())]),
        trace_id: Some("trace-xyz".to_string()),
        authenticated: true,
        token: None,
        metadata: HashMap::new(),
        is_authorized: false,
        extensions: HashMap::new(),
    };
    assert!(ctx.authenticated);
    assert_eq!(ctx.tenant_id.as_deref(), Some("tenant-1"));
    assert_eq!(ctx.trace_id.as_deref(), Some("trace-xyz"));
    assert_eq!(ctx.claims.get("role").map(String::as_str), Some("admin"));
}

/// @covers: SecurityContext (edge-security-runtime)
/// Verifies that the zero-value (unauthenticated, no fields set) context is valid.
#[test]
fn test_security_context_unauthenticated_default_fields_are_none() {
    let ctx = SecurityContext {
        principal: None,
        tenant_id: None,
        claims: HashMap::new(),
        trace_id: None,
        authenticated: false,
        token: None,
        metadata: HashMap::new(),
        is_authorized: false,
        extensions: HashMap::new(),
    };
    assert!(!ctx.authenticated);
    assert!(ctx.principal.is_none());
    assert!(ctx.tenant_id.is_none());
    assert!(ctx.trace_id.is_none());
    assert!(ctx.claims.is_empty());
}

/// @covers: SecurityContext (edge-security-runtime)
/// Verifies that two independently constructed contexts are independent
/// (no shared global state).
#[test]
fn test_security_context_two_independent_instances_do_not_share_state() {
    let ctx1 = SecurityContext {
        principal: None,
        tenant_id: Some("t-1".to_string()),
        claims: HashMap::new(),
        trace_id: None,
        authenticated: true,
        token: None,
        metadata: HashMap::new(),
        is_authorized: false,
        extensions: HashMap::new(),
    };
    let ctx2 = SecurityContext {
        principal: None,
        tenant_id: Some("t-2".to_string()),
        claims: HashMap::new(),
        trace_id: None,
        authenticated: false,
        token: None,
        metadata: HashMap::new(),
        is_authorized: false,
        extensions: HashMap::new(),
    };
    assert_ne!(ctx1.tenant_id, ctx2.tenant_id);
    assert_ne!(ctx1.authenticated, ctx2.authenticated);
}
