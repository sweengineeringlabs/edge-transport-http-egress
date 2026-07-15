//! `OAuthTokenSource` — public extension point for OAuth token providers.
//!
//! Implement this trait to plug your own credential store into the edge
//! OAuth middleware. The middleware calls `get_access_token()` on every
//! outbound request; your implementation is responsible for proactive
//! refresh and any credential file I/O.

use futures::future::BoxFuture;

use crate::api::refresh::errors::Result;

/// Provides a valid OAuth2 access token on demand.
///
/// Implementations are responsible for:
/// - Loading credentials from their backing store (file, env, secret manager …)
/// - Proactively refreshing the token before it expires
/// - Persisting refreshed credentials back to the store
///
/// The middleware calls this once per request; implementations should cache
/// the current token in memory and only hit the token endpoint when necessary.
///
/// # Examples
///
/// ```rust,no_run
/// use futures::future::BoxFuture;
/// use edge_transport_http_egress_oauth::{OAuthTokenSource, OAuthError};
///
/// /// Static token source — useful in tests and for long-lived service accounts.
/// #[derive(Debug)]
/// struct StaticToken(String);
///
/// impl OAuthTokenSource for StaticToken {
///     fn get_access_token(&self) -> BoxFuture<'_, Result<String, OAuthError>> {
///         let token = self.0.clone();
///         Box::pin(async move { Ok(token) })
///     }
/// }
///
/// // Wire into OAuthSvc::builder().token_source(Box::new(StaticToken("my-token".to_string())))
/// ```
pub trait OAuthTokenSource: Send + Sync + std::fmt::Debug + 'static {
    /// Return a valid access token. Must not return an expired token.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use edge_transport_http_egress_oauth::{OAuthTokenSource, OAuthError};
    /// // Implement get_access_token to return a fresh token from your credential store.
    /// // The middleware calls this on every outbound request.
    /// ```
    fn get_access_token(&self) -> BoxFuture<'_, Result<String>>;
}
