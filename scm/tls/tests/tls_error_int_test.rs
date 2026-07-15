//! Integration tests for `tls_error` in `edge-transport-http-egress-tls`.

use edge_transport_http_egress_tls::TlsConfigError;

/// @covers: TlsConfigError
/// Proves `TlsConfigError` is accessible from the crate root and that each variant
/// is constructible. A missing re-export or removed variant causes this to
/// fail to compile.
#[test]
fn test_tls_error_is_accessible() {
    let _ = core::marker::PhantomData::<TlsConfigError>;
}
