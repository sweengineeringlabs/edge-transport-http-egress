//! Compose the default resilience chain onto a client builder.
//!
//! Run with `cargo run -p edge-transport-http-egress-resilient-std --example resilient_compose`.
#![allow(
    clippy::expect_used,
    reason = "example code, not production library code"
)]

use edge_transport_http_egress_resilient_api::{
    CassetteConfig, ResilientDefaultsRequest, ResilientLayerComposer,
};
use edge_transport_http_egress_resilient_std::StdResilientLayerComposer;
use reqwest_middleware::ClientBuilder;

fn main() {
    let builder = StdResilientLayerComposer
        .apply_defaults(ResilientDefaultsRequest {
            builder: ClientBuilder::new(reqwest::Client::new()),
            cassette: CassetteConfig::disabled(),
            cassette_name: "example".to_string(),
        })
        .expect("the default chain composes");
    // The composed builder yields a client; no request is sent.
    let _client = builder.build();
    println!("retry, rate, breaker, cache and cassette layers composed");
}
