//! Minimal usage: build the retry layer with the default config.

fn main() {
    match edge_transport_http_egress_retry::HttpRetrySvc::build_retry_layer(
        edge_transport_http_egress_retry::RetryConfig::default(),
    ) {
        Ok(_) => println!("edge_transport_http_egress_retry layer built"),
        Err(e) => println!("edge_transport_http_egress_retry: {e}"),
    }
}
