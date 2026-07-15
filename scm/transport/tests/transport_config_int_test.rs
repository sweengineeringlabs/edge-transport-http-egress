//! Integration tests for `TransportConfig`.

use edge_transport_http_egress_transport::{HttpConfig, TransportConfig};

/// @covers: TransportConfig
#[test]
fn test_transport_config_is_constructable() {
    let _config = TransportConfig {
        http: HttpConfig::default(),
    };
}
