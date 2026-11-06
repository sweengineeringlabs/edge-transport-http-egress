//! Build a transport configuration and validate it.
//!
//! Run with `cargo run -p edge-transport-http-egress-transport-api --example transport_config`.
#![allow(
    clippy::expect_used,
    reason = "example code, not production library code"
)]

use edge_transport_http_egress_transport_api::{
    HttpConfig, HttpMethod, HttpRequest, ValidatableHttpConfig, ValidateRequest, Validator,
};

fn main() {
    let config = HttpConfig::with_base_url("https://api.example.test").with_timeout(10);
    println!(
        "config: base url {:?}, timeout {}s",
        config.base_url, config.timeout_secs
    );
    assert_eq!(config.timeout_secs, 10);

    ValidatableHttpConfig {
        config: config.clone(),
    }
    .validate(ValidateRequest)
    .expect("a positive timeout is valid");

    // A zero timeout is rejected by the validator rather than reaching a client.
    let err = ValidatableHttpConfig {
        config: config.with_timeout(0),
    }
    .validate(ValidateRequest)
    .expect_err("a zero timeout must be rejected");
    println!("zero timeout rejected: {err}");

    let request = HttpRequest::get("/health");
    assert_eq!(request.method, HttpMethod::Get);
}
