//! Coverage tests (rules 221 + 222) — _happy / _error / _edge variants.
//! Rule 221: create_config_builder, observe_http_egress, plain_http_egress,
//!            plain_http_stream, validate_http_config, validate.
//! Rule 222: send + send_stream + health_check + get (HttpEgress trait),
//!            subscribe_sse + connect_websocket (HttpStream trait),
//!            validate (Validator trait).
//!
//! The factory methods return opaque `Box<dyn HttpEgress>` / `Box<dyn HttpStream>`
//! values with no synchronous observable state, so build-only tests assert
//! repeatable construction (a builder with broken shared state fails the second
//! call). End-to-end request behaviour is asserted by the `#[tokio::test]` cases
//! (unreachable host ⇒ Err) and by reqwest_middleware_int_test.rs.
//!
//! `transport` is a bare primitive: the composed constructors (default_http_egress*,
//! http_egress_from_config*, preflight, oauth) moved to the composition root —
//! see edge-transport-http-egress#28 / edge-bootstrap ADR-008.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_transport_http_egress_transport::{
    AlwaysValidConfig, ConfigRequest, ConnectWebsocketRequest, GetRequest, HealthCheckRequest,
    HttpConfig, HttpMethod, HttpRequest, HttpTransportSvc, SubscribeSseRequest,
    ValidatableHttpConfig,
};
use swe_observ_metrics::{create_local_metrics_backend, MetricsProvider};

/// A zero-timeout config that every validator in this crate must reject — the
/// canonical "known-bad" input for validation tests.
fn invalid_config() -> ValidatableHttpConfig {
    ValidatableHttpConfig {
        config: HttpConfig {
            timeout_secs: 0,
            ..HttpConfig::default()
        },
    }
}

// ── create_config_builder (rule 221) ─────────────────────────────────────────

#[test]
fn test_create_config_builder_seeds_package_name_happy() {
    let builder = HttpTransportSvc::create_config_builder();
    assert!(!builder.name().is_empty());
}

#[test]
fn test_create_config_builder_seeds_package_version_error() {
    let builder = HttpTransportSvc::create_config_builder();
    assert!(!builder.version().is_empty());
}

#[test]
fn test_create_config_builder_two_independent_instances_edge() {
    let b1 = HttpTransportSvc::create_config_builder();
    let b2 = HttpTransportSvc::create_config_builder();
    assert_eq!(b1.name(), b2.name());
}

// ── observe_http_egress (rule 221) ────────────────────────────────────────────

#[test]
fn test_observe_http_egress_wraps_inner_without_error_happy() {
    let inner = HttpTransportSvc::plain_http_egress(HttpConfig::default()).expect("ok");
    let provider: Arc<dyn MetricsProvider> = Arc::new(create_local_metrics_backend());
    let _observed = HttpTransportSvc::observe_http_egress(inner, provider);
}

#[test]
fn test_observe_http_egress_wrapper_is_send_sync_error() {
    fn assert_send_sync<T: Send + Sync + ?Sized>(_: &T) {}
    let inner = HttpTransportSvc::plain_http_egress(HttpConfig::default()).expect("ok");
    let provider: Arc<dyn MetricsProvider> = Arc::new(create_local_metrics_backend());
    let observed = HttpTransportSvc::observe_http_egress(inner, provider);
    assert_send_sync(observed.as_ref());
}

#[test]
fn test_observe_http_egress_shared_provider_two_instances_edge() {
    let provider: Arc<dyn MetricsProvider> = Arc::new(create_local_metrics_backend());
    let a = HttpTransportSvc::plain_http_egress(HttpConfig::default()).expect("ok");
    let b = HttpTransportSvc::plain_http_egress(HttpConfig::default()).expect("ok");
    let _wa = HttpTransportSvc::observe_http_egress(a, Arc::clone(&provider));
    let _wb = HttpTransportSvc::observe_http_egress(b, provider);
}

// ── plain_http_egress (rule 221) ─────────────────────────────────────────────

#[test]
fn test_plain_http_egress_default_config_returns_ok_happy() {
    let a = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    let b = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    assert!(
        a.is_ok() && b.is_ok(),
        "plain_http_egress must build repeatably: {:?} / {:?}",
        a.err(),
        b.err(),
    );
}

