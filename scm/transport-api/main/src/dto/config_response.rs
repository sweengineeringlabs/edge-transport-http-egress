//! Response for [`crate::HttpEgress::config`].

use crate::HttpConfig;

/// Output of [`crate::HttpEgress::config`] — the instance's
/// [`HttpConfig`], for introspection (e.g. logging which endpoint an egress
/// is configured against).
#[derive(Debug, Clone)]
pub struct ConfigResponse {
    /// The instance's configuration.
    pub config: HttpConfig,
}
