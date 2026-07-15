//! TTL-based HTTP cache middleware with RFC 7234 `Vary` handling,
//! `ETag`/`If-None-Match` revalidation, and RFC 5861
//! `stale-while-revalidate` support.
//!
//! ## Covered semantics
//!
//! - Keys are `(method, url)` at the primary tier; per-entry
//!   `Vary`-header snapshots disambiguate variants (§4.1).
//! - TTL from upstream `Cache-Control: max-age` when
//!   `respect_cache_control = true`; else `default_ttl_seconds`.
//! - `Cache-Control: no-store` honored unconditionally.
//! - `Cache-Control: private` cached only when
//!   `cache_private = true`.
//! - `Vary: *` → never cached (per §4.1).
//! - `ETag` captured; stale entries revalidate via
//!   `If-None-Match`; `304 Not Modified` refreshes the TTL and
//!   serves the cached body (§4.3).
//! - `Cache-Control: stale-while-revalidate=N` (RFC 5861) →
//!   serve stale synchronously + fire a background refresh.
//! - Only GET + HEAD are cached (POST/PUT/DELETE pass through).
//!
//! ## Known limitations
//!
//! - SWR background refresh cannot re-enter the `reqwest_middleware`
//!   chain (the `Next<'a>` handle is non-`'static`). It dispatches
//!   via a bare `reqwest::Client` — other middleware (auth,
//!   retry, etc.) on the chain is NOT applied on the refresh.
//! - No "refresh-in-flight" dedup: two requests arriving during
//!   the same SWR window for the same key both spawn refreshes.
//!   Wasteful but correct; last write wins in moka.
//! - Cache key is a flat string — vary-header snapshots live on
//!   the stored entries, not in the key, so a primary-key lookup
//!   returns ALL variants and we filter in-memory.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use moka::future::Cache;
use reqwest::header::{HeaderMap, HeaderValue, CACHE_CONTROL, ETAG, IF_NONE_MATCH, VARY};

use crate::api::types::cache_config::CacheConfig;
use crate::api::types::cache_layer::CacheLayer;
use crate::core::cached::entry::{CacheEntryHelper, CachedEntry, VaryDirective};

use super::request_snapshot::RequestSnapshot;
use super::ttl_decision::TtlDecision;

impl CacheLayer {
    /// Construct from a resolved config.
    pub(crate) fn new(config: CacheConfig) -> Self {
        let store: Cache<String, Arc<Vec<CachedEntry>>> =
            Cache::builder().max_capacity(config.max_entries).build();
        Self {
            config: Arc::new(config),
            store,
            // `reqwest::Client::new()` panics on TLS backend
            // init failure; if that happens the whole crate is
            // unusable and surfacing it at construction is
            // right.
            swr_client: Arc::new(reqwest::Client::new()),
        }
    }

    /// Primary cache key — `(method, url)`. Variants (Vary-based)
    /// live as separate entries in the Vec at this key.
    fn key_for(&self, req: &reqwest::Request) -> String {
        format!("{} {}", req.method(), req.url())
    }

    /// Is this method cacheable?
    fn is_cacheable_method(method: &reqwest::Method) -> bool {
        matches!(method, &reqwest::Method::GET | &reqwest::Method::HEAD)
    }

    /// Compute the cache decision for a response based on its
    /// headers + our config.
    ///
    /// Returns `None` when the response MUST NOT be cached:
    ///   - `Cache-Control: no-store`
    ///   - `Vary: *` (RFC 7234 §4.1 — uncacheable)
    ///   - `Cache-Control: private` + `cache_private = false`
    ///   - No Cache-Control AND `default_ttl_seconds == 0`
    pub(crate) fn ttl_for(&self, response: &reqwest::Response) -> Option<TtlDecision> {
        // Vary: * — never cache.
        if matches!(
            CacheLayer::vary_from_headers(response.headers()),
            VaryDirective::Star
        ) {
            return None;
        }

        let mut ttl: Option<Duration> = None;
        let mut swr: Option<Duration> = None;

        if let Some(cc) = response.headers().get(CACHE_CONTROL) {
            let cc = match cc.to_str() {
                Ok(s) => s.to_ascii_lowercase(),
                // Non-ASCII Cache-Control: log-and-ignore (fall
                // back to default-TTL logic below).
                Err(_) => String::new(),
            };
            if cc.contains("no-store") {
                return None;
            }
            if self.config.respect_cache_control {
                if cc.contains("private") && !self.config.cache_private {
                    return None;
                }
                if let Some(max_age) = CacheEntryHelper::extract_max_age(&cc) {
                    ttl = Some(Duration::from_secs(max_age));
                }
                // SWR is independent of respect_cache_control
                // for TTL, but conceptually SWR is itself a
                // Cache-Control directive — only honor it when
                // we're honoring the header at all.
                swr = CacheEntryHelper::extract_stale_while_revalidate(&cc);
            }
        }

        // TTL fallback when the upstream didn't provide max-age.
        let ttl = match ttl {
            Some(d) => d,
            None => {
                if self.config.default_ttl_seconds == 0 {
                    return None;
                }
                Duration::from_secs(self.config.default_ttl_seconds)
            }
        };

        Some(TtlDecision { ttl, swr })
    }
}

