//! Minimal usage: build a cassette layer bound to a named fixture file.

use edge_transport_http_egress_cassette_api::{CassetteConfig, CassetteFactory};
use edge_transport_http_egress_cassette_std::StdCassetteFactory;

fn main() {
    match StdCassetteFactory.build_cassette_layer(CassetteConfig::default(), "example_cassette") {
        Ok(_) => println!(
            "edge_transport_http_egress_cassette_std layer built (fixture: example_cassette.yaml)"
        ),
        Err(e) => println!("edge_transport_http_egress_cassette_std: {e}"),
    }
}
