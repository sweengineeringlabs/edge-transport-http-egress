//! Minimal usage: build the retry layer with the default config.

use edge_transport_http_egress_retry::Processor;

fn main() {
    match edge_transport_http_egress_retry::HttpRetrySvc.decorate(
        edge_transport_http_egress_retry::DecorateRequest {
            config: edge_transport_http_egress_retry::RetryConfig::default(),
        },
    ) {
        Ok(_) => println!("edge_transport_http_egress_retry layer built"),
        Err(e) => println!("edge_transport_http_egress_retry: {e}"),
    }
}
