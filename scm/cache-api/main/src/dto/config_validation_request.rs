//! Request for [`crate::Validator::validate`].

use serde::{Deserialize, Serialize};

use crate::CacheConfig;

/// Input to [`crate::Validator::validate`] — the config to check for
/// structural validity before it's used to build a
/// [`CacheLayer`](crate::CacheLayer).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigValidationRequest {
    /// The config to validate.
    pub config: CacheConfig,
}
