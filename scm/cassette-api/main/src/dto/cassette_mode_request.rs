//! Request for [`crate::HttpCassette::mode`].

use serde::{Deserialize, Serialize};

/// Input to [`crate::HttpCassette::mode`] — no parameters are needed;
/// the struct exists to satisfy the uniform request/response contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CassetteModeRequest;
