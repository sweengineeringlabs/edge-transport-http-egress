//! Minimal usage: build the auth middleware with the default (pass-through) config.

fn main() {
    match edge_transport_http_egress_auth::AuthSvc::build_auth_middleware(
        edge_transport_http_egress_auth::AuthConfig::default(),
    ) {
        Ok(mw) => println!("edge_transport_http_egress_auth middleware built: {:?}", mw),
        Err(e) => println!("edge_transport_http_egress_auth: {e}"),
    }
}