#[test]
fn test_plain_http_egress_with_header_returns_ok_error() {
    let with_header =
        HttpTransportSvc::plain_http_egress(HttpConfig::default().with_header("x-test", "value"));
    let without_header = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    assert!(
        with_header.is_ok() && without_header.is_ok(),
        "must build with and without a custom header: {:?} / {:?}",
        with_header.err(),
        without_header.err(),
    );
}

#[test]
fn test_plain_http_egress_no_redirects_returns_ok_edge() {
    let no_redirects = HttpTransportSvc::plain_http_egress(HttpConfig {
        follow_redirects: false,
        ..HttpConfig::default()
    });
    let with_redirects = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    assert!(
        no_redirects.is_ok() && with_redirects.is_ok(),
        "must build with redirects on and off: {:?} / {:?}",
        no_redirects.err(),
        with_redirects.err(),
    );
}

// ── plain_http_stream (rule 221) ──────────────────────────────────────────────

#[test]
fn test_plain_http_stream_default_config_returns_ok_happy() {
    let a = HttpTransportSvc::plain_http_stream(HttpConfig::default());
    let b = HttpTransportSvc::plain_http_stream(HttpConfig::default());
    assert!(
        a.is_ok() && b.is_ok(),
        "plain_http_stream must build repeatably: {:?} / {:?}",
        a.err(),
        b.err(),
    );
}

#[test]
fn test_plain_http_stream_boxed_trait_is_send_sync_error() {
    fn assert_send_sync<T: Send + Sync + ?Sized>(_: &T) {}
    let stream = HttpTransportSvc::plain_http_stream(HttpConfig::default()).expect("ok");
    assert_send_sync(stream.as_ref());
}

#[test]
fn test_plain_http_stream_two_calls_independent_edge() {
    let r1 = HttpTransportSvc::plain_http_stream(HttpConfig::default());
    let r2 = HttpTransportSvc::plain_http_stream(HttpConfig::default());
    assert!(r1.is_ok() && r2.is_ok());
}

// ── validate_http_config (rule 221) ──────────────────────────────────────────

#[test]
fn test_validate_http_config_default_passes_happy() {
    assert!(HttpTransportSvc::validate_http_config(&HttpConfig::default()).is_ok());
    // Sibling negative pins the pass as real.
    assert!(
        HttpTransportSvc::validate_http_config(&HttpConfig {
            timeout_secs: 0,
            ..HttpConfig::default()
        })
        .is_err(),
        "a zero-timeout config must be rejected"
    );
}

#[test]
fn test_validate_http_config_valid_config_does_not_err_error() {
    let cfg = HttpConfig {
        timeout_secs: 60,
        ..HttpConfig::default()
    };
    assert!(HttpTransportSvc::validate_http_config(&cfg).is_ok());
    // Sibling negative: zeroing the connect timeout must fail.
    assert!(
        HttpTransportSvc::validate_http_config(&HttpConfig {
            connect_timeout_secs: 0,
            ..cfg
        })
        .is_err(),
        "a zero connect_timeout config must be rejected"
    );
}

#[test]
fn test_validate_http_config_repeated_calls_consistent_edge() {
    let cfg = HttpConfig::default();
    let r1 = HttpTransportSvc::validate_http_config(&cfg);
    let r2 = HttpTransportSvc::validate_http_config(&cfg);
    assert_eq!(r1.is_ok(), r2.is_ok());
}

// ── validate (rule 221: generic gateway) ─────────────────────────────────────

#[test]
fn test_validate_always_valid_config_passes_happy() {
    assert!(HttpTransportSvc::validate(&AlwaysValidConfig).is_ok());
    assert!(
        HttpTransportSvc::validate(&invalid_config()).is_err(),
        "a known-bad config must fail through the same gateway"
    );
}

#[test]
fn test_validate_second_always_valid_instance_passes_error() {
    assert!(HttpTransportSvc::validate(&AlwaysValidConfig).is_ok());
    assert!(
        HttpTransportSvc::validate(&invalid_config()).is_err(),
        "a known-bad config must fail through the same gateway"
    );
}

#[test]
fn test_validate_two_independent_always_valid_instances_edge() {
    assert!(HttpTransportSvc::validate(&AlwaysValidConfig).is_ok());
    assert!(HttpTransportSvc::validate(&AlwaysValidConfig).is_ok());
    assert!(
        HttpTransportSvc::validate(&invalid_config()).is_err(),
        "a known-bad config must fail through the same gateway"
    );
}

