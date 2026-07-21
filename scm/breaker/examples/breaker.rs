//! Minimal usage: build the breaker layer with the default config.

fn main() {
    match edge_transport_http_egress_breaker::HttpBreakerSvcProcessor::build_breaker_layer(
        edge_transport_http_egress_breaker::BreakerConfig::default(),
    ) {
        Ok(_) => println!("edge_transport_http_egress_breaker layer built"),
        Err(e) => println!("edge_transport_http_egress_breaker: {e}"),
    }
}
