//! End-to-end test for ADR-006 config-driven activation in `transport`.
//!
//! Proves the consumer experience: adding a `[retry]`/`[rate]`/`[breaker]`/
//! `[cache]`/`[cassette]` section to `application.toml`
//! wires that layer into the assembled egress; omitting it (or
//! `enabled = false`) leaves it off. `auth` and `tls` have no config-section
//! form (BYOSec reversed 2026-07-16/2026-07-17) — see the auth tests below
//! and `tls_int_test.rs` for their construct-and-pass-in equivalents.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_security_transport_egress_http_oauth::OAuthTokenSource;
use edge_transport_http_egress_transport::{HttpEgressBuildError, HttpTransportSvc};
use swe_edge_configbuilder::ConfigLoaderFactory;
use tempfile::TempDir;

fn loader(content: &str) -> (TempDir, swe_edge_configbuilder::SectionLoaderImpl) {
    let dir = TempDir::new().expect("create temp dir");
    std::fs::write(dir.path().join("application.toml"), content).expect("write application.toml");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    (dir, loader)
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
    // Prove the section was actually recognized (not silently ignored):
    // preflight must report exactly one enabled feature.
    let summary = HttpTransportSvc::preflight(&l).expect("preflight succeeds");
    assert_eq!(
        summary.enabled_count(),
        1,
        "a present [retry] section must be counted as enabled"
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
    // Prove retry was genuinely omitted (not that config is always ignored):
    // preflight must report zero enabled features.
    let summary = HttpTransportSvc::preflight(&l).expect("preflight succeeds");
    assert_eq!(
        summary.enabled_count(),
        0,
        "with no config sections, no feature must be enabled"
    );
}

/// @covers: http_egress_from_config — a semantically invalid `[retry]`
/// (multiplier = 0) parses fine as TOML but fails the retry crate's own
/// field validation at decorate-time, surfacing as `HttpEgressBuildError::Retry`
/// (not `Config`, which is reserved for load/parse-level failures — see the
/// `bogus` field tests above for that case).
#[test]
fn test_retry_invalid_section_returns_retry_error() {
    let toml = "[retry]\nmax_retries = 3\ninitial_interval_ms = 200\n\
        max_interval_ms = 10000\nmultiplier = 0.0\nretryable_statuses = [503]\n\
        retryable_methods = [\"GET\"]\n";
    let (_d, l) = loader(toml);
    let result = HttpTransportSvc::http_egress_from_config(&l);
    let is_retry_err = matches!(result, Err(HttpEgressBuildError::Retry(_)));
    assert!(
        is_retry_err,
        "invalid [retry] (multiplier=0) must surface a Retry validation error"
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
    // `enabled = false` must be honoured: the section is present but must not
    // count as an enabled feature.
    let summary = HttpTransportSvc::preflight(&l).expect("preflight succeeds");
    assert_eq!(
        summary.enabled_count(),
        0,
        "a [retry] section with enabled=false must not be counted as enabled"
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
    // All five config-driven sections are present and enabled — preflight
    // must count every one of them.
    let summary = HttpTransportSvc::preflight(&l).expect("preflight succeeds");
    assert_eq!(
        summary.enabled_count(),
        5,
        "all five config-driven features must be enabled: {summary}"
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

// ── auth (construct-and-pass-in strategy; BYOSec reversed 2026-07-16 — no
// [auth] TOML section, `edge-security`'s bearer crate has no config-section
// form) ──────────────────────────────────────────────────────────────────────

use edge_security_transport_egress_http::{
    AuthorizeRequest, AuthorizeResponse, HttpEgressAuthError, HttpEgressAuthStrategy,
};

/// A no-op [`HttpEgressAuthStrategy`] that leaves the request untouched.
struct NoopAuthStrategy;

impl HttpEgressAuthStrategy for NoopAuthStrategy {
    fn authorize(
        &self,
        request: AuthorizeRequest,
    ) -> Result<AuthorizeResponse, HttpEgressAuthError> {
        Ok(AuthorizeResponse {
            request: request.request,
        })
    }
}

/// @covers: http_egress_from_config_with_auth — a caller-supplied strategy
/// wires the auth layer; the config-driven middleware (`[retry]` here) is
/// wired alongside it.
#[test]
fn test_auth_with_config_driven_middleware_builds() {
    let (_d, l) = loader(RETRY_TOML);
    let strategy: std::sync::Arc<dyn HttpEgressAuthStrategy> =
        std::sync::Arc::new(NoopAuthStrategy);
    let result = HttpTransportSvc::http_egress_from_config_with_auth(&l, strategy);
    assert!(
        result.is_ok(),
        "auth strategy + [retry] must assemble into one egress"
    );
    // The config-driven part must be genuinely wired alongside auth: preflight
    // over the same loader reports the [retry] feature enabled.
    let summary = HttpTransportSvc::preflight(&l).expect("preflight succeeds");
    assert_eq!(
        summary.enabled_count(),
        1,
        "the [retry] section must be wired alongside the auth strategy"
    );
}

/// @covers: http_egress_from_config_with_auth — an auth-only egress (no
/// middleware sections present) builds.
#[test]
fn test_auth_only_builds() {
    let (_d, l) = loader("[unrelated]\nx = 1");
    let strategy: std::sync::Arc<dyn HttpEgressAuthStrategy> =
        std::sync::Arc::new(NoopAuthStrategy);
    let result = HttpTransportSvc::http_egress_from_config_with_auth(&l, strategy);
    assert!(result.is_ok(), "auth-only egress must build");
    // No config-driven sections present ⇒ preflight reports nothing enabled.
    let summary = HttpTransportSvc::preflight(&l).expect("preflight succeeds");
    assert_eq!(
        summary.enabled_count(),
        0,
        "an auth-only egress must have no config-driven features enabled"
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
        5,
        "all 5 config-driven egress features are reported (auth and tls have no config-section form)"
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
    assert_eq!(summary.total_count(), 5);
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

#[async_trait::async_trait]
impl OAuthTokenSource for StaticTokenSource {
    async fn get_access_token(
        &self,
        _request: edge_security_transport_egress_http_oauth::AccessTokenRequest,
    ) -> Result<
        edge_security_transport_egress_http_oauth::AccessTokenResponse,
        edge_security_transport_egress_http_oauth::OAuthError,
    > {
        Ok(
            edge_security_transport_egress_http_oauth::AccessTokenResponse {
                token: "test-token".to_owned(),
            },
        )
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
    // The config-driven part must be genuinely wired alongside OAuth.
    let summary = HttpTransportSvc::preflight(&l).expect("preflight succeeds");
    assert_eq!(
        summary.enabled_count(),
        1,
        "the [retry] section must be wired alongside the OAuth token source"
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
    // No config-driven sections present ⇒ preflight reports nothing enabled.
    let summary = HttpTransportSvc::preflight(&l).expect("preflight succeeds");
    assert_eq!(
        summary.enabled_count(),
        0,
        "an OAuth-only egress must have no config-driven features enabled"
    );
}
