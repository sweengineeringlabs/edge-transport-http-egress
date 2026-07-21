//! Composition site for [`Processor`] — one file per trait keeps wiring
//! focused.

use crate::api::{HttpCassetteSvc, Processor};

/// Factory for the default [`Processor`].
pub struct ProcessorFactory;

impl ProcessorFactory {
    /// Construct the default [`Processor`] — [`HttpCassetteSvc`], which
    /// identifies this crate's middleware as `"http-cassette"`.
    pub fn create() -> Box<dyn Processor> {
        Box::new(HttpCassetteSvc)
    }
}
