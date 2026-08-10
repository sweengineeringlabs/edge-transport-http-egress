//! SAF facade — public surface for the HTTP egress transport crate.

mod http;
mod transport_svc;
mod validator;

pub use validator::ValidatorSvcFactory;
