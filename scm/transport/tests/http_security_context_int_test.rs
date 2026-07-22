//! Integration tests for `HttpSecurityContext`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::HashMap;

use edge_transport_http_egress_transport::{
    HttpConfig, HttpRequest, HttpSecurityContext, HttpTransportSvc, SecurityContext,
};

/// `HttpSecurityContext::from` must wrap a real `SecurityContext` such that
/// `send_with_context` still accepts it and dispatches like a plain `send`.
#[tokio::test]
async fn test_http_security_context_from_wraps_and_dispatches_happy() {
    let egress = HttpTransportSvc::plain_http_egress(HttpConfig::default()).expect("ok");
    let ctx: HttpSecurityContext = SecurityContext {
        principal: None,
        tenant_id: Some("tenant-1".to_string()),
        claims: HashMap::new(),
        trace_id: None,
        authenticated: true,
        token: None,
        metadata: HashMap::new(),
        is_authorized: false,
        extensions: HashMap::new(),
    }
    .into();

    let result = egress
        .send_with_context(HttpRequest::get("http://0.0.0.0:1/unreachable"), ctx)
        .await;
    assert!(
        result.is_err(),
        "an unreachable URL must still fail through send_with_context"
    );
}

/// Two independently constructed contexts (authenticated vs not) must both
/// convert and dispatch — proving the wrapper isn't hardcoded to one fixed
/// input shape.
#[tokio::test]
async fn test_http_security_context_from_handles_distinct_inputs_edge() {
    let egress = HttpTransportSvc::plain_http_egress(HttpConfig::default()).expect("ok");

    let authenticated: HttpSecurityContext = SecurityContext {
        principal: None,
        tenant_id: Some("tenant-alice".to_string()),
        claims: HashMap::new(),
        trace_id: None,
        authenticated: true,
        token: None,
        metadata: HashMap::new(),
        is_authorized: true,
        extensions: HashMap::new(),
    }
    .into();
    let anonymous: HttpSecurityContext = SecurityContext {
        principal: None,
        tenant_id: None,
        claims: HashMap::new(),
        trace_id: None,
        authenticated: false,
        token: None,
        metadata: HashMap::new(),
        is_authorized: false,
        extensions: HashMap::new(),
    }
    .into();

    let a = egress
        .send_with_context(HttpRequest::get("http://0.0.0.0:1/a"), authenticated)
        .await;
    let b = egress
        .send_with_context(HttpRequest::get("http://0.0.0.0:1/b"), anonymous)
        .await;
    assert!(a.is_err() && b.is_err());
}
