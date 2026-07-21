//! Fluent builder for [`HttpConfig`].

use std::collections::HashMap;

/// Fluent builder for [`HttpConfig`].
///
/// Construct via [`HttpConfigBuilder::new`] and chain setter methods,
/// then call [`build`](Self::build) to obtain the final [`HttpConfig`].
#[derive(Debug, Default)]
pub struct HttpConfigBuilder {
    pub(crate) base_url: Option<String>,
    pub(crate) timeout_secs: Option<u64>,
    pub(crate) connect_timeout_secs: Option<u64>,
    pub(crate) max_retries: Option<u32>,
    pub(crate) default_headers: HashMap<String, String>,
    pub(crate) follow_redirects: Option<bool>,
    pub(crate) max_redirects: Option<u32>,
    pub(crate) user_agent: Option<String>,
    pub(crate) max_response_bytes: Option<Option<usize>>,
}
