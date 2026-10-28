//! Fluent builder for [`crate::api::HttpConfig`].

use std::collections::HashMap;

/// Fluent builder for [`crate::api::HttpConfig`].
///
/// Construct via [`HttpConfigBuilder::new`] and chain setter methods,
/// then call [`build`](Self::build) to obtain the final [`crate::api::HttpConfig`].
#[derive(Debug, Default)]
pub struct HttpConfigBuilder {
    pub(crate) base_url: Option<String>,
    pub(crate) timeout_secs: Option<u64>,
    pub(crate) connect_timeout_secs: Option<u64>,
    pub(crate) default_headers: HashMap<String, String>,
    pub(crate) user_agent: Option<String>,
}
