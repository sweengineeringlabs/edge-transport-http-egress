//! Request for [`crate::Validator::validate`].

use serde::{Deserialize, Serialize};

/// Input to [`crate::Validator::validate`] — no parameters are needed;
/// the struct exists to satisfy the uniform request/response contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidateRequest;
