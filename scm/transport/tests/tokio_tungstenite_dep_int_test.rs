//! Dependency coverage test for `tokio-tungstenite`.
//! @covers: tokio-tungstenite
//!
//! Rule 95: `tokio-tungstenite` is an optional dependency used in `src/` for
//! WebSocket support and must have integration coverage with an explicit
//! `use tokio_tungstenite::...` import when the `websocket` feature is enabled.
//!
//! Note: `tokio-tungstenite` is gated behind the `websocket` feature flag.
//! This test file is compiled unconditionally so we test the import in a way
//! that is always valid.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use tokio_tungstenite::tungstenite::protocol::Role;

/// @covers: tokio-tungstenite
/// Verifies that the `tokio_tungstenite` crate is accessible and that
/// `tungstenite::protocol::Role` variants are constructible — this is the
/// core type used when upgrading a TCP stream to a WebSocket.
#[test]
fn transport_struct_dep_tokio_tungstenite_role_variants_int_test() {
    // Role::Client and Role::Server are the two variants used when creating
    // a WebSocket handshake from an already-established TCP stream.
    let client_dbg = format!("{:?}", Role::Client);
    let server_dbg = format!("{:?}", Role::Server);
    // Pin each variant's Debug rendering to its known value — this catches an
    // accidental swap or collapse of the two variants, which a self-comparison
    // could not.
    assert_eq!(
        client_dbg, "Client",
        "Role::Client must render as \"Client\""
    );
    assert_eq!(
        server_dbg, "Server",
        "Role::Server must render as \"Server\""
    );
}

/// @covers: tokio-tungstenite
/// Verifies the `tungstenite::Message` enum variants are accessible —
/// the transport layer wraps these as `WsMessage`.
#[test]
fn transport_struct_dep_tokio_tungstenite_message_text_int_test() {
    use tokio_tungstenite::tungstenite::Message;
    let msg = Message::Text("hello".into());
    let is_text = msg.is_text();
    assert!(is_text, "Message::Text must report is_text() == true");
}
