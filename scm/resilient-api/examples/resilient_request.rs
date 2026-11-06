//! Build the requests a composition root hands to a `ResilientLayerComposer`, and show how a
//! failure names the layer it came from.
//!
//! Run with `cargo run -p edge-transport-http-egress-resilient-api --example resilient_request`.

use edge_transport_http_egress_resilient_api::{
    CassetteConfig, ResilientDefaultsRequest, ResilientError,
};
use reqwest_middleware::ClientBuilder;

fn main() {
    let request = ResilientDefaultsRequest {
        builder: ClientBuilder::new(reqwest::Client::new()),
        cassette: CassetteConfig::disabled(),
        cassette_name: "example".to_string(),
    };
    println!(
        "defaults request: cassette mode {:?}, name {:?}",
        request.cassette.mode, request.cassette_name
    );
    assert_eq!(request.cassette.mode, "disabled");

    let error = ResilientError::Cassette("fixture file unreadable".to_string());
    println!("error: {error}");
    assert!(error.to_string().contains("cassette"));
}
