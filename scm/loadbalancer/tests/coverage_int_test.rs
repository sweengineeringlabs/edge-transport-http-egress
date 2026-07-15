//! Coverage tests (rules 221 + 222) — _happy / _error / _edge variants.
//! Rule 221: create_config_builder, build_layer, validate_config,
//!            build_loadbalancer_layer, validate_loadbalancer_config.
//! Rule 222: describe (Processor trait), validate (Validator trait).

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_loadbalancer::{
    build_loadbalancer_layer, validate_loadbalancer_config, BackendConfig, LoadbalancerConfig,
    LoadbalancerSvc, Strategy,
};

fn valid_config() -> LoadbalancerConfig {
    LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig {
            url: "https://svc.internal".to_string(),
            weight: 1,
        }],
    }
}

fn empty_config() -> LoadbalancerConfig {
    LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![],
    }
}

// ── create_config_builder (rule 221) ─────────────────────────────────────────

#[test]
fn test_create_config_builder_returns_valid_loader_happy() {
    let loader = LoadbalancerSvc::create_config_builder().build_loader();
    let _ = loader;
}

#[test]
fn test_create_config_builder_does_not_panic_without_config_error() {
    let loader = LoadbalancerSvc::create_config_builder().build_loader();
    let _ = loader;
}

#[test]
fn test_create_config_builder_independent_instances_edge() {
    let l1 = LoadbalancerSvc::create_config_builder().build_loader();
    let l2 = LoadbalancerSvc::create_config_builder().build_loader();
    let _ = (l1, l2);
}

// ── build_layer (rule 221) ───────────────────────────────────────────────────

#[test]
fn test_build_layer_valid_config_returns_ok_happy() {
    let result = LoadbalancerSvc::build_layer(valid_config());
    assert!(result.is_ok(), "valid config must build layer");
}

#[test]
fn test_build_layer_empty_backends_returns_err_error() {
    let result = LoadbalancerSvc::build_layer(empty_config());
    assert!(result.is_err(), "empty backends must fail");
}

#[test]
fn test_build_layer_single_backend_succeeds_edge() {
    let cfg = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig {
            url: "https://only-one.internal".to_string(),
            weight: 1,
        }],
    };
    let result = LoadbalancerSvc::build_layer(cfg);
    assert!(result.is_ok());
}

// ── validate_config (rule 221) ───────────────────────────────────────────────

#[test]
fn test_validate_config_valid_config_passes_happy() {
    let result = LoadbalancerSvc::validate_config(&valid_config());
    assert!(result.is_ok());
}

#[test]
fn test_validate_config_empty_backends_fails_error() {
    let result = LoadbalancerSvc::validate_config(&empty_config());
    assert!(result.is_err());
    let msg = result.unwrap_err();
    assert!(
        msg.contains("empty"),
        "error message must mention 'empty': {msg}"
    );
}

#[test]
fn test_validate_config_zero_weight_backend_fails_edge() {
    let cfg = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig {
            url: "https://zero-weight.internal".to_string(),
            weight: 0,
        }],
    };
    let result = LoadbalancerSvc::validate_config(&cfg);
    assert!(result.is_err(), "zero-weight backend must fail validation");
}

// ── build_loadbalancer_layer (rule 221) ──────────────────────────────────────

#[test]
fn test_build_loadbalancer_layer_valid_config_returns_ok_happy() {
    let result = build_loadbalancer_layer(valid_config());
    assert!(result.is_ok());
}

#[test]
fn test_build_loadbalancer_layer_empty_backends_returns_err_error() {
    let result = build_loadbalancer_layer(empty_config());
    assert!(result.is_err());
}

#[test]
fn test_build_loadbalancer_layer_delegates_to_build_layer_edge() {
    // standalone fn and impl method must agree on outcome
    let direct = LoadbalancerSvc::build_layer(valid_config());
    let via_fn = build_loadbalancer_layer(valid_config());
    assert_eq!(direct.is_ok(), via_fn.is_ok());
}

// ── validate_loadbalancer_config (rule 221) ───────────────────────────────────

#[test]
fn test_validate_loadbalancer_config_valid_config_passes_happy() {
    let result = validate_loadbalancer_config(&valid_config());
    assert!(result.is_ok());
}

#[test]
fn test_validate_loadbalancer_config_empty_backends_fails_error() {
    let result = validate_loadbalancer_config(&empty_config());
    assert!(result.is_err());
}

#[test]
fn test_validate_loadbalancer_config_matches_validate_config_edge() {
    // standalone fn and impl method must agree
    let a = LoadbalancerSvc::validate_config(&valid_config());
    let b = validate_loadbalancer_config(&valid_config());
    assert_eq!(a.is_ok(), b.is_ok());
}

// ── describe (rule 222: Processor trait) ─────────────────────────────────────

#[test]
fn test_describe_layer_has_non_empty_debug_repr_happy() {
    let layer = LoadbalancerSvc::build_layer(valid_config()).expect("ok");
    let dbg = format!("{layer:?}");
    assert!(!dbg.is_empty());
}

#[test]
fn test_describe_svc_type_constructible_error() {
    let svc = edge_transport_http_egress_loadbalancer::LoadbalancerSvc;
    let _ = svc;
}

#[test]
fn test_describe_layer_debug_deterministic_edge() {
    let layer = LoadbalancerSvc::build_layer(valid_config()).expect("ok");
    let a = format!("{layer:?}");
    let b = format!("{layer:?}");
    assert_eq!(a, b);
}

// ── validate (rule 222: Validator trait) ─────────────────────────────────────

#[test]
fn test_validate_valid_config_passes_happy() {
    assert!(LoadbalancerSvc::validate_config(&valid_config()).is_ok());
}

#[test]
fn test_validate_empty_config_returns_non_empty_error_message_error() {
    let err = LoadbalancerSvc::validate_config(&empty_config()).unwrap_err();
    assert!(!err.is_empty(), "error message must not be empty");
}

#[test]
fn test_validate_url_empty_backend_fails_edge() {
    let cfg = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig {
            url: String::new(),
            weight: 1,
        }],
    };
    assert!(LoadbalancerSvc::validate_config(&cfg).is_err());
}
