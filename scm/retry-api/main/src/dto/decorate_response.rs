//! Response DTO for [`crate::Processor::decorate`].

/// Output of [`crate::Processor::decorate`] — the built retry
/// middleware layer.
#[derive(Debug)]
pub struct DecorateResponse {
    /// The constructed retry middleware layer.
    pub layer: std::sync::Arc<dyn crate::RetryLayer>,
}
