//! Integration tests for `HttpEgressResult`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::{HttpEgressError, HttpEgressResult};

#[test]
fn test_http_egress_result_type_ok_wraps_value() {
    let result: HttpEgressResult<u32> = Ok(42);
    if let Ok(val) = result {
        assert_eq!(val, 42);
    } else {
        panic!("expected Ok");
    }
}

#[test]
fn test_http_egress_result_type_err_wraps_error() {
    let result: HttpEgressResult<u32> = Err(HttpEgressError::ConnectionFailed("refused".into()));
    if let Err(err) = result {
        assert!(err.to_string().contains("refused"));
    } else {
        panic!("expected Err");
    }
}
