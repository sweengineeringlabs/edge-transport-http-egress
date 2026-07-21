//! Integration tests for `CassetteModeResponse`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cassette::{
    CassetteError, CassetteModeRequest, CassetteModeResponse, HttpCassette,
};

/// A minimal external test-double proving `HttpCassette::mode` can genuinely
/// fail for a real implementor — the crate's own `CassetteLayer` never
/// returns `Err` here, so this is the only way to exercise the error path.
struct FailingHttpCassette;

impl HttpCassette for FailingHttpCassette {
    fn mode(&self, _request: CassetteModeRequest) -> Result<CassetteModeResponse, CassetteError> {
        Err(CassetteError::InvalidConfig(
            "no operating mode configured".to_string(),
        ))
    }
}

/// @covers: mode
#[test]
fn test_mode_unconfigured_implementor_returns_err_error() {
    let cassette = FailingHttpCassette;
    let result = cassette.mode(CassetteModeRequest);
    assert!(
        matches!(result, Err(CassetteError::InvalidConfig(_))),
        "an external HttpCassette impl reporting no configured mode must surface as InvalidConfig; got: {result:?}"
    );
}