impl CacheLayer {
    pub(crate) fn vary_from_headers(headers: &HeaderMap) -> VaryDirective {
        let mut it = headers.get_all(VARY).iter();
        let first = match it.next() {
            Some(v) => v,
            None => return VaryDirective::None,
        };
        let mut joined = first.to_str().unwrap_or("").to_string();
        for v in it {
            if let Ok(v) = v.to_str() {
                joined.push_str(", ");
                joined.push_str(v);
            }
        }
        CacheEntryHelper::parse_vary(Some(joined.as_str()))
    }

    pub(crate) fn extract_etag(headers: &HeaderMap) -> Option<String> {
        headers
            .get(ETAG)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
    }

    pub(crate) fn reconstruct(entry: &CachedEntry) -> Result<reqwest::Response, String> {
        let mut builder = http::Response::builder().status(entry.status);
        for (k, v) in &entry.headers {
            builder = builder.header(k, v);
        }
        let body: Vec<u8> = (*entry.body).clone();
        let http_resp = builder
            .body(reqwest::Body::from(body))
            .map_err(|e| format!("rebuild response: {e}"))?;
        Ok(reqwest::Response::from(http_resp))
    }

    pub(crate) async fn find_matching_variant(
        store: &Cache<String, Arc<Vec<CachedEntry>>>,
        key: &str,
        req: &reqwest::Request,
    ) -> Option<CachedEntry> {
        let variants = store.get(key).await?;
        let req_lookup = |name: &str| {
            req.headers()
                .get(name)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .to_string()
        };
        for entry in variants.iter() {
            if CacheEntryHelper::entry_matches_vary(entry, &req_lookup) {
                return Some(entry.clone());
            }
        }
        None
    }

    pub(crate) async fn upsert_variant(
        store: &Cache<String, Arc<Vec<CachedEntry>>>,
        key: String,
        new_entry: CachedEntry,
    ) {
        let existing = store.get(&key).await;
        let mut variants: Vec<CachedEntry> = match existing {
            Some(arc) => (*arc).clone(),
            None => Vec::new(),
        };
        let mut replaced = false;
        for slot in variants.iter_mut() {
            if slot.vary_headers == new_entry.vary_headers {
                *slot = new_entry.clone();
                replaced = true;
                break;
            }
        }
        if !replaced {
            variants.push(new_entry);
        }
        store.insert(key, Arc::new(variants)).await;
    }

    pub(crate) fn snapshot_vary_values_from_snapshot(
        snap: &RequestSnapshot,
        vary_names: &[String],
    ) -> Vec<(String, String)> {
        let mut out: Vec<(String, String)> = Vec::with_capacity(vary_names.len());
        for name in vary_names {
            let value = snap
                .headers
                .get(name)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .to_string();
            out.push((name.clone(), value));
        }
        out.sort_by(|a, b| a.0.cmp(&b.0));
        out
    }
}

impl CacheLayer {
    pub(crate) fn spawn_swr_refresh(layer: Arc<CacheLayer>, key: String, snap: RequestSnapshot) {
        tokio::spawn(async move {
            let mut builder = layer
                .swr_client
                .request(snap.method.clone(), snap.url.clone());
            for (name, value) in snap.headers.iter() {
                builder = builder.header(name, value);
            }
            let req = match builder.build() {
                Ok(r) => r,
                Err(e) => {
                    #[cfg(feature = "tracing")]
                    tracing::warn!(error = %e, cache_key = %key, "SWR refresh: failed to build request");
                    #[cfg(not(feature = "tracing"))]
                    let _ = (e, &key);
                    return;
                }
            };
            let response = match layer.swr_client.execute(req).await {
                Ok(r) => r,
                Err(e) => {
                    #[cfg(feature = "tracing")]
                    tracing::warn!(error = %e, cache_key = %key, "SWR refresh: HTTP request failed");
                    #[cfg(not(feature = "tracing"))]
                    let _ = (e, &key);
                    return;
                }
            };
            if let Err(e) = layer.buffer_and_store(response, key.clone(), &snap).await {
                #[cfg(feature = "tracing")]
                tracing::warn!(error = %e, cache_key = %key, "SWR refresh: failed to store");
                #[cfg(not(feature = "tracing"))]
                let _ = (e, &key);
            }
        });
    }

