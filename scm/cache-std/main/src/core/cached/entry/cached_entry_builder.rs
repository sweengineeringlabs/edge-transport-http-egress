//! Fluent builder for [`CachedEntry`] — avoids a 7-field telescoping
//! constructor at the one call site that builds a fresh entry from scratch
//! (`MiddlewareHttpCache::store_response`); the `..stale` update-syntax refresh in
//! `refresh_on_304` needs no builder.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::cached_entry::CachedEntry;

pub(crate) struct CachedEntryBuilder {
    status: u16,
    headers: BTreeMap<String, String>,
    body: Arc<Vec<u8>>,
    expires_at: Instant,
    etag: Option<String>,
    vary_headers: Vec<(String, String)>,
    stale_while_revalidate: Option<Duration>,
}

impl CachedEntryBuilder {
    /// Start a builder with the required fields — no response can be cached
    /// without a status, body, and freshness deadline.
    pub(crate) fn new(status: u16, body: Arc<Vec<u8>>, expires_at: Instant) -> Self {
        Self {
            status,
            headers: BTreeMap::new(),
            body,
            expires_at,
            etag: None,
            vary_headers: Vec::new(),
            stale_while_revalidate: None,
        }
    }

    pub(crate) fn headers(mut self, headers: BTreeMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    pub(crate) fn etag(mut self, etag: Option<String>) -> Self {
        self.etag = etag;
        self
    }

    pub(crate) fn vary_headers(mut self, vary_headers: Vec<(String, String)>) -> Self {
        self.vary_headers = vary_headers;
        self
    }

    pub(crate) fn stale_while_revalidate(mut self, swr: Option<Duration>) -> Self {
        self.stale_while_revalidate = swr;
        self
    }

    pub(crate) fn build(self) -> CachedEntry {
        CachedEntry {
            status: self.status,
            headers: self.headers,
            body: self.body,
            expires_at: self.expires_at,
            etag: self.etag,
            vary_headers: self.vary_headers,
            stale_while_revalidate: self.stale_while_revalidate,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: build
    #[test]
    fn test_build_with_required_fields_only_uses_none_defaults() {
        let entry = CachedEntryBuilder::new(200, Arc::new(vec![1, 2, 3]), Instant::now()).build();
        assert_eq!(entry.status, 200);
        assert!(entry.etag.is_none());
        assert!(entry.vary_headers.is_empty());
    }

    /// @covers: etag
    #[test]
    fn test_etag_sets_field() {
        let entry = CachedEntryBuilder::new(200, Arc::new(vec![]), Instant::now())
            .etag(Some("\"abc123\"".to_string()))
            .build();
        assert_eq!(entry.etag.as_deref(), Some("\"abc123\""));
    }

    /// @covers: headers
    #[test]
    fn test_headers_sets_field() {
        let mut h = BTreeMap::new();
        h.insert("content-type".to_string(), "application/json".to_string());
        let entry = CachedEntryBuilder::new(200, Arc::new(vec![]), Instant::now())
            .headers(h.clone())
            .build();
        assert_eq!(entry.headers, h);
    }
}
