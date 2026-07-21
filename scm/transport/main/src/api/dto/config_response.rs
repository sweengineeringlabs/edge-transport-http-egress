//! Response for [`crate::api::HttpEgress::config`].

use crate::api::types::http::HttpConfig;

/// Output of [`crate::api::HttpEgress::config`] — the instance's
/// [`HttpConfig`], for introspection (e.g. logging which endpoint an egress
/// is configured against).
#[derive(Debug, Clone)]
pub struct ConfigResponse {
    /// The instance's configuration.
    pub config: HttpConfig,
}