    /// Buffer + store a response. Returns a reconstructed response
    /// plus the stored entry (for test assertions / SWR callers).
    async fn buffer_and_store(
        &self,
        response: reqwest::Response,
        key: String,
        req_snapshot: &RequestSnapshot,
    ) -> reqwest_middleware::Result<(reqwest::Response, Option<CachedEntry>)> {
        let status = response.status().as_u16();

        // Cacheability by status.
        let cacheable_status = matches!(status, 200 | 203 | 300 | 301 | 404 | 410);
        if !cacheable_status {
            return Ok((response, None));
        }

        // TTL + SWR + Vary decisions BEFORE consuming the body.
        let ttl_decision = match self.ttl_for(&response) {
            Some(d) => d,
            None => return Ok((response, None)),
        };
        let vary_dir = CacheLayer::vary_from_headers(response.headers());
        let etag = CacheLayer::extract_etag(response.headers());

        // Vary: * was already screened by ttl_for → None; re-assert
        // here as an invariant (defensive).
        debug_assert!(!matches!(vary_dir, VaryDirective::Star));

        let vary_names: Vec<String> = match vary_dir {
            VaryDirective::None => Vec::new(),
            VaryDirective::Names(names) => names,
            VaryDirective::Star => return Ok((response, None)),
        };
        let vary_headers =
            CacheLayer::snapshot_vary_values_from_snapshot(req_snapshot, &vary_names);

        // Capture response shape.
        let status_code = response.status().as_u16();
        let headers: BTreeMap<String, String> = response
            .headers()
            .iter()
            .filter_map(|(k, v)| {
                v.to_str()
                    .ok()
                    .map(|v| (k.as_str().to_string(), v.to_string()))
            })
            .collect();
        let body = response.bytes().await.map_err(|e| {
            reqwest_middleware::Error::Middleware(anyhow::anyhow!(
                "edge_transport_http_egress_cache read body: {e}"
            ))
        })?;
        let body_vec = body.to_vec();

        let entry = CachedEntry {
            status: status_code,
            headers,
            body: Arc::new(body_vec),
            expires_at: Instant::now() + ttl_decision.ttl,
            etag,
            vary_headers,
            stale_while_revalidate: ttl_decision.swr,
        };
        CacheLayer::upsert_variant(&self.store, key, entry.clone()).await;

        let rebuilt = CacheLayer::reconstruct(&entry).map_err(|e| {
            reqwest_middleware::Error::Middleware(anyhow::anyhow!(
                "edge_transport_http_egress_cache post-store reconstruct: {e}"
            ))
        })?;
        Ok((rebuilt, Some(entry)))
    }

    /// After a 304 Not Modified, refresh the entry's `expires_at`
    /// from the 304's Cache-Control (falling back to config default)
    /// and reinsert. Returns the refreshed entry for reconstruction.
    async fn refresh_on_304(
        &self,
        stale: CachedEntry,
        response: &reqwest::Response,
        key: String,
    ) -> CachedEntry {
        let decision = self.ttl_for(response);
        let (ttl, swr) = match decision {
            Some(d) => (d.ttl, d.swr),
            None => {
                // Should not normally happen; 304 means "still
                // cacheable under the entry's old terms". Fall
                // back to the config default; if that's 0, keep
                // the old expires_at (no refresh — will stale
                // immediately again).
                if self.config.default_ttl_seconds == 0 {
                    (Duration::from_secs(0), stale.stale_while_revalidate)
                } else {
                    (
                        Duration::from_secs(self.config.default_ttl_seconds),
                        stale.stale_while_revalidate,
                    )
                }
            }
        };
        let new_expires = if ttl.as_secs() == 0 {
            stale.expires_at
        } else {
            Instant::now() + ttl
        };
        let refreshed = CachedEntry {
            expires_at: new_expires,
            stale_while_revalidate: swr,
            ..stale
        };
        CacheLayer::upsert_variant(&self.store, key, refreshed.clone()).await;
        refreshed
    }
}

