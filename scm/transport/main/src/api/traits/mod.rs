//! SEA interface contract — outbound transport traits.

mod http_egress;
mod http_stream;
mod validator;

pub use http_egress::HttpEgress;
pub use http_stream::HttpStream;
pub use validator::Validator;
