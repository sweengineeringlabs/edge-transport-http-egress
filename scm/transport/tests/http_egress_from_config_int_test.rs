//! End-to-end test for ADR-006 config-driven activation in `transport`.
//!
//! Proves the consumer experience: adding a `[tls]` section to `application.toml`
//! wires client TLS into the assembled egress; omitting it (or `enabled = false`)
//! leaves TLS off. We exploit the fact that `build_tls_layer` resolves the cert
//! eagerly — so a present `[tls]` with a missing cert path fails at build time,
//! which is direct evidence that the TLS layer was actually attached.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_oauth::OAuthTokenSource;
use edge_transport_http_egress_transport::{HttpEgressBuildError, HttpTransportSvc};
use swe_edge_configbuilder::ConfigLoaderFactory;
use tempfile::TempDir;

fn loader(content: &str) -> (TempDir, swe_edge_configbuilder::SectionLoaderImpl) {
    let dir = TempDir::new().expect("create temp dir");
    std::fs::write(dir.path().join("application.toml"), content).expect("write application.toml");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    (dir, loader)
}

/// @covers: http_egress_from_config — `[tls]` present wires the TLS layer; eager
/// cert resolution fails on the missing file, proving the layer was attached.
#[test]
fn test_tls_section_present_wires_tls_layer() {
    let (_d, l) = loader("[tls]\nkind = \"pem\"\npath = \"/definitely/missing-xyz.pem\"");
    let result = HttpTransportSvc::http_egress_from_config(&l);
    assert!(
        matches!(result, Err(HttpEgressBuildError::Tls(_))),
        "[tls] present must wire TLS and fail eagerly on the missing cert"
    );
}

/// @covers: http_egress_from_config — no `[tls]` section ⇒ TLS off; the egress
/// builds successfully without touching any cert.
#[test]
fn test_no_tls_section_builds_without_tls() {
    let (_d, l) = loader("[unrelated]\nkey = \"value\"");
    let result = HttpTransportSvc::http_egress_from_config(&l);
    assert!(
        result.is_ok(),
        "absent [tls] must build successfully with TLS off"
    );
}

/// @covers: http_egress_from_config — `enabled = false` disables a present
/// `[tls]` section (no cert resolution, builds clean).
#[test]
fn test_tls_enabled_false_disables_tls() {
    let (_d, l) = loader("[tls]\nkind = \"pem\"\npath = \"/missing.pem\"\nenabled = false");
    let result = HttpTransportSvc::http_egress_from_config(&l);
    assert!(
        result.is_ok(),
        "enabled = false must disable TLS — no cert resolution attempted"
    );
}

/// @covers: http_egress_from_config — an invalid `[tls]` (empty path) surfaces a
/// Config validation error, not a silent pass.
#[test]
fn test_tls_invalid_section_returns_config_error() {
    let (_d, l) = loader("[tls]\nkind = \"pem\"\npath = \"\"");
    let result = HttpTransportSvc::http_egress_from_config(&l);
    assert!(
        matches!(result, Err(HttpEgressBuildError::Config(_))),
        "invalid [tls] must surface a Config error from validate_enabled"
    );
}

// ── [retry] section ────────────────────────────────────────────────────────

const RETRY_TOML: &str = "[retry]\nmax_retries = 3\ninitial_interval_ms = 200\n\
    max_interval_ms = 10000\nmultiplier = 2.0\nretryable_statuses = [503]\n\
    retryable_methods = [\"GET\"]\n";

/// @covers: http_egress_from_config — a valid `[retry]` section is loaded and the
/// retry layer is wired (the egress builds successfully).
#[test]
fn test_retry_section_present_builds() {
    let (_d, l) = loader(RETRY_TOML);
    let result = HttpTransportSvc::http_egress_from_config(&l);
    assert!(
        result.is_ok(),
        "valid [retry] must build with the retry layer wired"
    );
}

/// @covers: http_egress_from_config — no `[retry]` section ⇒ retry omitted; builds.
#[test]
fn test_no_retry_section_builds() {
    let (_d, l) = loader("[unrelated]\nkey = \"value\"");
    let result = HttpTransportSvc::http_egress_from_config(&l);
    assert!(
        result.is_ok(),
        "absent [retry] must build with retry omitted"
    );
}

