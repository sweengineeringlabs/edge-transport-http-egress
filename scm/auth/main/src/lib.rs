//! edge_transport_http_egress_auth — HTTP auth middleware for reqwest-middleware.
//!
//! Attaches bearer tokens, basic-auth credentials, or custom
//! API-key headers to outbound HTTP requests. Credentials are
//! resolved from environment variables at config-load time; the
//! config itself stores only the env-var NAME, never the raw
//! credential.
//!

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use crate::api::{
    ApplicationConfigBuilder, AuthConfig, AuthError, AuthMiddleware, AuthStrategy, AuthSvc,
    AwsSigV4StrategyBuilder, AwsSigV4StrategyConfig, AwsSigV4StrategyConfigBuilder,
    CredentialResolver, CredentialSource, CredentialSourceConfig, CredentialSourceResolver,
    HttpAuth, OAuthTokenSourceFactory, Processor, Validator,
};
pub use crate::core::credential::FileCredentialResolver;
