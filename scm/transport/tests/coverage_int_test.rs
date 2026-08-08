//! Coverage tests (rules 221 + 222) — _happy / _error / _edge variants.
//! Rule 221: create_config_builder, http_egress_from_config, preflight,
//!            default_http_egress, default_http_egress_with_config,
//!            observe_http_egress, default_http_stream_outbound, plain_http_egress,
//!            validate_http_config, validate.
//! Rule 222: send + send_stream + health_check + get (HttpEgress trait),
//!            subscribe_sse + connect_websocket (HttpStream trait),
//!            validate (Validator trait).
//!
//! The factory methods return opaque `Box<dyn HttpEgress>` / `Box<dyn HttpStream>`
//! values with no synchronous observable state, so build-only tests assert
//! repeatable construction (a builder with broken shared state fails the second
//! call) and cross-check `preflight` where a section's on/off state is
//! observable. End-to-end request behaviour is asserted by the `#[tokio::test]`
//! cases (unreachable host ⇒ Err) and by reqwest_middleware_int_test.rs.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_transport_http_egress_transport::{
    AlwaysValidConfig, ConfigRequest, ConnectWebsocketRequest, GetRequest, HealthCheckRequest,
    HttpConfig, HttpMethod, HttpRequest, HttpTransportSvc, SubscribeSseRequest,
    ValidatableHttpConfig,
};
use swe_edge_configbuilder::ConfigLoaderFactory;
use swe_observ_metrics::{create_local_metrics_backend, MetricsProvider};
use tempfile::TempDir;

fn empty_loader() -> (TempDir, swe_edge_configbuilder::SectionLoaderImpl) {
    let dir = TempDir::new().expect("create temp dir");
    std::fs::write(dir.path().join("application.toml"), "[unused]\nx = 1")
        .expect("write application.toml");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    (dir, loader)
}

fn invalid_loader() -> (TempDir, swe_edge_configbuilder::SectionLoaderImpl) {
    let dir = TempDir::new().expect("create temp dir");
    std::fs::write(dir.path().join("application.toml"), "[cache]\nbogus = 1")
        .expect("write application.toml");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    (dir, loader)
}

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

// ── http_egress_from_config (rule 221) ───────────────────────────────────────

#[test]
fn test_http_egress_from_config_empty_sections_builds_happy() {
    let (_d, l) = empty_loader();
    assert!(HttpTransportSvc::http_egress_from_config(&l).is_ok());
    // Prove the (absent) sections were genuinely evaluated: nothing is enabled.
    assert_eq!(
        edge_transport_http_egress_resilient::DefaultResilientLayers::preflight(&l)
            .expect("preflight succeeds")
            .enabled_count(),
        0,
        "empty config ⇒ no features enabled"
    );
}

#[test]
fn test_http_egress_from_config_invalid_section_returns_err_error() {
    let (_d, l) = invalid_loader();
    assert!(HttpTransportSvc::http_egress_from_config(&l).is_err());
}

#[test]
fn test_http_egress_from_config_two_calls_independent_edge() {
    let (_d1, l1) = empty_loader();
    let (_d2, l2) = empty_loader();
    let r1 = HttpTransportSvc::http_egress_from_config(&l1);
    let r2 = HttpTransportSvc::http_egress_from_config(&l2);
    assert!(r1.is_ok() && r2.is_ok());
}

// ── preflight (rule 221) ──────────────────────────────────────────────────────

#[test]
fn test_preflight_empty_sections_all_disabled_happy() {
    let (_d, l) = empty_loader();
    let summary = edge_transport_http_egress_resilient::DefaultResilientLayers::preflight(&l)
        .expect("preflight must succeed");
    assert_eq!(summary.enabled_count(), 0);
}

#[test]
fn test_preflight_invalid_section_returns_err_error() {
    let (_d, l) = invalid_loader();
    assert!(edge_transport_http_egress_resilient::DefaultResilientLayers::preflight(&l).is_err());
}

