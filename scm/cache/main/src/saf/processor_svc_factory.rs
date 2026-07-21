//! Composition site for [`Processor`] — one file per trait keeps wiring
//! focused.

use crate::api::{HttpCacheSvcProcessor, Processor};

/// Factory for the default [`Processor`].
pub struct ProcessorSvcFactory;

impl ProcessorSvcFactory {
    /// Construct the default [`Processor`] — [`HttpCacheSvcProcessor`], which
    /// identifies this crate's middleware as `"http-cache"`.
    pub fn create() -> Box<dyn Processor> {
        Box::new(HttpCacheSvcProcessor)
    }
}
