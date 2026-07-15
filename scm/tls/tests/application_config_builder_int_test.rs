//! Integration tests for `ApplicationConfigBuilder` in `edge-transport-http-egress-tls`.

use edge_transport_http_egress_tls::HttpTlsSvc;
use swe_edge_configbuilder::ConfigBuilderImpl;

/// @covers: ApplicationConfigBuilder
/// Proves `HttpTlsSvc::create_config_builder` returns a `ConfigBuilderImpl`
/// (the concrete type that `ApplicationConfigBuilder` aliases). A removed or
/// renamed type alias breaks this test to compile.
#[test]
fn test_application_config_builder_exists() {
    let _: ConfigBuilderImpl = HttpTlsSvc::create_config_builder();
}
