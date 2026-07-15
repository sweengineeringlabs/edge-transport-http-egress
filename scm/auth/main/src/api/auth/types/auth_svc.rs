//! `AuthSvc` — SAF service facade type for `edge-transport-http-egress-auth`.

/// Service facade for the auth middleware crate.
///
/// Methods delegate to core implementations via saf wrappers.
pub struct AuthSvc;

impl AuthSvc {
    /// Return a config builder pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        crate::saf::create_config_builder()
    }

    /// Build an [`AuthMiddleware`](crate::api::auth::types::AuthMiddleware)
    /// from a caller-supplied config.
    ///
    /// Uses the default credential resolver to resolve every env-var
    /// reference in the config at call time. A missing env var fails with
    /// [`AuthError::MissingEnvVar`](crate::api::auth::errors::AuthError)
    /// so startup (not the first request) surfaces the misconfiguration.
    ///
    /// The returned [`AuthMiddleware`](crate::api::auth::types::AuthMiddleware)
    /// implements `reqwest_middleware::Middleware` — plug into a
    /// `reqwest_middleware::ClientBuilder` via `.with(mw)`.
    pub fn build_auth_middleware(
        config: crate::api::auth::types::auth_config::AuthConfig,
    ) -> Result<crate::api::auth::types::AuthMiddleware, crate::api::auth::errors::AuthError> {
        crate::saf::build_auth_middleware(config)
    }
}