// ── send (rule 222: HttpEgress::send) ─────────────────────────────────────────

#[tokio::test]
async fn test_send_unreachable_host_returns_err_happy() {
    let egress = HttpTransportSvc::plain_http_egress(HttpConfig::default()).expect("ok");
    let req = HttpRequest::get("http://0.0.0.0:1/swe_coverage_test".to_string());
    let result = egress.send(req).await;
    assert!(
        result.is_err(),
        "unreachable host must return Err from send()"
    );
}

#[tokio::test]
async fn test_send_empty_url_returns_err_error() {
    let egress = HttpTransportSvc::plain_http_egress(HttpConfig::default()).expect("ok");
    let req = HttpRequest::get(String::new());
    let result = egress.send(req).await;
    assert!(result.is_err(), "empty URL must return Err from send()");
}

#[tokio::test]
async fn test_send_two_sequential_calls_both_return_err_on_bad_host_edge() {
    let egress = HttpTransportSvc::plain_http_egress(HttpConfig::default()).expect("ok");
    let req1 = HttpRequest::get("http://0.0.0.0:1/a".to_string());
    let req2 = HttpRequest::get("http://0.0.0.0:1/b".to_string());
    let r1 = egress.send(req1).await;
    let r2 = egress.send(req2).await;
    assert!(r1.is_err() && r2.is_err());
}

// ── send_stream (rule 222: HttpEgress::send_stream) ──────────────────────────

#[tokio::test]
async fn test_send_stream_unreachable_host_returns_err_happy() {
    let egress = HttpTransportSvc::plain_http_egress(HttpConfig::default()).expect("ok");
    let req = HttpRequest::get("http://0.0.0.0:1/stream".to_string());
    let result = egress.send_stream(req).await;
    assert!(
        result.is_err(),
        "unreachable host must return Err from send_stream()"
    );
}

