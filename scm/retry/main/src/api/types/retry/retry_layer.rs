//! Public type — the retry middleware layer consumers plug into
//! `reqwest_middleware::ClientBuilder::with(..)`.
//!
//! Declaration only. The constructor, backoff logic, and the
//! `reqwest_middleware::Middleware` impl live in `core/`.

use std::sync::Arc;

use crate::api::RetryConfig;

/// Retry middleware layer. Opaque handle — consumers get one from
/// `HttpRetrySvc.decorate(DecorateRequest { config })?.layer` and pass it to
/// `reqwest_middleware::ClientBuilder`.
pub struct RetryLayer {
    pub(crate) config: Arc<RetryConfig>,
}
