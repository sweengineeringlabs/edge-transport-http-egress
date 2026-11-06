//! Minimal usage: build the rate layer with the default config.

use edge_transport_http_egress_rate_api::{RateConfig, RateFactory};
use edge_transport_http_egress_rate_std::StdRateFactory;

fn main() {
    match StdRateFactory.build_rate_layer(RateConfig::default()) {
        Ok(_) => println!("edge_transport_http_egress_rate_std layer built"),
        Err(e) => println!("edge_transport_http_egress_rate_std: {e}"),
    }
}
