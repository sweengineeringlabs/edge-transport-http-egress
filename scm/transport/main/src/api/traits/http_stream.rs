//! HTTP streaming port — SSE consumption and WebSocket connections.

use async_trait::async_trait;

use crate::api::dto::{
    ConnectWebsocketRequest, ConnectWebsocketResponse, SubscribeSseRequest, SubscribeSseResponse,
};
use crate::api::error::HttpEgressError;

/// Makes HTTP transport-level streaming connections to external services.
///
/// # SSE (Server-Sent Events)
/// Opens an HTTP connection and returns a lazy stream of
/// [`SseEvent`](crate::api::types::sse::SseEvent) frames parsed from
/// the `text/event-stream` response body.
///
/// # WebSocket
/// Completes the WebSocket handshake and returns a full-duplex
/// [`WsChannel`](crate::api::types::ws::WsChannel). The caller may send and
/// receive frames concurrently; the connection stays open until the channel
/// is dropped.
#[async_trait]
pub trait HttpStream: Send + Sync {
    /// Subscribe to an SSE feed at `request.url`.
    ///
    /// Returns a lazy stream that yields [`SseEvent`](crate::api::types::sse::SseEvent)
    /// frames as they arrive from the remote service.
    async fn subscribe_sse(
        &self,
        request: SubscribeSseRequest,
    ) -> Result<SubscribeSseResponse, HttpEgressError>;

    /// Open a WebSocket connection to `request.url`.
    ///
    /// Returns a [`WsChannel`](crate::api::types::ws::WsChannel) after the
    /// handshake completes.
    async fn connect_websocket(
        &self,
        request: ConnectWebsocketRequest,
    ) -> Result<ConnectWebsocketResponse, HttpEgressError>;
}
