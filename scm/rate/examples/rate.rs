//! Minimal usage: build the rate layer with the default config.

fn main() {
    match edge_transport_http_egress_rate::HttpRateSvc::build_rate_layer(
        edge_transport_http_egress_rate::RateConfig::default(),
    ) {
        Ok(_) => println!("edge_transport_http_egress_rate layer built"),
        Err(e) => println!("edge_transport_http_egress_rate: {e}"),
    }
}
