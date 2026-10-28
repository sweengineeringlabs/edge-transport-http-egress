//! SEA interface contract — outbound transport traits.

use std::future::Future;
use std::pin::Pin;

use crate::api::error::HttpEgressError;

mod http_egress;
mod http_stream;
mod validator;

pub use http_egress::HttpEgress;
pub use http_stream::HttpStream;
pub use validator::Validator;

/// Object-safe asynchronous result used by the outbound HTTP ports.
///
/// This is equivalent to the future type used by `async_trait`, expressed
/// directly so strict Clippy can inspect the public trait without
/// macro-generated `must_use` attributes.
pub type HttpEgressFuture<'a, T> =
    Pin<Box<dyn Future<Output = Result<T, HttpEgressError>> + Send + 'a>>;
