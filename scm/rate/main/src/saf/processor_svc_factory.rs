//! Composition site for [`Processor`] — one file per trait keeps wiring
//! focused.

use crate::api::{HttpRateSvcProcessor, Processor};

/// Factory for the default [`Processor`].
pub struct ProcessorFactory;

impl ProcessorFactory {
    /// Construct the default [`Processor`] — [`HttpRateSvcProcessor`], which
    /// identifies this crate's middleware as `"http-rate"`.
    pub fn create() -> Box<dyn Processor> {
        Box::new(HttpRateSvcProcessor)
    }
}
