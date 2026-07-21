//! `Processor` — primary processing trait for the retry crate.

use crate::api::ApplicationConfigBuilder;
use crate::api::DecorateRequest;
use crate::api::DecorateResponse;
use crate::api::ProcessorRequest;
use crate::api::ProcessorResponse;
use crate::api::RetryConfigBuilder;
use crate::api::RetryError;

/// Primary processing contract (service_type = "processor").
///
/// Implemented by [`HttpRetrySvc`](crate::api::HttpRetrySvc) in `core/`.
/// Middleware components identify themselves via [`Processor::describe`]
/// and build a middleware layer via [`Processor::decorate`].
pub trait Processor {
    /// Identify this processor unit for log / trace output.
    fn describe(&self, req: ProcessorRequest) -> Result<ProcessorResponse, RetryError>;

    /// Build a retry middleware layer from a resolved policy.
    fn decorate(&self, req: DecorateRequest) -> Result<DecorateResponse, RetryError>;

    /// Construct a fluent [`RetryConfigBuilder`] seeded with SWE defaults.
    ///
    /// Associated (no `self`) so it can be called without an instance;
    /// keeps the trait dyn-compatible for `Box<dyn Processor>`.
    fn new_config_builder() -> RetryConfigBuilder
    where
        Self: Sized;

    /// Construct the [`ApplicationConfigBuilder`] bound to
    /// `config/application.toml`, pre-seeded with the crate identity.
    fn new_app_config_builder() -> ApplicationConfigBuilder
    where
        Self: Sized;
}
