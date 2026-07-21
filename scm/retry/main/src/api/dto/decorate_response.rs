//! Response DTO for [`crate::api::Processor::decorate`].

/// Output of [`crate::api::Processor::decorate`] — the built retry
/// middleware layer.
#[derive(Debug)]
pub struct DecorateResponse {
    /// The constructed retry middleware layer.
    pub layer: crate::api::RetryLayer,
}
