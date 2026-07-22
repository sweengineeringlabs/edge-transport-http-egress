//! SAF facade — public surface for the HTTP egress transport crate.

mod http;
mod http_egress_svc_factory;
mod http_stream_svc_factory;
mod transport_svc;
mod validator;

pub use http_egress_svc_factory::HttpEgressSvcFactory;
pub use http_stream_svc_factory::HttpStreamSvcFactory;
pub use validator::ValidatorSvcFactory;
