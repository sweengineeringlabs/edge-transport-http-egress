//! Composition site for [`Processor`] — one file per trait keeps wiring
//! focused.

use crate::api::{HttpRetrySvc, Processor};

/// Factory for the default [`Processor`].
pub struct ProcessorFactory;

impl ProcessorFactory {
    /// Construct the default [`Processor`] — [`HttpRetrySvc`], which
    /// identifies this crate's middleware as `"http-retry"`.
    pub fn create() -> Box<dyn Processor> {
        Box::new(HttpRetrySvc)
    }
}
