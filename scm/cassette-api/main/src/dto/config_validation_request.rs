//! Request for [`crate::Validator::validate`].

use serde::{Deserialize, Serialize};

use crate::CassetteConfig;

/// Input to [`crate::Validator::validate`] — the config to check for
/// structural validity before it is used to build a cassette layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigValidationRequest {
    /// The config to validate.
    pub config: CassetteConfig,
}
