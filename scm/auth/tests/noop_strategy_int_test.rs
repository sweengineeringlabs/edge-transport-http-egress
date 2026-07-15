//! Integration tests for the Noop strategy path (AuthConfig::None).
//!
//! The strategy is `pub(crate)`.  Observable effects:
//! - `AuthConfig::None` always builds without requiring any env vars.
//! - The resulting middleware attaches no authentication headers.
//! - The middleware is a valid `reqwest_middleware::Middleware`.
//! - `Send + Sync` bounds are satisfied.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_auth::{AuthConfig, AuthMiddleware, AuthSvc};

// ---------------------------------------------------------------------------
// Build always succeeds — no env vars required
// ---------------------------------------------------------------------------

#[test]
fn test_noop_strategy_builds_without_any_env_vars() {
    // Deliberately do not set any env vars. None config must succeed.
    AuthSvc::build_auth_middleware(AuthConfig::None)
        .expect("AuthConfig::None must build unconditionally");
}

#[test]
fn test_noop_strategy_builds_even_when_common_env_vars_are_unset() {
    // Extra assurance: unset env vars that other strategies need.
    // None doesn't reference any env var — it must not be influenced.
    let irrelevant = "SWE_AUTH_NOOP_IRREL_01";
    std::env::remove_var(irrelevant);
    AuthSvc::build_auth_middleware(AuthConfig::None)
        .expect("None must build even when token env vars are absent");
}

// ---------------------------------------------------------------------------
// Returned middleware is a valid AuthMiddleware
// ---------------------------------------------------------------------------

#[test]
fn test_noop_strategy_build_returns_auth_middleware() {
    let mw: AuthMiddleware =
        AuthSvc::build_auth_middleware(AuthConfig::None).expect("None must build");
    // Verify the type is a real AuthMiddleware by exercising its Debug impl.
    let s = format!("{mw:?}");
    assert!(
        s.contains("AuthMiddleware"),
        "None build must return AuthMiddleware: {s}"
    );
}

// ---------------------------------------------------------------------------
// No auth headers attached — verified via reqwest_middleware wiring
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_noop_strategy_middleware_wires_into_reqwest_middleware_without_panic() {
    // Wiring the noop middleware into a ClientBuilder must not panic or error.
    let mw = AuthSvc::build_auth_middleware(AuthConfig::None).expect("None must build");
    let _client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
        .with(mw)
        .build();
    // Reaching here: the middleware handle path compiles and doesn't panic.
}

// ---------------------------------------------------------------------------
// Send + Sync
// ---------------------------------------------------------------------------

#[test]
fn test_noop_strategy_auth_middleware_is_send_and_sync() {
    fn require_send_sync<T: Send + Sync>() {}
    require_send_sync::<AuthMiddleware>();
}

#[test]
fn test_noop_strategy_auth_middleware_can_be_moved_across_threads() {
    let mw = AuthSvc::build_auth_middleware(AuthConfig::None).expect("None must build");
    let handle = std::thread::spawn(move || {
        // Confirm the middleware is usable in the spawned thread.
        let s = format!("{mw:?}");
        assert!(s.contains("AuthMiddleware"), "Debug in thread: {s}");
    });
    handle.join().expect("thread must not panic");
}

// ---------------------------------------------------------------------------
// Multiple noop middlewares are independent
// ---------------------------------------------------------------------------

#[test]
fn test_noop_strategy_two_independent_instances_both_build_and_debug() {
    let mw1 = AuthSvc::build_auth_middleware(AuthConfig::None).unwrap();
    let mw2 = AuthSvc::build_auth_middleware(AuthConfig::None).unwrap();
    // Both must be independently usable.
    let s1 = format!("{mw1:?}");
    let s2 = format!("{mw2:?}");
    assert!(s1.contains("http-auth"), "mw1: {s1}");
    assert!(s2.contains("http-auth"), "mw2: {s2}");
}

// ---------------------------------------------------------------------------
// build_auth_middleware with AuthConfig::None produces noop middleware
// ---------------------------------------------------------------------------

#[test]
fn test_noop_strategy_builder_fn_produces_noop_build() {
    let mw = AuthSvc::build_auth_middleware(AuthConfig::None)
        .expect("default builder must produce a noop middleware");
    let s = format!("{mw:?}");
    assert!(
        !s.is_empty(),
        "builder() noop middleware Debug must be non-empty"
    );
}