/// @covers: http_egress_from_config — an invalid `[retry]` (multiplier = 0)
/// surfaces a Config validation error.
#[test]
fn test_retry_invalid_section_returns_config_error() {
    let toml = "[retry]\nmax_retries = 3\ninitial_interval_ms = 200\n\
        max_interval_ms = 10000\nmultiplier = 0.0\nretryable_statuses = [503]\n\
        retryable_methods = [\"GET\"]\n";
    let (_d, l) = loader(toml);
    let result = HttpTransportSvc::http_egress_from_config(&l);
    assert!(
        matches!(result, Err(HttpEgressBuildError::Config(_))),
        "invalid [retry] (multiplier=0) must surface a Config error"
    );
}

/// @covers: http_egress_from_config — `[retry]` with `enabled = false` is omitted.
#[test]
fn test_retry_enabled_false_omits_retry() {
    let toml = "[retry]\nenabled = false\nmax_retries = 3\ninitial_interval_ms = 200\n\
        max_interval_ms = 10000\nmultiplier = 2.0\nretryable_statuses = [503]\n\
        retryable_methods = [\"GET\"]\n";
    let (_d, l) = loader(toml);
    let result = HttpTransportSvc::http_egress_from_config(&l);
    assert!(
        result.is_ok(),
        "enabled=false [retry] must build with retry omitted"
    );
}

// ── rate / breaker / cache / cassette + full stack ──────────────────────────

const ALL_SECTIONS_TOML: &str = r#"
[retry]
max_retries = 3
initial_interval_ms = 200
max_interval_ms = 10000
multiplier = 2.0
retryable_statuses = [503]
retryable_methods = ["GET"]

[rate]
tokens_per_second = 100
burst_capacity = 200
per_host = true

[breaker]
failure_threshold = 5
half_open_after_seconds = 30
reset_after_successes = 2
failure_statuses = [500, 503]

[cache]
default_ttl_seconds = 60
max_entries = 1000
respect_cache_control = true
cache_private = false

[cassette]
mode = "disabled"
cassette_dir = "tests/cassettes"
match_on = ["method", "url"]
scrub_headers = ["authorization"]
scrub_body_paths = []
"#;

/// @covers: http_egress_from_config — every config-driven section present and
/// valid assembles into one egress (the full middleware stack wires together).
#[test]
fn test_all_sections_present_builds() {
    let (_d, l) = loader(ALL_SECTIONS_TOML);
    let result = HttpTransportSvc::http_egress_from_config(&l);
    assert!(
        result.is_ok(),
        "all valid sections must assemble into one egress"
    );
}

/// @covers: http_egress_from_config — `[rate]` is config-driven: a malformed
/// section surfaces a Config error (proving the section is loaded, not ignored).
#[test]
fn test_rate_invalid_section_returns_config_error() {
    let (_d, l) = loader("[rate]\nbogus = 1");
    assert!(
        matches!(
            HttpTransportSvc::http_egress_from_config(&l),
            Err(HttpEgressBuildError::Config(_))
        ),
        "[rate] must be config-driven and reject a malformed section"
    );
}

/// @covers: http_egress_from_config — `[breaker]` is config-driven.
#[test]
fn test_breaker_invalid_section_returns_config_error() {
    let (_d, l) = loader("[breaker]\nbogus = 1");
    assert!(
        matches!(
            HttpTransportSvc::http_egress_from_config(&l),
            Err(HttpEgressBuildError::Config(_))
        ),
        "[breaker] must be config-driven and reject a malformed section"
    );
}

/// @covers: http_egress_from_config — `[cache]` is config-driven.
#[test]
fn test_cache_invalid_section_returns_config_error() {
    let (_d, l) = loader("[cache]\nbogus = 1");
    assert!(
        matches!(
            HttpTransportSvc::http_egress_from_config(&l),
            Err(HttpEgressBuildError::Config(_))
        ),
        "[cache] must be config-driven and reject a malformed section"
    );
}

/// @covers: http_egress_from_config — `[cassette]` is config-driven.
#[test]
fn test_cassette_invalid_section_returns_config_error() {
    let (_d, l) = loader("[cassette]\nbogus = 1");
    assert!(
        matches!(
            HttpTransportSvc::http_egress_from_config(&l),
            Err(HttpEgressBuildError::Config(_))
        ),
        "[cassette] must be config-driven and reject a malformed section"
    );
}

// ── [auth] section (static auth; OAuth is programmatic, not config-driven) ───

