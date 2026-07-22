//! Request for [`crate::api::Validator::validate`].

use serde::{Deserialize, Serialize};

use crate::api::BreakerConfig;

/// Input to [`crate::api::Validator::validate`] — the config to check for
/// structural validity before it's used to build a [`BreakerLayerBreakerMetrics`](crate::api::BreakerLayerBreakerMetrics).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigValidationRequest {
    /// The config to validate.
    pub config: BreakerConfig,
}
