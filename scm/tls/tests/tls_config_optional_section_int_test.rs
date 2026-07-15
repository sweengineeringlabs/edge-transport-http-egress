//! Integration tests for the backend-owned `[tls]` OptionalSection (ADR-006).
//!
//! Proves config-driven activation: presence of `[tls]` ⇒ `FeatureState::Enabled`,
//! absence ⇒ `Disabled`, with cross-field validation and `deny_unknown_fields`.
//!
//! Only `OptionalSection` is imported (not `ConfigSection`), so `section_name`,
//! `metadata`, and `load_optional` resolve unambiguously despite `TlsConfig`
//! implementing both traits during the migration.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_tls::TlsConfig;
use swe_edge_configbuilder::{ConfigError, ConfigLoaderFactory, OptionalSection};
use tempfile::TempDir;

fn loader_with(content: &str) -> (TempDir, swe_edge_configbuilder::SectionLoaderImpl) {
    let dir = TempDir::new().expect("create temp dir");
    std::fs::write(dir.path().join("application.toml"), content).expect("write application.toml");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    (dir, loader)
}

/// @covers: section_name — canonical `[tls]` key is owned by this crate.
#[test]
fn test_section_name_is_tls() {
    assert_eq!(TlsConfig::section_name(), "tls");
}

/// @covers: metadata — feature is annotated for startup summaries.
#[test]
fn test_metadata_describes_feature() {
    let meta = TlsConfig::metadata();
    assert!(!meta.description.is_empty());
    assert_eq!(meta.owner, "platform-team");
}

/// @covers: load_optional — absent `[tls]` resolves to Disabled (feature off).
#[test]
fn test_absent_tls_section_returns_disabled() {
    let (_dir, loader) = loader_with("[other]\nx = 1");
    let state = TlsConfig::load_optional(&loader).expect("absent section is not an error");
    assert!(state.is_disabled(), "no [tls] section ⇒ TLS off");
}

/// @covers: load_optional — present `[tls]` with a pem identity ⇒ Enabled.
#[test]
fn test_pem_section_present_returns_enabled() {
    let (_dir, loader) = loader_with("[tls]\nkind = \"pem\"\npath = \"/etc/certs/client.pem\"");
    let state = TlsConfig::load_optional(&loader).expect("valid section loads");
    let cfg = state.into_option().expect("present ⇒ Enabled");
    assert!(matches!(cfg, TlsConfig::Pem { path } if path == "/etc/certs/client.pem"));
}

/// @covers: load_optional — present `[tls]` with a pkcs12 identity ⇒ Enabled.
#[test]
fn test_pkcs12_section_present_returns_enabled() {
    let (_dir, loader) = loader_with("[tls]\nkind = \"pkcs12\"\npath = \"/etc/certs/client.p12\"");
    let state = TlsConfig::load_optional(&loader).expect("valid section loads");
    assert!(matches!(
        state.into_option(),
        Some(TlsConfig::Pkcs12 { .. })
    ));
}

/// @covers: enabled = false — disables a present section (config-driven opt-out).
#[test]
fn test_enabled_false_disables_present_section() {
    let (_dir, loader) = loader_with("[tls]\nkind = \"pem\"\npath = \"/x.pem\"\nenabled = false");
    let state = TlsConfig::load_optional(&loader).expect("enabled=false is not an error");
    assert!(state.is_disabled());
}

/// @covers: deny_unknown_fields — `enabled = true` is rejected (ADR-006 gotcha).
#[test]
fn test_enabled_true_is_rejected_by_deny_unknown_fields() {
    let (_dir, loader) = loader_with("[tls]\nkind = \"pem\"\npath = \"/x.pem\"\nenabled = true");
    let err = TlsConfig::load_optional(&loader).expect_err("enabled=true must be rejected");
    assert!(matches!(err, ConfigError::Parse(_)), "got {err:?}");
}

/// @covers: deny_unknown_fields — an arbitrary unknown key is rejected.
#[test]
fn test_unknown_field_is_rejected() {
    let (_dir, loader) = loader_with("[tls]\nkind = \"pem\"\npath = \"/x.pem\"\nbogus = 1");
    let err = TlsConfig::load_optional(&loader).expect_err("unknown field must be rejected");
    assert!(matches!(err, ConfigError::Parse(_)), "got {err:?}");
}

/// @covers: kind deserialization — an unknown identity kind is rejected.
#[test]
fn test_unknown_kind_is_rejected() {
    let (_dir, loader) = loader_with("[tls]\nkind = \"jks\"\npath = \"/x\"");
    let err = TlsConfig::load_optional(&loader).expect_err("unknown kind must not parse");
    assert!(matches!(err, ConfigError::Parse(_)), "got {err:?}");
}

/// @covers: validate_enabled — an empty identity path is a validation error.
#[test]
fn test_empty_path_returns_validation_error() {
    let (_dir, loader) = loader_with("[tls]\nkind = \"pem\"\npath = \"\"");
    let err = TlsConfig::load_optional(&loader).expect_err("empty path must fail validation");
    assert!(matches!(err, ConfigError::Validation { .. }), "got {err:?}");
    let msg = err.to_string();
    assert!(msg.contains("path"), "error names the field: {msg}");
    assert!(msg.contains("tls"), "error names the section: {msg}");
}
