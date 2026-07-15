//! Integration tests for `HttpConfigValidator`.

use edge_transport_http_egress_transport::HttpConfigValidatorAlias;

#[test]
fn test_http_config_validator_type_is_object_safe() {
    fn _check(_: &HttpConfigValidatorAlias) {}
}
