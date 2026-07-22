//! Composition site for [`Processor`] — one file per trait keeps wiring
//! focused.

use crate::api::{LoadbalancerSvcProcessor, Processor};

/// Factory for the default [`Processor`].
pub struct ProcessorFactory;

impl ProcessorFactory {
    /// Construct the default [`Processor`] — [`LoadbalancerSvcProcessor`],
    /// which identifies this crate's middleware by its canonical crate name.
    pub fn create() -> Box<dyn Processor> {
        Box::new(LoadbalancerSvcProcessor)
    }
}
