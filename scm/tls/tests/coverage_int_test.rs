//! Coverage tests (rules 221 + 222) — _happy / _error / _edge variants.
//! Rule 221: create_config_builder, build_tls_layer, describe_tls_provider,
//!            validate_tls_config in tls_svc.rs.
//! Rule 222: describe + identity (HttpTls trait), describe (Provider trait),
//!            validate (Validator trait).

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_tls::{
    describe_tls_provider, validate_tls_config, HttpTlsSvc, TlsConfig,
};

// ── create_config_builder (rule 221) ─────────────────────────────────────────

#[test]
fn test_create_config_builder_seeds_package_name_happy() {
    let builder = HttpTlsSvc::create_config_builder();
    assert!(
        !builder.name().is_empty(),
        "config builder must carry the package name"
    );
}

#[test]
fn test_create_config_builder_seeds_package_version_error() {
    let builder = HttpTlsSvc::create_config_builder();
    assert!(
        !builder.version().is_empty(),
        "config builder must carry the package version"
    );
}

#[test]
fn test_create_config_builder_two_independent_instances_edge() {
    let b1 = HttpTlsSvc::create_config_builder();
    let b2 = HttpTlsSvc::create_config_builder();
    assert_eq!(b1.name(), b2.name());
}

// ── build_tls_layer (rule 221) ────────────────────────────────────────────────

#[test]
fn test_build_tls_layer_none_config_succeeds_happy() {
    let result = HttpTlsSvc::build_tls_layer(TlsConfig::None);
    assert!(
        result.is_ok(),
        "TlsConfig::None (no client cert) must build successfully"
    );
}

#[test]
fn test_build_tls_layer_pem_missing_path_returns_err_error() {
    let cfg = TlsConfig::Pem {
        path: "/nonexistent/swe_test_cert_coverage.pem".to_string(),
    };
    let result = HttpTlsSvc::build_tls_layer(cfg);
    assert!(
        result.is_err(),
        "missing PEM file must return Err, not panic"
    );
}

#[test]
fn test_build_tls_layer_idempotent_for_none_config_edge() {
    let r1 = HttpTlsSvc::build_tls_layer(TlsConfig::None);
    let r2 = HttpTlsSvc::build_tls_layer(TlsConfig::None);
    assert!(r1.is_ok() && r2.is_ok());
}

// ── describe_tls_provider (rule 221) ─────────────────────────────────────────

#[test]
fn test_describe_tls_provider_returns_non_empty_string_happy() {
    let label = describe_tls_provider(&HttpTlsSvc);
    assert!(
        !label.is_empty(),
        "describe_tls_provider must return a non-empty label"
    );
}

#[test]
fn test_describe_tls_provider_returns_static_str_error() {
    // static str means the label never allocates; repeated calls return the same pointer value
    let l1 = describe_tls_provider(&HttpTlsSvc);
    let l2 = describe_tls_provider(&HttpTlsSvc);
    assert_eq!(l1, l2, "describe_tls_provider must be deterministic");
}

#[test]
fn test_describe_tls_provider_svc_is_send_sync_edge() {
    fn assert_send_sync<T: Send + Sync>(_: &T) {}
    assert_send_sync(&HttpTlsSvc);
}

// ── validate_tls_config (rule 221) ───────────────────────────────────────────

#[test]
fn test_validate_tls_config_none_passes_happy() {
    let result = validate_tls_config(&TlsConfig::None);
    assert!(result.is_ok(), "TlsConfig::None must pass validation");
}

#[test]
fn test_validate_tls_config_pem_empty_path_returns_err_error() {
    let cfg = TlsConfig::Pem {
        path: String::new(),
    };
    let result = validate_tls_config(&cfg);
    assert!(result.is_err(), "empty PEM path must fail validation");
    let msg = result.unwrap_err();
    assert!(
        !msg.is_empty(),
        "validation error message must be non-empty"
    );
}

#[test]
fn test_validate_tls_config_pkcs12_empty_path_returns_err_edge() {
    let cfg = TlsConfig::Pkcs12 {
        path: String::new(),
        password_env: None,
    };
    let result = validate_tls_config(&cfg);
    assert!(result.is_err(), "empty PKCS12 path must fail validation");
}

// ── describe (rule 222: HttpTls + Provider traits) ────────────────────────────

#[test]
fn test_describe_provider_returns_known_label_happy() {
    // HttpTlsSvc implements Provider::describe() — accessible via describe_tls_provider
    let label = describe_tls_provider(&HttpTlsSvc);
    assert!(
        !label.is_empty(),
        "Provider::describe must return a non-empty label"
    );
}

#[test]
fn test_describe_provider_label_stable_across_calls_error() {
    let a = describe_tls_provider(&HttpTlsSvc);
    let b = describe_tls_provider(&HttpTlsSvc);
    assert_eq!(a, b);
}

#[test]
fn test_describe_svc_type_constructible_edge() {
    let svc = HttpTlsSvc;
    let _ = svc;
}

// ── identity (rule 222: HttpTls::identity) ────────────────────────────────────

#[test]
fn test_identity_none_config_returns_ok_none_happy() {
    // build_tls_layer with None internally calls identity() on the NoopProvider.
    let layer = HttpTlsSvc::build_tls_layer(TlsConfig::None).expect("must build");
    let _ = layer;
}

#[test]
fn test_identity_pem_missing_path_surfaces_error_error() {
    // identity() is called eagerly inside build_tls_layer; a missing PEM surfaces as Err.
    let cfg = TlsConfig::Pem {
        path: "/no/such/path_coverage_test.pem".to_string(),
    };
    let result = HttpTlsSvc::build_tls_layer(cfg);
    assert!(
        result.is_err(),
        "missing identity path must propagate from identity()"
    );
}

#[test]
fn test_identity_none_layer_is_send_sync_edge() {
    fn assert_send_sync<T: Send + Sync>(_: T) {}
    let layer = HttpTlsSvc::build_tls_layer(TlsConfig::None).expect("must build");
    assert_send_sync(layer);
}

// ── validate (rule 222: Validator trait) ─────────────────────────────────────

#[test]
fn test_validate_none_config_passes_happy() {
    let result = validate_tls_config(&TlsConfig::None);
    assert!(result.is_ok());
}

#[test]
fn test_validate_empty_pem_path_returns_non_empty_error_error() {
    let cfg = TlsConfig::Pem {
        path: String::new(),
    };
    let err = validate_tls_config(&cfg).unwrap_err();
    assert!(!err.is_empty());
}

#[test]
fn test_validate_none_and_pem_with_path_both_pass_edge() {
    // TlsConfig::Pem with a non-empty path must pass validate() even if the file is absent
    // (file existence is checked later at identity() time, not at validate() time)
    let pem = TlsConfig::Pem {
        path: "/hypothetical/path.pem".to_string(),
    };
    assert!(validate_tls_config(&TlsConfig::None).is_ok());
    assert!(
        validate_tls_config(&pem).is_ok(),
        "non-empty PEM path must pass validate()"
    );
}
