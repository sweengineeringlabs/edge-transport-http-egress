//! Integration tests for `TransportConfig`.

use edge_transport_http_egress_transport::{HttpConfig, TransportConfig};

#[test]
fn test_transport_config_struct_stores_http_config() {
    let cfg = TransportConfig {
        http: HttpConfig::with_base_url("https://api.example.com"),
    };
    assert_eq!(
        cfg.http.base_url.as_deref(),
        Some("https://api.example.com")
    );
}
