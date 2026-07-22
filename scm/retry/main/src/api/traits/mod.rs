//! SEA interface contracts — primary traits for `edge-transport-http-egress-retry`.

mod processor;
mod validator;

pub use processor::Processor;
pub use validator::Validator;
