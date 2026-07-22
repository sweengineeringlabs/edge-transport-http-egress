//! Request for [`crate::api::HttpEgress::get`].

use serde::{Deserialize, Serialize};

/// Input to [`crate::api::HttpEgress::get`] — the URL to fetch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetRequest {
    /// The URL to send a GET request to.
    pub url: String,
}
