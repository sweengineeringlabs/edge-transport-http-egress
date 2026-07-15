//! Integration tests for `ApplicationConfigBuilder` in `edge-transport-http-egress-auth`.

use edge_transport_http_egress_auth::AuthSvc;

/// @covers: ApplicationConfigBuilder
/// `ApplicationConfigBuilder` (an alias of `ConfigBuilderImpl`) is produced by
/// the public `create_config_builder` factory, pre-seeded with this crate's
/// package name and version. Constructing one exercises that type through the
/// real public surface — this fails to compile if the factory is removed and
/// panics if construction breaks.
#[test]
fn test_create_config_builder_constructs_application_config_builder() {
    let _builder = AuthSvc::create_config_builder();
}