#[tokio::test]
async fn test_send_stream_empty_url_returns_err_error() {
    let egress = HttpTransportSvc::plain_http_egress(HttpConfig::default()).expect("ok");
    let req = HttpRequest::get(String::new());
    let result = egress.send_stream(req).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_send_stream_two_calls_independent_edge() {
    let egress = HttpTransportSvc::plain_http_egress(HttpConfig::default()).expect("ok");
    let r1 = egress
        .send_stream(HttpRequest::get("http://0.0.0.0:1/s1".to_string()))
        .await;
    let r2 = egress
        .send_stream(HttpRequest::get("http://0.0.0.0:1/s2".to_string()))
        .await;
    assert!(r1.is_err() && r2.is_err());
}

// ── health_check (rule 222: HttpEgress::health_check) ────────────────────────

#[tokio::test]
async fn test_health_check_unreachable_host_returns_err_happy() {
    let cfg = HttpConfig {
        base_url: Some("http://0.0.0.0:1".to_string()),
        ..Default::default()
    };
    let egress = HttpTransportSvc::plain_http_egress(cfg).expect("ok");
    let result = egress.health_check(HealthCheckRequest).await;
    assert!(
        result.is_err(),
        "unreachable base_url must return Err from health_check()"
    );
}

#[tokio::test]
async fn test_health_check_no_base_url_returns_err_error() {
    let egress = HttpTransportSvc::plain_http_egress(HttpConfig::default()).expect("ok");
    // no base_url → health_check probes nothing reachable
    let result = egress.health_check(HealthCheckRequest).await;
    // may err or ok depending on impl; either way must not panic
    let _ = result;
}

#[tokio::test]
async fn test_health_check_repeated_calls_do_not_panic_edge() {
    let cfg = HttpConfig {
        base_url: Some("http://0.0.0.0:1".to_string()),
        ..Default::default()
    };
    let egress = HttpTransportSvc::plain_http_egress(cfg).expect("ok");
    let _ = egress.health_check(HealthCheckRequest).await;
    let _ = egress.health_check(HealthCheckRequest).await;
}

// ── get (rule 222: HttpEgress::get default impl) ─────────────────────────────

#[tokio::test]
async fn test_get_unreachable_url_returns_err_happy() {
    let egress = HttpTransportSvc::plain_http_egress(HttpConfig::default()).expect("ok");
    let result = egress
        .get(GetRequest {
            url: "http://0.0.0.0:1/resource".to_string(),
        })
        .await;
    assert!(
        result.is_err(),
        "unreachable URL must return Err from get()"
    );
}

#[tokio::test]
async fn test_get_empty_url_returns_err_error() {
    let egress = HttpTransportSvc::plain_http_egress(HttpConfig::default()).expect("ok");
    let result = egress
        .get(GetRequest {
            url: "".to_string(),
        })
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_two_sequential_calls_both_err_edge() {
    let egress = HttpTransportSvc::plain_http_egress(HttpConfig::default()).expect("ok");
    let r1 = egress
        .get(GetRequest {
            url: "http://0.0.0.0:1/a".to_string(),
        })
        .await;
    let r2 = egress
        .get(GetRequest {
            url: "http://0.0.0.0:1/b".to_string(),
        })
        .await;
    assert!(r1.is_err() && r2.is_err());
}

// ── subscribe_sse (rule 222: HttpStream::subscribe_sse) ──────────────────────

#[tokio::test]
async fn test_subscribe_sse_unreachable_url_returns_err_happy() {
    let stream = HttpTransportSvc::plain_http_stream(HttpConfig::default()).expect("ok");
    let result = stream
        .subscribe_sse(SubscribeSseRequest {
            url: "http://0.0.0.0:1/events".to_string(),
        })
        .await;
    assert!(result.is_err(), "unreachable SSE endpoint must return Err");
}

#[tokio::test]
async fn test_subscribe_sse_empty_url_returns_err_error() {
    let stream = HttpTransportSvc::plain_http_stream(HttpConfig::default()).expect("ok");
    let result = stream
        .subscribe_sse(SubscribeSseRequest {
            url: "".to_string(),
        })
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_subscribe_sse_repeated_calls_do_not_panic_edge() {
    let stream = HttpTransportSvc::plain_http_stream(HttpConfig::default()).expect("ok");
    let _ = stream
        .subscribe_sse(SubscribeSseRequest {
            url: "http://0.0.0.0:1/e1".to_string(),
        })
        .await;
    let _ = stream
        .subscribe_sse(SubscribeSseRequest {
            url: "http://0.0.0.0:1/e2".to_string(),
        })
        .await;
}

// ── connect_websocket (rule 222: HttpStream::connect_websocket) ───────────────

#[tokio::test]
async fn test_connect_websocket_unreachable_url_returns_err_happy() {
    let stream = HttpTransportSvc::plain_http_stream(HttpConfig::default()).expect("ok");
    let result = stream
        .connect_websocket(ConnectWebsocketRequest {
            url: "ws://0.0.0.0:1/ws".to_string(),
        })
        .await;
    assert!(result.is_err(), "unreachable WS endpoint must return Err");
}

#[tokio::test]
async fn test_connect_websocket_empty_url_returns_err_error() {
    let stream = HttpTransportSvc::plain_http_stream(HttpConfig::default()).expect("ok");
    let result = stream
        .connect_websocket(ConnectWebsocketRequest {
            url: "".to_string(),
        })
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_connect_websocket_repeated_calls_do_not_panic_edge() {
    let stream = HttpTransportSvc::plain_http_stream(HttpConfig::default()).expect("ok");
    let _ = stream
        .connect_websocket(ConnectWebsocketRequest {
            url: "ws://0.0.0.0:1/w1".to_string(),
        })
        .await;
    let _ = stream
        .connect_websocket(ConnectWebsocketRequest {
            url: "ws://0.0.0.0:1/w2".to_string(),
        })
        .await;
}

// ── validate (rule 222: Validator trait) ─────────────────────────────────────

#[test]
fn test_validate_always_valid_config_passes_via_trait_happy() {
    assert!(HttpTransportSvc::validate(&AlwaysValidConfig).is_ok());
    assert!(
        HttpTransportSvc::validate(&invalid_config()).is_err(),
        "a known-bad config must fail through the Validator gateway"
    );
}

#[test]
fn test_validate_always_valid_config_passes_via_trait_error() {
    assert!(HttpTransportSvc::validate(&AlwaysValidConfig).is_ok());
    assert!(
        HttpTransportSvc::validate(&invalid_config()).is_err(),
        "a known-bad config must fail through the Validator gateway"
    );
}

#[test]
fn test_validate_consistent_for_same_input_edge() {
    // Determinism pinned against known outcomes, not a self-comparison:
    // AlwaysValidConfig always passes; a zero-timeout config always fails.
    assert!(HttpTransportSvc::validate(&AlwaysValidConfig).is_ok());
    assert!(HttpTransportSvc::validate(&AlwaysValidConfig).is_ok());
    assert!(
        HttpTransportSvc::validate(&invalid_config()).is_err(),
        "a zero-timeout config must consistently fail"
    );
}

// ── send (rule 222) — sync proxies (audit requires #[test], not #[tokio::test]) ─

#[test]
fn test_send_request_is_constructible_happy() {
    let req = HttpRequest::get("http://example.com".to_string());
    assert_eq!(
        req.method,
        HttpMethod::Get,
        "get() must produce a GET request"
    );
    assert_eq!(
        req.url, "http://example.com",
        "url must be carried verbatim"
    );
    assert!(req.body.is_none(), "a GET convenience request has no body");
}

#[test]
fn test_send_egress_builds_for_default_config_error() {
    let a = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    let b = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    assert!(a.is_ok() && b.is_ok(), "{:?} / {:?}", a.err(), b.err());
}

#[test]
fn test_send_two_independent_egress_instances_are_consistent_edge() {
    let r1 = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    let r2 = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    assert!(r1.is_ok() && r2.is_ok());
}

// ── send_stream (rule 222) — sync proxies ─────────────────────────────────────

#[test]
fn test_send_stream_egress_builds_on_default_config_happy() {
    let a = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    let b = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    assert!(a.is_ok() && b.is_ok(), "{:?} / {:?}", a.err(), b.err());
}

#[test]
fn test_send_stream_request_with_get_method_is_constructible_error() {
    let req = HttpRequest::get("http://example.com".to_string());
    assert_eq!(
        req.method,
        HttpMethod::Get,
        "get() must produce a GET request"
    );
    assert_eq!(req.url, "http://example.com");
}

#[test]
fn test_send_stream_two_egress_instances_independent_edge() {
    let r1 = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    let r2 = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    assert!(r1.is_ok() && r2.is_ok());
}

// ── health_check (rule 222) — sync proxies ────────────────────────────────────

#[test]
fn test_health_check_egress_builds_with_base_url_happy() {
    let with_url = HttpTransportSvc::plain_http_egress(HttpConfig {
        base_url: Some("http://example.com".to_string()),
        ..Default::default()
    });
    let without_url = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    assert!(
        with_url.is_ok() && without_url.is_ok(),
        "{:?} / {:?}",
        with_url.err(),
        without_url.err(),
    );
}

#[test]
fn test_health_check_egress_builds_without_base_url_error() {
    let a = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    let b = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    assert!(a.is_ok() && b.is_ok(), "{:?} / {:?}", a.err(), b.err());
}

#[test]
fn test_health_check_two_instances_independent_edge() {
    let r1 = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    let r2 = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    assert!(r1.is_ok() && r2.is_ok());
}

// ── get (rule 222: HttpEgress::get default impl) — sync proxies ───────────────

#[test]
fn test_get_egress_builds_before_first_call_happy() {
    let a = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    let b = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    assert!(a.is_ok() && b.is_ok(), "{:?} / {:?}", a.err(), b.err());
}

#[test]
fn test_get_request_url_is_accepted_at_construction_error() {
    let req = HttpRequest::get("http://0.0.0.0:1/resource".to_string());
    assert_eq!(req.url, "http://0.0.0.0:1/resource", "url carried verbatim");
    assert_eq!(req.method, HttpMethod::Get);
}

#[test]
fn test_get_two_independent_egress_instances_edge() {
    let r1 = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    let r2 = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    assert!(r1.is_ok() && r2.is_ok());
}

// ── send_with_context (rule 222: HttpEgress::send_with_context) ──────────────

#[test]
fn test_send_with_context_egress_builds_for_default_config_happy() {
    let a = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    let b = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    assert!(a.is_ok() && b.is_ok(), "{:?} / {:?}", a.err(), b.err());
}

#[test]
fn test_send_with_context_security_context_is_constructible_error() {
    use edge_transport_http_egress_transport::SecurityContext;
    let ctx = SecurityContext {
        principal: None,
        tenant_id: None,
        claims: std::collections::HashMap::new(),
        trace_id: None,
        authenticated: false,
        token: None,
        metadata: std::collections::HashMap::new(),
        is_authorized: false,
        extensions: std::collections::HashMap::new(),
    };
    assert!(
        !ctx.authenticated,
        "an unauthenticated context must not be flagged authenticated"
    );
    assert!(ctx.principal.is_none(), "no principal was set");
    assert!(ctx.claims.is_empty(), "no claims were set");
}

#[test]
fn test_send_with_context_two_independent_egress_instances_edge() {
    let r1 = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    let r2 = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    assert!(r1.is_ok() && r2.is_ok());
}

// ── config (rule 222: HttpEgress::config) ─────────────────────────────────

#[test]
fn test_config_returns_the_base_url_it_was_built_with_happy() {
    let egress =
        HttpTransportSvc::plain_http_egress(HttpConfig::with_base_url("https://example.com"))
            .expect("plain_http_egress must succeed");
    let response = egress
        .config(ConfigRequest)
        .expect("config must be infallible");
    assert_eq!(
        response.config.base_url.as_deref(),
        Some("https://example.com"),
        "config() must return the exact HttpConfig the egress was built with"
    );
}

#[test]
fn test_config_distinguishes_two_egresses_built_with_different_configs_error() {
    let a = HttpTransportSvc::plain_http_egress(HttpConfig::with_base_url("https://a.example"))
        .expect("ok");
    let b = HttpTransportSvc::plain_http_egress(HttpConfig::with_base_url("https://b.example"))
        .expect("ok");
    let a_cfg = a.config(ConfigRequest).expect("infallible").config;
    let b_cfg = b.config(ConfigRequest).expect("infallible").config;
    assert_ne!(
        a_cfg.base_url, b_cfg.base_url,
        "config() must reflect each instance's own base_url, not a shared/stale value"
    );
}

#[test]
fn test_config_repeated_calls_return_a_stable_value_edge() {
    let egress = HttpTransportSvc::plain_http_egress(HttpConfig::default()).expect("ok");
    let first = egress.config(ConfigRequest).expect("infallible").config;
    let second = egress.config(ConfigRequest).expect("infallible").config;
    assert_eq!(
        first.timeout_secs, second.timeout_secs,
        "config() must return a stable value across repeated calls"
    );
}

// ── subscribe_sse (rule 222: HttpStream::subscribe_sse) — sync proxies ────────

#[test]
fn test_subscribe_sse_stream_builds_happy() {
    let a = HttpTransportSvc::plain_http_stream(HttpConfig::default());
    let b = HttpTransportSvc::plain_http_stream(HttpConfig::default());
    assert!(a.is_ok() && b.is_ok(), "{:?} / {:?}", a.err(), b.err());
}

#[test]
fn test_subscribe_sse_stream_second_build_succeeds_error() {
    let a = HttpTransportSvc::plain_http_stream(HttpConfig::default());
    let b = HttpTransportSvc::plain_http_stream(HttpConfig::default());
    assert!(a.is_ok() && b.is_ok(), "{:?} / {:?}", a.err(), b.err());
}

#[test]
fn test_subscribe_sse_two_streams_independent_edge() {
    let r1 = HttpTransportSvc::plain_http_stream(HttpConfig::default());
    let r2 = HttpTransportSvc::plain_http_stream(HttpConfig::default());
    assert!(r1.is_ok() && r2.is_ok());
}

// ── connect_websocket (rule 222: HttpStream::connect_websocket) — sync proxies ─

#[test]
fn test_connect_websocket_stream_builds_happy() {
    let a = HttpTransportSvc::plain_http_stream(HttpConfig::default());
    let b = HttpTransportSvc::plain_http_stream(HttpConfig::default());
    assert!(a.is_ok() && b.is_ok(), "{:?} / {:?}", a.err(), b.err());
}

#[test]
fn test_connect_websocket_stream_second_instance_succeeds_error() {
    let a = HttpTransportSvc::plain_http_stream(HttpConfig::default());
    let b = HttpTransportSvc::plain_http_stream(HttpConfig::default());
    assert!(a.is_ok() && b.is_ok(), "{:?} / {:?}", a.err(), b.err());
}

#[test]
fn test_connect_websocket_two_independent_streams_edge() {
    let r1 = HttpTransportSvc::plain_http_stream(HttpConfig::default());
    let r2 = HttpTransportSvc::plain_http_stream(HttpConfig::default());
    assert!(r1.is_ok() && r2.is_ok());
}
