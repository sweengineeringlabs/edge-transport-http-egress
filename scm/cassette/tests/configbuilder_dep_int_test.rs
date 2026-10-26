//! Dependency coverage test for `configbuilder`.
//! @covers: configbuilder
//!
//! Rule 95: `configbuilder` is used in `src/` and must have
//! integration coverage with an explicit `use configbuilder::...` import.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use configbuilder::{BuilderFinalizer as _, ConfigBuilder as _, ConfigBuilderImpl, Loader as _};
use edge_transport_http_egress_cassette::HttpCassetteSvc;

/// @covers: configbuilder
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
