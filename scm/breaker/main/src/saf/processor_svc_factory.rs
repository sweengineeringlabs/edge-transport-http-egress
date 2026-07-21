//! Composition site for [`Processor`] — one file per trait keeps wiring
//! focused.

use crate::api::{HttpBreakerSvcProcessor, Processor};

/// Factory for the default [`Processor`].
pub struct ProcessorFactory;

impl ProcessorFactory {
    /// Construct the default [`Processor`] — [`HttpBreakerSvcProcessor`], which
    /// identifies this crate's middleware as `"http-breaker"`.
    pub fn create() -> Box<dyn Processor> {
        Box::new(HttpBreakerSvcProcessor)
    }
}
