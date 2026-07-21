//! Integration tests for `CassetteConfigBuilder`.
//!
//! Rule 120: `src/api/types/cassette/cassette_config_builder.rs` requires a
//! corresponding test file.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cassette::{CassetteConfigBuilder, CassetteError};

/// @covers: new
/// A freshly-constructed builder must build a config carrying the SWE defaults
/// (replay mode), proving `new()` seeds real defaults rather than blank state.
#[test]
fn cassette_struct_cassette_config_builder_new_returns_default_int_test() {
    let cfg = CassetteConfigBuilder::new()
        .build_config()
        .expect("new builder must build a default config");
    assert_eq!(
        cfg.mode, "replay",
        "a builder from new() must default to replay mode"
    );
}

/// @covers: build_config
/// Builder with all defaults (no fields set) must succeed and use `"replay"` mode.
#[test]
fn cassette_struct_cassette_config_builder_build_config_defaults_succeeds_int_test() {
    let cfg = CassetteConfigBuilder::new()
        .build_config()
        .expect("default builder must produce a valid config");
    assert_eq!(
        cfg.mode, "replay",
        "default builder mode must be 'replay'; got: {}",
        cfg.mode
    );
}

/// @covers: with_mode
/// Every accepted mode must round-trip into the built config verbatim, and an
/// unknown mode must be rejected — proving the setter validates, not just stores.
#[test]
fn cassette_struct_cassette_config_builder_with_valid_mode_succeeds_int_test() {
    for mode in ["replay", "record", "auto", "disabled"] {
        let cfg = CassetteConfigBuilder::new()
            .with_mode(mode)
            .build_config()
            .unwrap_or_else(|e| panic!("mode '{mode}' must be accepted; got: {e:?}"));
        assert_eq!(
            cfg.mode, mode,
            "with_mode must round-trip the mode verbatim"
        );
    }
    // Negative counterpart in the same test: an unknown mode must be rejected.
    assert!(
        CassetteConfigBuilder::new()
            .with_mode("passthrough")
            .build_config()
            .is_err(),
        "an unknown mode must be rejected, proving with_mode validates its input"
    );
}

/// @covers: with_mode
/// Setting an unknown mode must return a `CassetteError::ParseFailed`.
#[test]
fn cassette_struct_cassette_config_builder_with_invalid_mode_fails_int_test() {
    let result = CassetteConfigBuilder::new()
        .with_mode("passthrough")
        .build_config();
    assert!(result.is_err(), "unknown mode must produce an error");
    let err = result.unwrap_err();
    assert!(
        matches!(err, CassetteError::ParseFailed(_)),
        "unknown mode must yield ParseFailed; got: {err:?}"
    );
}

/// @covers: with_cassette_dir
/// Setting a cassette directory must be reflected in the built config.
#[test]
fn cassette_struct_cassette_config_builder_with_cassette_dir_reflected_int_test() {
    let cfg = CassetteConfigBuilder::new()
        .with_cassette_dir("/tmp/cassettes")
        .build_config()
        .expect("builder must succeed");
    assert_eq!(
        cfg.cassette_dir, "/tmp/cassettes",
        "cassette_dir must reflect the value set on the builder"
    );
}

/// @covers: with_match_on
/// Setting match keys must be reflected in the built config.
#[test]
fn cassette_struct_cassette_config_builder_with_match_on_reflected_int_test() {
    let keys = vec!["method".to_string(), "url".to_string()];
    let cfg = CassetteConfigBuilder::new()
        .with_match_on(keys.clone())
        .build_config()
        .expect("builder must succeed");
    assert_eq!(
        cfg.match_on, keys,
        "match_on must reflect the value set on the builder"
    );
}

/// @covers: with_scrub_headers
/// Setting scrub headers must be reflected in the built config.
#[test]
fn cassette_struct_cassette_config_builder_with_scrub_headers_reflected_int_test() {
    let headers = vec!["x-api-key".to_string()];
    let cfg = CassetteConfigBuilder::new()
        .with_scrub_headers(headers.clone())
        .build_config()
        .expect("builder must succeed");
    assert_eq!(
        cfg.scrub_headers, headers,
        "scrub_headers must reflect the value set on the builder"
    );
}

/// @covers: with_scrub_body_paths
/// Setting scrub body paths must be reflected in the built config.
#[test]
fn cassette_struct_cassette_config_builder_with_scrub_body_paths_reflected_int_test() {
    let paths = vec!["metadata.trace_id".to_string()];
    let cfg = CassetteConfigBuilder::new()
        .with_scrub_body_paths(paths.clone())
        .build_config()
        .expect("builder must succeed");
    assert_eq!(
        cfg.scrub_body_paths, paths,
        "scrub_body_paths must reflect the value set on the builder"
    );
}
