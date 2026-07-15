//! Minimal usage: build a cassette layer bound to a named fixture file.

fn main() {
    match edge_transport_http_egress_cassette::HttpCassetteSvc::build_cassette_layer(
        edge_transport_http_egress_cassette::CassetteConfig::default(),
        "example_cassette",
    ) {
        Ok(_) => println!(
            "edge_transport_http_egress_cassette layer built (fixture: example_cassette.yaml)"
        ),
        Err(e) => println!("edge_transport_http_egress_cassette: {e}"),
    }
}
