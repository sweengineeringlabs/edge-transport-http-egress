//! Composition site for [`Validator`] — one file per trait keeps wiring
//! focused.

use crate::api::Validator;
use crate::core::cache::default_validator::DefaultValidator;

/// Factory for the default [`Validator`].
pub struct ValidatorSvcFactory;

impl ValidatorSvcFactory {
    /// Construct the default [`Validator`] for [`CacheConfig`](crate::api::CacheConfig).
    pub fn create() -> Box<dyn Validator> {
        Box::new(DefaultValidator)
    }
}
