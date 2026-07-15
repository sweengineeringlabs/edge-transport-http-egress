//! Minimal usage: build the TLS layer (pass-through) and apply to a reqwest ClientBuilder.

fn main() {
    match edge_transport_http_egress_tls::HttpTlsSvc::build_tls_layer(
        edge_transport_http_egress_tls::TlsConfig::default(),
    ) {
        Ok(layer) => match layer.apply_to(reqwest::Client::builder()) {
            Ok(_builder) => {
                println!("edge_transport_http_egress_tls layer applied to ClientBuilder")
            }
            Err(e) => println!("edge_transport_http_egress_tls: apply_to failed: {e}"),
        },
        Err(e) => println!("edge_transport_http_egress_tls: {e}"),
    }
}
