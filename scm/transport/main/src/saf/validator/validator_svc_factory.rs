//! Composition site for [`Validator`] — one file per trait keeps wiring
//! focused.

use crate::api::{AlwaysValidConfig, Validator};

/// Factory for the default [`Validator`].
pub struct ValidatorSvcFactory;

impl ValidatorSvcFactory {
    /// Construct the default, always-passing [`Validator`]
    /// ([`AlwaysValidConfig`]) — a zero-state marker for call sites that
    /// don't need real validation wired in.
    pub fn create() -> Box<dyn Validator> {
        Box::new(AlwaysValidConfig)
    }
}
