//! Request for [`crate::api::Processor::describe`].

use serde::{Deserialize, Serialize};

/// Input to [`crate::api::Processor::describe`] — no parameters are needed;
/// the struct exists to satisfy the uniform request/response contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescribeRequest;
