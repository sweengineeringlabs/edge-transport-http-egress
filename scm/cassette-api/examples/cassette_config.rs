//! Build a cassette configuration with the builder and parse one from TOML.
//!
//! Run with `cargo run -p edge-transport-http-egress-cassette-api --example cassette_config`.
#![allow(
    clippy::expect_used,
    reason = "example code, not production library code"
)]

use edge_transport_http_egress_cassette_api::{
    CassetteConfig, CassetteConfigBuilder, CassetteError,
};

fn main() {
    let built = CassetteConfigBuilder::new()
        .with_mode("record")
        .with_cassette_dir("tests/fixtures")
        .build_config()
        .expect("the example configuration is valid");
    println!("builder: mode {}, dir {}", built.mode, built.cassette_dir);
    assert_eq!(built.mode, "record");

    let toml = r#"
        mode = "replay"
        cassette_dir = "tests/cassettes"
        match_on = ["method", "url"]
        scrub_headers = ["authorization"]
        scrub_body_paths = []
    "#;
    let parsed = CassetteConfig::from_config(toml).expect("the example TOML is valid");
    println!("toml: matching on {:?}", parsed.match_on);
    assert_eq!(parsed.mode, "replay");

    // An unknown key is rejected rather than silently ignored.
    let err =
        CassetteConfig::from_config("verbose = true").expect_err("unknown field must be rejected");
    assert!(matches!(err, CassetteError::ParseFailed(_)));
    println!("unknown field rejected: {err}");
}
