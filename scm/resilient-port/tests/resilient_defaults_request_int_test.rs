//! Contract tests for the defaults request DTO.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cassette::CassetteConfig;
use edge_transport_http_egress_resilient_port::ResilientDefaultsRequest;
use reqwest_middleware::ClientBuilder;

/// @covers: ResilientDefaultsRequest
#[test]
fn test_resilient_defaults_request_retains_cassette_policy_and_name_edge() {
    let request = ResilientDefaultsRequest {
        builder: ClientBuilder::new(reqwest::Client::new()),
        cassette: CassetteConfig::disabled(),
        cassette_name: String::new(),
    };

    assert_eq!(request.cassette.mode, "disabled");
    assert!(request.cassette.match_on.is_empty());
    assert!(request.cassette_name.is_empty());
    let _client = request.builder.build();
}