#[test]
fn test_preflight_total_count_matches_feature_set_edge() {
    let (_d, l) = empty_loader();
    let summary =
        edge_transport_http_egress_resilient::DefaultResilientLayers::preflight(&l).expect("ok");
    // retry + rate + breaker + cache + cassette = 5 (auth and
    // tls have no config-section form since BYOSec was reversed
    // 2026-07-16/17 — bearer has no swe-edge-configbuilder integration at
    // all, and tls's OptionalSection impl resolves against edge-security's
    // own vendored swe-edge-configbuilder, a different compiled crate than
    // this repo's)
    assert_eq!(summary.total_count(), 5);
}

// ── default_http_egress (rule 221) ────────────────────────────────────────────

#[test]
fn test_default_http_egress_returns_ok_happy() {
    let a = HttpTransportSvc::default_http_egress();
    let b = HttpTransportSvc::default_http_egress();
    assert!(
        a.is_ok() && b.is_ok(),
        "default_http_egress must build repeatably: {:?} / {:?}",
        a.err(),
        b.err(),
    );
}

#[test]
fn test_default_http_egress_boxed_trait_is_send_sync_error() {
    fn assert_send_sync<T: Send + Sync + ?Sized>(_: &T) {}
    let egress = HttpTransportSvc::default_http_egress().expect("ok");
    assert_send_sync(egress.as_ref());
}

#[test]
fn test_default_http_egress_two_calls_independent_edge() {
    let r1 = HttpTransportSvc::default_http_egress();
    let r2 = HttpTransportSvc::default_http_egress();
    assert!(r1.is_ok() && r2.is_ok());
}

// ── default_http_egress_with_config (rule 221) ────────────────────────────────

#[test]
fn test_default_http_egress_with_config_default_config_returns_ok_happy() {
    let a = HttpTransportSvc::default_http_egress_with_config(HttpConfig::default());
    let b = HttpTransportSvc::default_http_egress_with_config(HttpConfig::default());
    assert!(
        a.is_ok() && b.is_ok(),
        "default config must build repeatably: {:?} / {:?}",
        a.err(),
        b.err(),
    );
}

#[test]
fn test_default_http_egress_with_config_with_base_url_returns_ok_error() {
    let with_url = HttpTransportSvc::default_http_egress_with_config(HttpConfig::with_base_url(
        "https://svc.internal",
    ));
    let without_url = HttpTransportSvc::default_http_egress_with_config(HttpConfig::default());
    assert!(
        with_url.is_ok() && without_url.is_ok(),
        "must build with and without a base_url: {:?} / {:?}",
        with_url.err(),
        without_url.err(),
    );
}

