//! Composition site for [`Validator`] — one file per trait keeps wiring
//! focused.

use crate::api::Validator;
use crate::core::DefaultValidator;

/// Factory for the default [`Validator`].
pub struct ValidatorFactory;

impl ValidatorFactory {
    /// Construct the default [`Validator`] for
    /// [`LoadbalancerConfig`](crate::api::LoadbalancerConfig).
    pub fn create() -> Box<dyn Validator> {
        Box::new(DefaultValidator)
    }
}
