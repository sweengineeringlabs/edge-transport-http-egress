//! Request for [`crate::api::Validator::validate`].

use serde::{Deserialize, Serialize};

/// Input to [`crate::api::Validator::validate`] — no parameters are needed;
/// the struct exists to satisfy the uniform request/response contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidateRequest;
