//! End-to-end coverage of the public loadbalancer surface — happy / error /
//! edge variants for each entry point.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_loadbalancer::{
    BackendConfig, ConfigValidationRequest, DescribeRequest, LoadbalancerConfig,
    LoadbalancerMiddlewareError, LoadbalancerSvcProcessor, ProcessorFactory, Strategy,
    ValidatorFactory,
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

// ── LoadbalancerSvcProcessor::create_config_builder ───────────────────────────────────

#[test]
fn test_create_config_builder_seeds_crate_name_happy() {
    let builder = LoadbalancerSvcProcessor::create_config_builder();
    assert_eq!(builder.name(), "edge-transport-http-egress-loadbalancer");
    assert!(!builder.version().is_empty());
}

#[test]
fn test_create_config_builder_loader_reports_missing_section_error() {
    // With no config dir set, loading a section must be a real error — not a
    // silently-succeeding stub.
    let loader = LoadbalancerSvcProcessor::create_config_builder()
        .build_loader()
        .expect("build_loader succeeds with no config dir");
    let result: Result<LoadbalancerConfig, _> = loader.load_section("loadbalancer");
    assert!(
        result.is_err(),
        "no config file present must be a real error"
    );
}

// ── LoadbalancerSvcProcessor::build_layer ─────────────────────────────────────────────

#[test]
fn test_build_layer_valid_config_returns_layer_happy() {
    let layer = LoadbalancerSvcProcessor::build_layer(valid_config()).expect("valid builds");
    assert!(format!("{layer:?}").contains("LoadbalancerLayerPoolMetrics"));
}

#[test]
fn test_build_layer_empty_backends_returns_invalid_config_error() {
    assert!(matches!(
        LoadbalancerSvcProcessor::build_layer(empty_config()),
        Err(LoadbalancerMiddlewareError::InvalidConfig(_))
    ));
}

#[test]
fn test_build_layer_zero_weight_backend_rejected_edge() {
    let cfg = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig {
            url: "https://zero.internal".to_string(),
            weight: 0,
        }],
    };
    assert!(matches!(
        LoadbalancerSvcProcessor::build_layer(cfg),
        Err(LoadbalancerMiddlewareError::InvalidConfig(_))
    ));
}

// ── LoadbalancerSvcProcessor::validate_config ─────────────────────────────────────────

#[test]
fn test_validate_config_valid_passes_invalid_fails_happy() {
    assert!(LoadbalancerSvcProcessor::validate_config(&valid_config()).is_ok());
    assert!(LoadbalancerSvcProcessor::validate_config(&empty_config()).is_err());
}

#[test]
fn test_validate_config_empty_url_returns_error_edge() {
    let cfg = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig {
            url: String::new(),
            weight: 1,
        }],
    };
    let err = LoadbalancerSvcProcessor::validate_config(&cfg).unwrap_err();
    assert!(
        matches!(err, LoadbalancerMiddlewareError::InvalidConfig(ref m) if m.contains("non-empty url")),
        "{err}"
    );
}

// ── Processor::describe (via SAF factory) ────────────────────────────────────

#[test]
fn test_processor_describe_returns_crate_name_happy() {
    let resp = ProcessorFactory::create()
        .describe(DescribeRequest)
        .expect("describe infallible");
    assert_eq!(resp.value, "edge-transport-http-egress-loadbalancer");
}

// ── Validator::validate (via SAF factory) ────────────────────────────────────

#[test]
fn test_validator_validate_valid_passes_invalid_fails_happy() {
    let validator = ValidatorFactory::create();
    assert!(validator
        .validate(ConfigValidationRequest {
            config: valid_config()
        })
        .is_ok());
    assert!(validator
        .validate(ConfigValidationRequest {
            config: empty_config()
        })
        .is_err());
}
