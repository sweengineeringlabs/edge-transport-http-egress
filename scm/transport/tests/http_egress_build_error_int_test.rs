//! Integration tests for `HttpEgressBuildError`.

use edge_transport_http_egress_transport::HttpEgressBuildError;

#[test]
fn test_http_egress_build_error_struct_display_formats_with_prefix() {
    let tls_err = edge_transport_http_egress_tls::TlsConfigError::Config("bad config".into());
    let build_err: HttpEgressBuildError = tls_err.into();
    let msg = build_err.to_string();
    assert!(
        msg.starts_with("tls:"),
        "error message must start with 'tls:', got: {msg:?}"
    );
}

#[test]
fn test_http_egress_build_error_struct_debug_is_non_empty() {
    let tls_err = edge_transport_http_egress_tls::TlsConfigError::Config("x".into());
    let build_err: HttpEgressBuildError = tls_err.into();
    let dbg = format!("{:?}", build_err);
    assert!(!dbg.is_empty());
}
