//! Contract tests for the config-driven request DTO.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use configbuilder::ConfigLoaderFactory;
use edge_transport_http_egress_resilient_port::ResilientConfigRequest;
use reqwest_middleware::ClientBuilder;

/// @covers: ResilientConfigRequest
#[test]
fn test_resilient_config_request_borrows_exact_loader_and_builds_edge() {
    let loader = ConfigLoaderFactory::create_loader().expect("create config loader");
    let request = ResilientConfigRequest {
        builder: ClientBuilder::new(reqwest::Client::new()),
        loader: &loader,
    };

    assert!(std::ptr::eq(request.loader, &loader));
    let _client = request.builder.build();
}