/// @covers: http_egress_from_config — a valid `[auth]` section wires the static
/// auth layer (`kind = "none"` is a pass-through and builds clean).
#[test]
fn test_auth_section_present_builds() {
    let (_d, l) = loader("[auth]\nkind = \"none\"");
    let result = HttpTransportSvc::http_egress_from_config(&l);
    assert!(
        result.is_ok(),
        "valid [auth] must build with the auth layer wired"
    );
}

/// @covers: http_egress_from_config — `[auth]` is config-driven: a malformed
/// section surfaces a Config error.
#[test]
fn test_auth_invalid_section_returns_config_error() {
    let (_d, l) = loader("[auth]\nkind = \"bogus_scheme\"");
    assert!(
        matches!(
            HttpTransportSvc::http_egress_from_config(&l),
            Err(HttpEgressBuildError::Config(_))
        ),
        "[auth] must be config-driven and reject an unknown kind"
    );
}

/// @covers: http_egress_from_config — `[auth]` with `enabled = false` is omitted.
#[test]
fn test_auth_enabled_false_omits_auth() {
    let (_d, l) = loader("[auth]\nenabled = false\nkind = \"none\"");
    let result = HttpTransportSvc::http_egress_from_config(&l);
    assert!(
        result.is_ok(),
        "enabled=false [auth] must build with auth omitted"
    );
}

// ── preflight summary ───────────────────────────────────────────────────────

/// @covers: preflight — reports every egress feature with its on/off state; a
/// present `[cache]` shows enabled, the rest disabled.
#[test]
fn test_preflight_reports_enabled_and_disabled() {
    let (_d, l) = loader(
        "[cache]\ndefault_ttl_seconds = 60\nmax_entries = 1000\n\
         respect_cache_control = true\ncache_private = false",
    );
    let summary = HttpTransportSvc::preflight(&l).expect("preflight succeeds");
    assert_eq!(
        summary.total_count(),
        7,
        "all 7 egress features are reported"
    );
    assert_eq!(summary.enabled_count(), 1, "only [cache] is enabled");
    let text = summary.to_string();
    assert!(text.contains("cache"), "summary must name cache: {text}");
}

/// @covers: preflight — with no sections, every feature reports disabled.
#[test]
fn test_preflight_all_disabled_when_no_sections() {
    let (_d, l) = loader("[unrelated]\nx = 1");
    let summary = HttpTransportSvc::preflight(&l).expect("preflight succeeds");
    assert_eq!(summary.total_count(), 7);
    assert_eq!(summary.enabled_count(), 0, "no sections ⇒ nothing enabled");
}

/// @covers: preflight — a malformed present section surfaces a Config error.
#[test]
fn test_preflight_invalid_section_returns_config_error() {
    let (_d, l) = loader("[cache]\nbogus = 1");
    assert!(
        matches!(
            HttpTransportSvc::preflight(&l),
            Err(HttpEgressBuildError::Config(_))
        ),
        "preflight must surface a Config error for a malformed section"
    );
}

// ── OAuth (programmatic token source + config-driven middleware) ─────────────

/// A no-op [`OAuthTokenSource`] returning a static token without network I/O.
#[derive(Debug)]
struct StaticTokenSource;

impl OAuthTokenSource for StaticTokenSource {
    fn get_access_token(
        &self,
    ) -> futures::future::BoxFuture<'_, edge_transport_http_egress_oauth::Result<String>> {
        Box::pin(async { Ok("test-token".to_owned()) })
    }
}

/// @covers: http_egress_from_config_with_oauth — OAuth occupies the auth slot
/// while the config-driven middleware (`[retry]` here) is wired alongside it.
#[test]
fn test_oauth_with_config_driven_middleware_builds() {
    let (_d, l) = loader(RETRY_TOML);
    let source: std::sync::Arc<dyn OAuthTokenSource> = std::sync::Arc::new(StaticTokenSource);
    let result = HttpTransportSvc::http_egress_from_config_with_oauth(&l, source);
    assert!(
        result.is_ok(),
        "OAuth + [retry] must assemble into one egress"
    );
}

/// @covers: http_egress_from_config_with_oauth — an OAuth-only egress (no
/// middleware sections present) builds.
#[test]
fn test_oauth_only_builds() {
    let (_d, l) = loader("[unrelated]\nx = 1");
    let source: std::sync::Arc<dyn OAuthTokenSource> = std::sync::Arc::new(StaticTokenSource);
    let result = HttpTransportSvc::http_egress_from_config_with_oauth(&l, source);
    assert!(result.is_ok(), "OAuth-only egress must build");
}
