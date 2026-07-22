#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for `LoadbalancerConfig` TOML parsing.

use edge_transport_http_egress_loadbalancer::{LoadbalancerConfig, Strategy};

/// @covers: LoadbalancerConfig::from_toml — round-robin strategy
#[test]
fn test_loadbalancer_config_parses_round_robin_strategy() {
    let toml = r#"
[loadbalancer]
strategy = "round-robin"
[[loadbalancer.backends]]
url = "https://api-1.internal"
weight = 1
"#;
    let cfg = LoadbalancerConfig::from_toml(toml).expect("must parse");
    assert_eq!(cfg.strategy, Strategy::RoundRobin);
    assert_eq!(cfg.backends.len(), 1);
    assert_eq!(cfg.backends[0].url, "https://api-1.internal");
    assert_eq!(cfg.backends[0].weight, 1);
}

/// @covers: LoadbalancerConfig::from_toml — weighted strategy
#[test]
fn test_loadbalancer_config_parses_weighted_strategy() {
    let toml = r#"
[loadbalancer]
strategy = "weighted"
[[loadbalancer.backends]]
url = "https://api-1.internal"
weight = 3
[[loadbalancer.backends]]
url = "https://api-2.internal"
weight = 1
"#;
    let cfg = LoadbalancerConfig::from_toml(toml).expect("must parse");
    assert_eq!(cfg.strategy, Strategy::Weighted);
    assert_eq!(cfg.backends.len(), 2);
    assert_eq!(cfg.backends[0].weight, 3);
}

/// @covers: LoadbalancerConfig::from_toml — least-connections strategy
#[test]
fn test_loadbalancer_config_parses_least_connections_strategy() {
    let toml = r#"
[loadbalancer]
strategy = "least-connections"
[[loadbalancer.backends]]
url = "https://api.test"
weight = 1
"#;
    let cfg = LoadbalancerConfig::from_toml(toml).expect("must parse");
    assert_eq!(cfg.strategy, Strategy::LeastConnections);
}

/// @covers: LoadbalancerConfig::from_toml — default strategy when omitted
#[test]
fn test_loadbalancer_config_defaults_to_round_robin_when_strategy_omitted() {
    let toml = r#"
[loadbalancer]
[[loadbalancer.backends]]
url = "https://api.test"
weight = 1
"#;
    let cfg = LoadbalancerConfig::from_toml(toml).expect("must parse");
    assert_eq!(cfg.strategy, Strategy::RoundRobin);
}

/// @covers: LoadbalancerConfig::from_toml — missing section returns error
#[test]
fn test_loadbalancer_config_fails_for_missing_section() {
    let toml = r#"[other]\nfoo = "bar""#;
    assert!(
        LoadbalancerConfig::from_toml(toml).is_err(),
        "missing section must fail"
    );
}
