//! Integration tests for `StdResilientLayerComposer`: the preflight summary and the composer
//! behind the `ResilientLayerComposer` contract.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use configbuilder::{
    BuilderFinalizer as _, ConfigBuilder as _, ConfigLoaderFactory, FeatureSummaryOps as _,
};
use edge_transport_http_egress_cassette_api::CassetteConfig;
use edge_transport_http_egress_resilient_api::{
    ResilientConfigRequest, ResilientDefaultsRequest, ResilientError, ResilientLayerComposer,
};
use edge_transport_http_egress_resilient_std::StdResilientLayerComposer;
use edge_transport_http_egress_retry_api::RetryConfig;
use reqwest_middleware::ClientBuilder;

/// A complete `[retry]` section (every field the schema requires) with the given retry budget.
fn retry_section(max_retries: u32) -> String {
    format!("[retry]\nmax_retries = {max_retries}\ninitial_interval_ms = 100\nmax_interval_ms = 1000\nmultiplier = 2.0\nretryable_statuses = [503]\nretryable_methods = [\"GET\"]\n")
}

fn loader_for(toml: &str) -> (tempfile::TempDir, configbuilder::SectionLoaderImpl) {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("application.toml"), toml).expect("write config");
    let loader = ConfigLoaderFactory::create_config_builder()
        .with_name("edge-transport-http-egress-resilient-test")
        .with_version("0.0.0")
        .with_config_dir(dir.path())
        .build_loader()
        .expect("loader");
    (dir, loader)
}

/// @covers: preflight
#[test]
fn test_preflight_with_no_sections_reports_every_layer_off_happy() {
    let (_dir, loader) = loader_for("[application]\nname = \"x\"\n");
    let summary = StdResilientLayerComposer
        .preflight(&loader)
        .expect("no section is present");
    assert_eq!(
        summary.total_count(),
        5,
        "retry, rate, breaker, cache, cassette"
    );
    assert_eq!(
        summary.enabled_count(),
        0,
        "nothing is configured, nothing is on"
    );
}

/// @covers: preflight
#[test]
fn test_preflight_with_a_retry_section_reports_only_retry_on_edge() {
    let (_dir, loader) = loader_for(&retry_section(2));
    let summary = StdResilientLayerComposer
        .preflight(&loader)
        .expect("retry section parses");
    assert_eq!(summary.total_count(), 5);
    assert_eq!(
        summary.enabled_count(),
        1,
        "only the configured layer is on"
    );
    assert!(summary.to_string().contains("retry"), "got: {summary}");
}

/// @covers: preflight
#[test]
fn test_preflight_with_an_invalid_cassette_section_fails_error() {
    let (_dir, loader) = loader_for("[cassette]\nverbose = true\n");
    assert!(
        StdResilientLayerComposer.preflight(&loader).is_err(),
        "an unknown key in a present section must surface, not be silently dropped"
    );
}

/// @covers: apply_defaults
#[test]
fn test_apply_defaults_with_an_unreadable_cassette_returns_cassette_error_error() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("broken.yaml"), "interactions: [unclosed").expect("write");
    let cassette = CassetteConfig {
        cassette_dir: dir.path().to_string_lossy().replace('\\', "/"),
        ..CassetteConfig::default()
    };
    let result = StdResilientLayerComposer.apply_defaults(ResilientDefaultsRequest {
        builder: ClientBuilder::new(reqwest::Client::new()),
        cassette,
        cassette_name: "broken".to_string(),
    });
    assert!(
        matches!(result, Err(ResilientError::Cassette(_))),
        "a corrupt fixture must be reported as the cassette layer failing"
    );
}

/// @covers: apply_from_config
#[test]
fn test_apply_from_config_reads_the_retry_section_the_retry_api_parses_happy() {
    use configbuilder::{FeatureState, OptionalSection as _};

    let (_dir, loader) = loader_for(&retry_section(7));
    match RetryConfig::load_optional(&loader).expect("retry section parses") {
        FeatureState::Enabled(cfg) => assert_eq!(cfg.max_retries, 7),
        other => panic!("a present [retry] section must be enabled, got {other:?}"),
    }
    StdResilientLayerComposer
        .apply_from_config(ResilientConfigRequest {
            builder: ClientBuilder::new(reqwest::Client::new()),
            loader: &loader,
        })
        .expect("a retry-only config must compose");
}

/// @covers: apply_from_config
#[test]
fn test_apply_from_config_with_an_invalid_section_returns_config_error_error() {
    let (_dir, loader) = loader_for("[retry]\nmax_retries = \"many\"\n");
    let result = StdResilientLayerComposer.apply_from_config(ResilientConfigRequest {
        builder: ClientBuilder::new(reqwest::Client::new()),
        loader: &loader,
    });
    assert!(
        matches!(result, Err(ResilientError::Config(_))),
        "a malformed section must be reported as a config error"
    );
}
