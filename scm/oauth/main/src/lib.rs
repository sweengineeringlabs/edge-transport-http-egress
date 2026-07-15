//! `edge_transport_http_egress_oauth` — OAuth2 token-refresh middleware for edge egress.
//!
//! Defines the [`OAuthTokenSource`] trait and a `reqwest-middleware` shell
//! ([`OAuthMiddleware`]) that wraps it. Concrete implementations (credential
//! file loaders, token-refresh HTTP calls) live in consumer crates.
//!
//! # Quick start
//!
//! ```rust,no_run
//! use std::sync::Arc;
//! use futures::future::BoxFuture;
//! use edge_transport_http_egress_oauth::{OAuthBuilderOps, OAuthTokenSource, OAuthSvc, Result};
//!
//! #[derive(Debug)]
//! struct MyTokenSource;
//!
//! impl OAuthTokenSource for MyTokenSource {
//!     fn get_access_token(&self) -> BoxFuture<'_, Result<String>> {
//!         Box::pin(async { Ok("my-token".to_string()) })
//!     }
//! }
//!
//! # fn main() -> std::result::Result<(), edge_transport_http_egress_oauth::OAuthError> {
//! let mw = OAuthSvc::builder()
//!     .with_token_source(Arc::new(MyTokenSource))
//!     .build()?;
//!
//! let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
//!     .with(mw)
//!     .build();
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use crate::api::{
    ApplicationConfigBuilder, OAuthBuilder, OAuthBuilderOps, OAuthConfig, OAuthCredentials,
    OAuthError, OAuthMiddleware, OAuthProvider, OAuthStrategy, OAuthSvc, OAuthTokenSource,
    Processor, Result, Validator,
};
