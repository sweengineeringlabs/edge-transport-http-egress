//! Request for [`crate::api::Validator::validate`].

use serde::{Deserialize, Serialize};

use crate::api::LoadbalancerConfig;

/// Input to [`crate::api::Validator::validate`] — the config to check for
/// structural validity before it is used to build a
/// [`LoadbalancerLayer`](crate::api::LoadbalancerLayer).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigValidationRequest {
    /// The config to validate.
    pub config: LoadbalancerConfig,
}