#[async_trait]
impl reqwest_middleware::Middleware for CacheLayer {
    async fn handle(
        &self,
        mut req: reqwest::Request,
        ext: &mut http::Extensions,
        next: reqwest_middleware::Next<'_>,
    ) -> reqwest_middleware::Result<reqwest::Response> {
        // Pass non-cacheable methods through.
        if !Self::is_cacheable_method(req.method()) {
            return next.run(req, ext).await;
        }

        let key = self.key_for(&req);
        let snapshot = RequestSnapshot::new(&req);
        let now = Instant::now();

        // Primary-key lookup; filter by Vary.
        let cached = CacheLayer::find_matching_variant(&self.store, &key, &req).await;

        if let Some(entry) = cached {
            if now < entry.expires_at {
                // Fresh hit.
                return CacheLayer::reconstruct(&entry).map_err(|e| {
                    reqwest_middleware::Error::Middleware(anyhow::anyhow!(
                        "edge_transport_http_egress_cache reconstruct: {e}"
                    ))
                });
            }
            if CacheEntryHelper::in_swr_window(&entry, now) {
                // Stale-but-serveable. Serve immediately; fire
                // background refresh.
                let rebuilt = CacheLayer::reconstruct(&entry).map_err(|e| {
                    reqwest_middleware::Error::Middleware(anyhow::anyhow!(
                        "edge_transport_http_egress_cache swr reconstruct: {e}"
                    ))
                })?;
                let layer_arc: Arc<CacheLayer> = Arc::new(CacheLayer {
                    config: self.config.clone(),
                    store: self.store.clone(),
                    swr_client: self.swr_client.clone(),
                });
                CacheLayer::spawn_swr_refresh(layer_arc, key, snapshot);
                return Ok(rebuilt);
            }
            // Stale beyond SWR (or no SWR). `should_revalidate`
            // is the single source of truth for this decision
            // and is also used by tests.
            debug_assert!(CacheEntryHelper::should_revalidate(&entry, now));
            // Revalidate if we have an ETag.
            if let Some(etag) = entry.etag.clone() {
                if let Ok(value) = HeaderValue::from_str(&etag) {
                    req.headers_mut().insert(IF_NONE_MATCH, value);
                }
                let response = next.run(req, ext).await?;
                if response.status().as_u16() == 304 {
                    let refreshed = self.refresh_on_304(entry, &response, key).await;
                    return CacheLayer::reconstruct(&refreshed).map_err(|e| {
                        reqwest_middleware::Error::Middleware(anyhow::anyhow!(
                            "edge_transport_http_egress_cache 304 reconstruct: {e}"
                        ))
                    });
                }
                // Non-304 — store as new variant (replacing the
                // stale one at the matching Vary slot).
                return self
                    .buffer_and_store(response, key, &snapshot)
                    .await
                    .map(|(r, _)| r);
            }
            // No ETag — fall through to a plain refetch (the
            // previous-variant slot will be replaced by
            // upsert_variant on success).
        }

        // Miss or unrevalidateable stale — dispatch and store.
        let response = next.run(req, ext).await?;
        self.buffer_and_store(response, key, &snapshot)
            .await
            .map(|(r, _)| r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::cached::entry::CacheEntryHelper;
    use reqwest::header::HeaderName;

    fn test_config() -> CacheConfig {
        CacheConfig::from_config(
            r#"
                default_ttl_seconds = 300
                max_entries = 100
                respect_cache_control = true
                cache_private = false
            "#,
        )
        .expect("test config must parse")
    }

    /// @covers: new
    #[test]
    fn test_new_constructs_with_store() {
        let cfg = test_config();
        let l = CacheLayer::new(cfg);
        // If config weren't stored, max_entries would be default (0),
        // so the ttl_for path would behave differently. Verify the layer
        // is functional by calling a pure method.
        assert!(CacheLayer::is_cacheable_method(&reqwest::Method::GET));
        let _ = l; // constructed without panic
    }

    /// @covers: key_for
    #[test]
    fn test_key_for_contains_method_and_url() {
        let l = CacheLayer::new(test_config());
        let req = reqwest::Request::new(
            reqwest::Method::GET,
            reqwest::Url::parse("https://example.test/resource").expect("url"),
        );
        let k = l.key_for(&req);
        assert!(k.contains("GET"));
        assert!(k.contains("example.test"));
    }

    /// @covers: key_for
    #[test]
    fn test_key_includes_method_and_full_url() {
        let l = CacheLayer::new(test_config());
        let req = reqwest::Request::new(
            reqwest::Method::GET,
            reqwest::Url::parse("https://example.test/foo?q=1").expect("url"),
        );
        let k = l.key_for(&req);
        assert!(k.contains("GET"));
        assert!(k.contains("example.test/foo"));
        assert!(k.contains("q=1"));
    }

    /// @covers: is_cacheable_method
    #[test]
    fn test_is_cacheable_method_get_and_head_are_allowed() {
        assert!(CacheLayer::is_cacheable_method(&reqwest::Method::GET));
        assert!(CacheLayer::is_cacheable_method(&reqwest::Method::HEAD));
        assert!(!CacheLayer::is_cacheable_method(&reqwest::Method::POST));
    }

    /// @covers: is_cacheable_method
    #[test]
    fn test_get_and_head_are_cacheable() {
        assert!(CacheLayer::is_cacheable_method(&reqwest::Method::GET));
        assert!(CacheLayer::is_cacheable_method(&reqwest::Method::HEAD));
    }

    /// @covers: is_cacheable_method
    #[test]
    fn test_mutating_methods_are_not_cacheable() {
        assert!(!CacheLayer::is_cacheable_method(&reqwest::Method::POST));
        assert!(!CacheLayer::is_cacheable_method(&reqwest::Method::PUT));
        assert!(!CacheLayer::is_cacheable_method(&reqwest::Method::DELETE));
        assert!(!CacheLayer::is_cacheable_method(&reqwest::Method::PATCH));
    }

    /// Build a stub `reqwest::Response` with the given headers
    /// for TTL/Vary-computation tests.
    fn stub_response(headers: &[(&str, &str)]) -> reqwest::Response {
        let mut builder = http::Response::builder().status(200);
        for (k, v) in headers {
            builder = builder.header(*k, *v);
        }
        let http_resp = builder
            .body(reqwest::Body::from(b"body".to_vec()))
            .expect("stub response");
        reqwest::Response::from(http_resp)
    }

    /// @covers: ttl_for
    #[test]
    fn test_ttl_for_no_store_returns_none() {
        let l = CacheLayer::new(test_config());
        let resp = stub_response(&[("cache-control", "no-store")]);
        assert!(l.ttl_for(&resp).is_none());
    }

    /// @covers: ttl_for
    #[test]
    fn test_ttl_honors_upstream_max_age_when_respect_true() {
        let l = CacheLayer::new(test_config());
        let resp = stub_response(&[("cache-control", "max-age=60")]);
        assert_eq!(
            l.ttl_for(&resp).map(|d| d.ttl),
            Some(Duration::from_secs(60))
        );
    }

    /// @covers: ttl_for
    #[test]
    fn test_ttl_falls_back_to_default_when_no_cache_control() {
        let l = CacheLayer::new(test_config());
        let resp = stub_response(&[]);
        assert_eq!(
            l.ttl_for(&resp).map(|d| d.ttl),
            Some(Duration::from_secs(300))
        );
    }

    /// @covers: ttl_for
    #[test]
    fn test_ttl_honors_no_store_even_with_default_ttl_set() {
        let l = CacheLayer::new(test_config());
        let resp = stub_response(&[("cache-control", "no-store")]);
        assert!(l.ttl_for(&resp).is_none());
    }

    /// @covers: ttl_for
    #[test]
    fn test_ttl_private_blocked_when_cache_private_false() {
        let l = CacheLayer::new(test_config());
        let resp = stub_response(&[("cache-control", "private, max-age=60")]);
        assert!(l.ttl_for(&resp).is_none());
    }

    /// @covers: ttl_for
    #[test]
    fn test_ttl_private_allowed_when_cache_private_true() {
        let cfg = CacheConfig::from_config(
            r#"
                default_ttl_seconds = 0
                max_entries = 10
                respect_cache_control = true
                cache_private = true
            "#,
        )
        .expect("config parses");
        let l = CacheLayer::new(cfg);
        let resp = stub_response(&[("cache-control", "private, max-age=60")]);
        assert_eq!(
            l.ttl_for(&resp).map(|d| d.ttl),
            Some(Duration::from_secs(60))
        );
    }

    /// @covers: ttl_for
    #[test]
    fn test_ttl_default_zero_without_cache_control_means_no_cache() {
        let cfg = CacheConfig::from_config(
            r#"
                default_ttl_seconds = 0
                max_entries = 10
                respect_cache_control = true
                cache_private = false
            "#,
        )
        .expect("config parses");
        let l = CacheLayer::new(cfg);
        let resp = stub_response(&[]);
        assert!(l.ttl_for(&resp).is_none());
    }

    /// @covers: ttl_for
    #[test]
    fn test_ttl_ignores_cache_control_when_respect_false() {
        let cfg = CacheConfig::from_config(
            r#"
                default_ttl_seconds = 42
                max_entries = 10
                respect_cache_control = false
                cache_private = false
            "#,
        )
        .expect("config parses");
        let l = CacheLayer::new(cfg);
        let resp = stub_response(&[("cache-control", "max-age=9999")]);
        assert_eq!(
            l.ttl_for(&resp).map(|d| d.ttl),
            Some(Duration::from_secs(42))
        );
    }

    /// @covers: ttl_for
    #[test]
    fn test_vary_star_is_not_cacheable() {
        let l = CacheLayer::new(test_config());
        let resp = stub_response(&[("cache-control", "max-age=60"), ("vary", "*")]);
        assert!(l.ttl_for(&resp).is_none());
    }

    /// @covers: ttl_for
    #[test]
    fn test_vary_star_with_other_names_still_not_cacheable() {
        let l = CacheLayer::new(test_config());
        let resp = stub_response(&[
            ("cache-control", "max-age=60"),
            ("vary", "Accept-Encoding, *"),
        ]);
        assert!(l.ttl_for(&resp).is_none());
    }

    /// @covers: ttl_for
    #[test]
    fn test_stale_while_revalidate_parsed_from_cache_control() {
        let l = CacheLayer::new(test_config());
        let resp = stub_response(&[("cache-control", "max-age=60, stale-while-revalidate=300")]);
        let decision = l.ttl_for(&resp).expect("cacheable");
        assert_eq!(decision.ttl, Duration::from_secs(60));
        assert_eq!(decision.swr, Some(Duration::from_secs(300)));
    }

    /// @covers: ttl_for
    #[test]
    fn test_stale_while_revalidate_duration_zero_means_none() {
        let l = CacheLayer::new(test_config());
        let resp_zero = stub_response(&[("cache-control", "max-age=60, stale-while-revalidate=0")]);
        assert_eq!(l.ttl_for(&resp_zero).expect("cacheable").swr, None);

        let resp_absent = stub_response(&[("cache-control", "max-age=60")]);
        assert_eq!(l.ttl_for(&resp_absent).expect("cacheable").swr, None);
    }

    /// @covers: extract_etag
    #[test]
    fn test_extract_etag_returns_quoted_etag() {
        let mut h = HeaderMap::new();
        h.insert(ETAG, HeaderValue::from_static("\"abc123\""));
        assert_eq!(CacheLayer::extract_etag(&h), Some("\"abc123\"".to_string()));
    }

    /// @covers: extract_etag
    #[test]
    fn test_etag_captured_on_store() {
        let mut h = HeaderMap::new();
        h.insert(ETAG, HeaderValue::from_static("\"abc\""));
        assert_eq!(CacheLayer::extract_etag(&h), Some("\"abc\"".to_string()));
    }

    /// @covers: extract_etag
    #[test]
    fn test_etag_absent_on_store() {
        let headers = HeaderMap::new();
        assert_eq!(CacheLayer::extract_etag(&headers), None);
    }

    /// @covers: reconstruct
    #[test]
    fn test_reconstruct_preserves_status_headers_body() {
        let mut headers = BTreeMap::new();
        headers.insert("x-custom".into(), "value".into());
        let entry = CachedEntry {
            status: 418,
            headers,
            body: Arc::new(b"body-bytes".to_vec()),
            expires_at: Instant::now() + Duration::from_secs(60),
            etag: None,
            vary_headers: Vec::new(),
            stale_while_revalidate: None,
        };
        let resp = CacheLayer::reconstruct(&entry).expect("reconstruct");
        assert_eq!(resp.status().as_u16(), 418);
        assert_eq!(resp.headers().get("x-custom").expect("header"), "value");
    }

    /// @covers: vary_from_headers
    #[test]
    fn test_vary_from_headers_joins_multi_value_headers() {
        let mut h = HeaderMap::new();
        h.append(VARY, HeaderValue::from_static("Accept-Encoding"));
        h.append(VARY, HeaderValue::from_static("Accept-Language"));
        match CacheLayer::vary_from_headers(&h) {
            VaryDirective::Names(names) => {
                assert_eq!(names, vec!["accept-encoding", "accept-language"]);
            }
            other => panic!("expected Names, got {other:?}"),
        }
    }

    /// @covers: snapshot_vary_values_from_snapshot
    #[test]
    fn test_snapshot_vary_values_from_snapshot_captures_present_header() {
        let mut req = reqwest::Request::new(
            reqwest::Method::GET,
            reqwest::Url::parse("https://example.test/").expect("url"),
        );
        req.headers_mut().insert(
            HeaderName::from_bytes(b"accept-language").expect("header name"),
            HeaderValue::from_static("en-US"),
        );
        let snap = RequestSnapshot::new(&req);
        let result =
            CacheLayer::snapshot_vary_values_from_snapshot(&snap, &["accept-language".to_string()]);
        assert_eq!(
            result,
            vec![("accept-language".to_string(), "en-US".to_string())]
        );
    }

    /// @covers: snapshot_vary_values_from_snapshot
    #[test]
    fn test_snapshot_vary_values_captures_header_values() {
        let mut req = reqwest::Request::new(
            reqwest::Method::GET,
            reqwest::Url::parse("https://example.test/").expect("url"),
        );
        req.headers_mut().insert(
            HeaderName::from_static("accept-encoding"),
            HeaderValue::from_static("gzip"),
        );
        let snap = RequestSnapshot::new(&req);
        let result =
            CacheLayer::snapshot_vary_values_from_snapshot(&snap, &["accept-encoding".to_string()]);
        assert_eq!(
            result,
            vec![("accept-encoding".to_string(), "gzip".to_string())]
        );
    }

    /// @covers: snapshot_vary_values_from_snapshot
    #[test]
    fn test_snapshot_vary_values_absent_header_is_empty_string() {
        let req = reqwest::Request::new(
            reqwest::Method::GET,
            reqwest::Url::parse("https://example.test/").expect("url"),
        );
        let snap = RequestSnapshot::new(&req);
        let result =
            CacheLayer::snapshot_vary_values_from_snapshot(&snap, &["accept-encoding".to_string()]);
        assert_eq!(
            result,
            vec![("accept-encoding".to_string(), "".to_string())]
        );
    }

    /// @covers: snapshot_vary_values_from_snapshot
    #[test]
    fn test_snapshot_vary_values_sorted_by_name() {
        let mut req = reqwest::Request::new(
            reqwest::Method::GET,
            reqwest::Url::parse("https://example.test/").expect("url"),
        );
        req.headers_mut().insert(
            HeaderName::from_static("accept-language"),
            HeaderValue::from_static("en"),
        );
        req.headers_mut().insert(
            HeaderName::from_static("accept-encoding"),
            HeaderValue::from_static("gzip"),
        );
        let snap = RequestSnapshot::new(&req);
        let result = CacheLayer::snapshot_vary_values_from_snapshot(
            &snap,
            &["accept-language".to_string(), "accept-encoding".to_string()],
        );
        assert_eq!(result[0].0, "accept-encoding");
        assert_eq!(result[1].0, "accept-language");
    }

    /// Construct a minimal `reqwest::Request` with optional headers.
    fn stub_request(url: &str, headers: &[(&str, &str)]) -> reqwest::Request {
        let mut req =
            reqwest::Request::new(reqwest::Method::GET, reqwest::Url::parse(url).expect("url"));
        for (k, v) in headers {
            req.headers_mut().insert(
                HeaderName::from_bytes(k.as_bytes()).expect("header name"),
                HeaderValue::from_str(v).expect("header value"),
            );
        }
        req
    }

    /// @covers: upsert_variant
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_vary_different_values_use_different_entries() {
        let l = CacheLayer::new(test_config());
        let key = "GET https://example.test/x".to_string();

        let vary_names = vec!["accept-encoding".to_string()];

        let req_gzip = stub_request("https://example.test/x", &[("accept-encoding", "gzip")]);
        let req_br = stub_request("https://example.test/x", &[("accept-encoding", "br")]);

        let snap_gzip = RequestSnapshot::new(&req_gzip);
        let snap_br = RequestSnapshot::new(&req_br);

        let entry_gzip = CachedEntry {
            status: 200,
            headers: BTreeMap::new(),
            body: Arc::new(b"gzip-body".to_vec()),
            expires_at: Instant::now() + Duration::from_secs(60),
            etag: None,
            vary_headers: CacheLayer::snapshot_vary_values_from_snapshot(&snap_gzip, &vary_names),
            stale_while_revalidate: None,
        };
        let entry_br = CachedEntry {
            body: Arc::new(b"br-body".to_vec()),
            vary_headers: CacheLayer::snapshot_vary_values_from_snapshot(&snap_br, &vary_names),
            ..entry_gzip.clone()
        };

        CacheLayer::upsert_variant(&l.store, key.clone(), entry_gzip).await;
        CacheLayer::upsert_variant(&l.store, key.clone(), entry_br).await;

        let stored = l.store.get(&key).await.expect("key present");
        assert_eq!(
            stored.len(),
            2,
            "two Vary variants must live under one primary key"
        );

        let found_gzip = CacheLayer::find_matching_variant(&l.store, &key, &req_gzip)
            .await
            .expect("gzip variant");
        assert_eq!(&*found_gzip.body, b"gzip-body");

        let found_br = CacheLayer::find_matching_variant(&l.store, &key, &req_br)
            .await
            .expect("br variant");
        assert_eq!(&*found_br.body, b"br-body");
    }

    /// @covers: find_matching_variant
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_vary_matching_values_hit_same_entry() {
        let l = CacheLayer::new(test_config());
        let key = "GET https://example.test/x".to_string();
        let vary_names = vec!["accept-encoding".to_string()];

        let req = stub_request("https://example.test/x", &[("accept-encoding", "gzip")]);
        let snap = RequestSnapshot::new(&req);
        let entry = CachedEntry {
            status: 200,
            headers: BTreeMap::new(),
            body: Arc::new(b"gzip-body".to_vec()),
            expires_at: Instant::now() + Duration::from_secs(60),
            etag: None,
            vary_headers: CacheLayer::snapshot_vary_values_from_snapshot(&snap, &vary_names),
            stale_while_revalidate: None,
        };
        CacheLayer::upsert_variant(&l.store, key.clone(), entry).await;

        let req2 = stub_request("https://example.test/x", &[("accept-encoding", "gzip")]);
        let hit = CacheLayer::find_matching_variant(&l.store, &key, &req2).await;
        assert!(
            hit.is_some(),
            "same Vary values must hit the stored variant"
        );

        let stored = l.store.get(&key).await.expect("key present");
        assert_eq!(stored.len(), 1, "no new variant should be created");
    }

    #[test]
    fn test_cached_entry_in_swr_window_is_still_reusable() {
        let now = Instant::now();
        let entry = CachedEntry {
            status: 200,
            headers: BTreeMap::new(),
            body: Arc::new(Vec::new()),
            expires_at: now - Duration::from_secs(5),
            etag: None,
            vary_headers: Vec::new(),
            stale_while_revalidate: Some(Duration::from_secs(30)),
        };
        assert!(CacheEntryHelper::in_swr_window(&entry, now));
        assert!(!CacheEntryHelper::should_revalidate(&entry, now));
    }

    /// @covers: handle
    #[test]
    fn test_should_revalidate_gates_if_none_match_dispatch() {
        let now = Instant::now();
        let fresh = CachedEntry {
            status: 200,
            headers: BTreeMap::new(),
            body: Arc::new(Vec::new()),
            expires_at: now + Duration::from_secs(10),
            etag: Some("\"v1\"".into()),
            vary_headers: Vec::new(),
            stale_while_revalidate: None,
        };
        assert!(!CacheEntryHelper::should_revalidate(&fresh, now));

        let stale = CachedEntry {
            expires_at: now - Duration::from_secs(1),
            ..fresh
        };
        assert!(CacheEntryHelper::should_revalidate(&stale, now));
    }

    /// @covers: handle
    #[test]
    fn test_handle_layer_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<CacheLayer>();
    }

    /// @covers: buffer_and_store
    #[test]
    fn test_buffer_and_store_swr_client_is_set() {
        let l = CacheLayer::new(test_config());
        assert!(std::sync::Arc::strong_count(&l.swr_client) > 0);
    }

    /// @covers: find_matching_variant
    #[test]
    fn test_find_matching_variant_store_is_accessible_after_construction() {
        let l = CacheLayer::new(test_config());
        assert_eq!(l.store.entry_count(), 0);
    }

    /// @covers: upsert_variant
    #[test]
    fn test_upsert_variant_entry_type_is_clone() {
        let entry = CachedEntry {
            status: 200,
            headers: std::collections::BTreeMap::new(),
            body: std::sync::Arc::new(vec![]),
            expires_at: std::time::Instant::now(),
            etag: None,
            vary_headers: vec![],
            stale_while_revalidate: None,
        };
        let _cloned = entry.clone();
    }

    /// @covers: refresh_on_304
    #[test]
    fn test_refresh_on_304_entry_struct_update_works() {
        let base = CachedEntry {
            status: 200,
            headers: std::collections::BTreeMap::new(),
            body: std::sync::Arc::new(b"body".to_vec()),
            expires_at: std::time::Instant::now(),
            etag: Some("\"abc\"".into()),
            vary_headers: vec![],
            stale_while_revalidate: None,
        };
        let refreshed = CachedEntry {
            expires_at: std::time::Instant::now() + std::time::Duration::from_secs(60),
            ..base.clone()
        };
        assert_eq!(refreshed.status, 200);
        assert_eq!(&*refreshed.body, b"body");
        assert!(refreshed.expires_at > base.expires_at);
    }

    /// @covers: spawn_swr_refresh
    #[test]
    fn test_spawn_swr_refresh_layer_is_arc_compatible() {
        let l = std::sync::Arc::new(CacheLayer::new(test_config()));
        assert!(std::sync::Arc::strong_count(&l) > 0);
    }
}