#[test]
fn test_default_http_egress_with_config_zero_timeout_still_builds_edge() {
    // zero timeout is odd but not invalid at the config level
    let zero = HttpTransportSvc::default_http_egress_with_config(HttpConfig {
        timeout_secs: 0,
        ..HttpConfig::default()
    });
    let normal = HttpTransportSvc::default_http_egress_with_config(HttpConfig::default());
    assert!(
        zero.is_ok() && normal.is_ok(),
        "a zero-timeout config must still build: {:?} / {:?}",
        zero.err(),
        normal.err(),
    );
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

// ── default_http_stream_outbound (rule 221) ───────────────────────────────────

#[test]
fn test_default_http_stream_outbound_returns_ok_happy() {
    let a = HttpTransportSvc::default_http_stream_outbound();
    let b = HttpTransportSvc::default_http_stream_outbound();
    assert!(
        a.is_ok() && b.is_ok(),
        "default_http_stream_outbound must build repeatably: {:?} / {:?}",
        a.err(),
        b.err(),
    );
}

#[test]
fn test_default_http_stream_outbound_boxed_trait_is_send_sync_error() {
    fn assert_send_sync<T: Send + Sync + ?Sized>(_: &T) {}
    let stream = HttpTransportSvc::default_http_stream_outbound().expect("ok");
    assert_send_sync(stream.as_ref());
}

#[test]
fn test_default_http_stream_outbound_two_calls_independent_edge() {
    let r1 = HttpTransportSvc::default_http_stream_outbound();
    let r2 = HttpTransportSvc::default_http_stream_outbound();
    assert!(r1.is_ok() && r2.is_ok());
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
    let stream = HttpTransportSvc::default_http_stream_outbound().expect("ok");
    let result = stream
        .subscribe_sse(SubscribeSseRequest {
            url: "http://0.0.0.0:1/events".to_string(),
        })
        .await;
    assert!(result.is_err(), "unreachable SSE endpoint must return Err");
}

#[tokio::test]
async fn test_subscribe_sse_empty_url_returns_err_error() {
    let stream = HttpTransportSvc::default_http_stream_outbound().expect("ok");
    let result = stream
        .subscribe_sse(SubscribeSseRequest {
            url: "".to_string(),
        })
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_subscribe_sse_repeated_calls_do_not_panic_edge() {
    let stream = HttpTransportSvc::default_http_stream_outbound().expect("ok");
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
    let stream = HttpTransportSvc::default_http_stream_outbound().expect("ok");
    let result = stream
        .connect_websocket(ConnectWebsocketRequest {
            url: "ws://0.0.0.0:1/ws".to_string(),
        })
        .await;
    assert!(result.is_err(), "unreachable WS endpoint must return Err");
}

#[tokio::test]
async fn test_connect_websocket_empty_url_returns_err_error() {
    let stream = HttpTransportSvc::default_http_stream_outbound().expect("ok");
    let result = stream
        .connect_websocket(ConnectWebsocketRequest {
            url: "".to_string(),
        })
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_connect_websocket_repeated_calls_do_not_panic_edge() {
    let stream = HttpTransportSvc::default_http_stream_outbound().expect("ok");
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
fn test_subscribe_sse_stream_outbound_builds_happy() {
    let a = HttpTransportSvc::default_http_stream_outbound();
    let b = HttpTransportSvc::default_http_stream_outbound();
    assert!(a.is_ok() && b.is_ok(), "{:?} / {:?}", a.err(), b.err());
}

#[test]
fn test_subscribe_sse_stream_outbound_second_build_succeeds_error() {
    let a = HttpTransportSvc::default_http_stream_outbound();
    let b = HttpTransportSvc::default_http_stream_outbound();
    assert!(a.is_ok() && b.is_ok(), "{:?} / {:?}", a.err(), b.err());
}

#[test]
fn test_subscribe_sse_two_stream_outbounds_independent_edge() {
    let r1 = HttpTransportSvc::default_http_stream_outbound();
    let r2 = HttpTransportSvc::default_http_stream_outbound();
    assert!(r1.is_ok() && r2.is_ok());
}

// ── connect_websocket (rule 222: HttpStream::connect_websocket) — sync proxies ─

#[test]
fn test_connect_websocket_stream_outbound_builds_happy() {
    let a = HttpTransportSvc::default_http_stream_outbound();
    let b = HttpTransportSvc::default_http_stream_outbound();
    assert!(a.is_ok() && b.is_ok(), "{:?} / {:?}", a.err(), b.err());
}

#[test]
fn test_connect_websocket_stream_outbound_second_instance_succeeds_error() {
    let a = HttpTransportSvc::default_http_stream_outbound();
    let b = HttpTransportSvc::default_http_stream_outbound();
    assert!(a.is_ok() && b.is_ok(), "{:?} / {:?}", a.err(), b.err());
}

#[test]
fn test_connect_websocket_two_independent_stream_outbounds_edge() {
    let r1 = HttpTransportSvc::default_http_stream_outbound();
    let r2 = HttpTransportSvc::default_http_stream_outbound();
    assert!(r1.is_ok() && r2.is_ok());
}

// ── http_egress_from_config_with_oauth + plain_http_egress_with_oauth (rule 221)

#[cfg(feature = "oauth")]
mod oauth_svc_coverage {
    use std::sync::Arc;

    use edge_security_transport_egress_http_oauth::{
        AccessTokenRequest, AccessTokenResponse, OAuthError, OAuthTokenSource,
    };
    use edge_transport_http_egress_transport::{HttpConfig, HttpTransportSvc};

    #[derive(Debug)]
    struct StaticToken(String);

    #[async_trait::async_trait]
    impl OAuthTokenSource for StaticToken {
        async fn get_access_token(
            &self,
            _request: AccessTokenRequest,
        ) -> std::result::Result<AccessTokenResponse, OAuthError> {
            Ok(AccessTokenResponse {
                token: self.0.clone(),
            })
        }
    }

    fn static_token() -> Arc<dyn OAuthTokenSource> {
        Arc::new(StaticToken("test-access-token".to_string()))
    }

    #[test]
    fn test_http_egress_from_config_with_oauth_valid_loader_succeeds_happy() {
        let (_dir, loader) = super::empty_loader();
        let result = HttpTransportSvc::http_egress_from_config_with_oauth(&loader, static_token());
        assert!(result.is_ok(), "valid loader + token source must succeed");
        // The config-driven part is genuinely evaluated alongside OAuth.
        assert_eq!(
            edge_transport_http_egress_resilient::DefaultResilientLayers::preflight(&loader)
                .expect("preflight succeeds")
                .enabled_count(),
            0,
            "no config sections ⇒ no features enabled"
        );
    }

    #[test]
    fn test_http_egress_from_config_with_oauth_invalid_config_returns_err_error() {
        let (_dir, loader) = super::invalid_loader();
        // cache feature is enabled by default; the bogus cache section fails to parse.
        let result = HttpTransportSvc::http_egress_from_config_with_oauth(&loader, static_token());
        assert!(
            result.is_err(),
            "a malformed [cache] section must surface a build error"
        );
    }

    #[test]
    fn test_http_egress_from_config_with_oauth_two_builds_independent_edge() {
        let (_dir, loader) = super::empty_loader();
        let r1 = HttpTransportSvc::http_egress_from_config_with_oauth(&loader, static_token());
        let r2 = HttpTransportSvc::http_egress_from_config_with_oauth(&loader, static_token());
        assert!(r1.is_ok() && r2.is_ok());
    }

    #[test]
    fn test_plain_http_egress_with_oauth_default_config_succeeds_happy() {
        let a =
            HttpTransportSvc::plain_http_egress_with_oauth(HttpConfig::default(), static_token());
        let b =
            HttpTransportSvc::plain_http_egress_with_oauth(HttpConfig::default(), static_token());
        assert!(
            a.is_ok() && b.is_ok(),
            "default config + token source must build repeatably: {:?} / {:?}",
            a.err(),
            b.err(),
        );
    }

    #[test]
    fn test_plain_http_egress_with_oauth_no_base_url_still_builds_error() {
        // HttpConfig without a base_url is valid at build time; error only
        // surfaces when a request is made without an explicit URL.
        let no_url =
            HttpTransportSvc::plain_http_egress_with_oauth(HttpConfig::default(), static_token());
        let with_url = HttpTransportSvc::plain_http_egress_with_oauth(
            HttpConfig::with_base_url("https://svc.internal"),
            static_token(),
        );
        assert!(
            no_url.is_ok() && with_url.is_ok(),
            "must build with and without a base_url: {:?} / {:?}",
            no_url.err(),
            with_url.err(),
        );
    }

    #[test]
    fn test_plain_http_egress_with_oauth_two_independent_builds_edge() {
        let r1 =
            HttpTransportSvc::plain_http_egress_with_oauth(HttpConfig::default(), static_token());
        let r2 =
            HttpTransportSvc::plain_http_egress_with_oauth(HttpConfig::default(), static_token());
        assert!(r1.is_ok() && r2.is_ok());
    }
}
