//! Minimal usage: build the loadbalancer layer with a two-backend config.

fn main() {
    use edge_transport_http_egress_loadbalancer::{
        BackendConfig, LoadbalancerConfig, LoadbalancerSvcProcessor, Strategy,
    };

    let config = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![
            BackendConfig {
                url: "https://api-1.internal".to_string(),
                weight: 1,
            },
            BackendConfig {
                url: "https://api-2.internal".to_string(),
                weight: 1,
            },
        ],
    };

    match LoadbalancerSvcProcessor::build_layer(config) {
        Ok(_) => println!("edge_transport_http_egress_loadbalancer layer built"),
        Err(e) => println!("edge_transport_http_egress_loadbalancer: {e}"),
    }
}
