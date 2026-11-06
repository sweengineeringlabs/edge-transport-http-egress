//! Interface contract for the validator abstraction.

use crate::Validator;

/// Dyn-safe alias for the validator trait object interface.
pub type ValidatorObject = dyn Validator;
