//! Dependency coverage test for `swe-edge-configbuilder`.
//! @covers: swe-edge-configbuilder
//!
//! Rule 95: `swe-edge-configbuilder` is used in `src/` and must have
//! integration coverage with an explicit `use swe_edge_configbuilder::...` import.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cassette::HttpCassetteSvc;
use swe_edge_configbuilder::ConfigBuilderImpl;

/// @covers: swe-edge-configbuilder
/// Confirms `create_config_builder` returns a `ConfigBuilderImpl` whose loader
/// can read the crate's shipped `[cassette]` section — end-to-end proof the
/// builder was seeded with a working name/version.
#[test]
fn cassette_type_configbuilder_dep_create_config_builder_int_test() {
    let builder: ConfigBuilderImpl = HttpCassetteSvc::create_config_builder();
    let cfg: edge_transport_http_egress_cassette::CassetteConfig = builder
        .with_config_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/config"))
        .build_loader()
        .expect("seeded builder must produce a loader")
        .load_section("cassette")
        .expect("shipped [cassette] section must load");
    assert_eq!(
        cfg.mode, "replay",
        "loader must read the shipped replay policy"
    );
}
