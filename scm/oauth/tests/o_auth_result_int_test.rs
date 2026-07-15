//! Integration tests for `Result` type alias.
//!
//! Rule 120: `src/api/error/result.rs` requires a corresponding test file.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_oauth::{OAuthError, Result};

/// @covers: Result type alias
/// The `Result` alias must be usable as a return type and must propagate errors.
#[test]
fn oauth_struct_o_auth_result_ok_variant_int_test() {
    fn make_ok() -> Result<u32> {
        Ok(42u32)
    }
    let ok = make_ok();
    assert!(ok.is_ok(), "Result::Ok must be the Ok variant");
    assert_eq!(
        ok.expect("Ok variant must carry the value"),
        42u32,
        "Result::Ok must carry the value"
    );
}

/// @covers: Result type alias
/// The `Result` alias must propagate `OAuthError` in the `Err` variant.
#[test]
fn oauth_struct_o_auth_result_err_variant_int_test() {
    let err: Result<u32> = Err(OAuthError::Configuration("test".into()));
    assert!(err.is_err(), "Result::Err must be distinguishable from Ok");
}
