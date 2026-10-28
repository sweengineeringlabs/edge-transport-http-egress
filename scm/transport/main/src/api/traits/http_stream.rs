//! HTTP streaming port — SSE consumption and WebSocket connections.

use super::HttpEgressFuture;
use crate::api::dto::{
    ConnectWebsocketRequest, ConnectWebsocketResponse, SubscribeSseRequest, SubscribeSseResponse,
};

/// Makes HTTP transport-level streaming connections to external services.
///
/// # SSE (Server-Sent Events)
/// Opens an HTTP connection and returns a lazy stream of
/// [`SseEvent`](crate::api::types::sse::SseEvent) frames parsed from
/// the `text/event-stream` response body.
///
/// # WebSocket
/// Completes the WebSocket handshake and returns a full-duplex
/// [`WsChannel`](crate::api::WsChannel). The caller may send and
/// receive frames concurrently; the connection stays open until the channel
/// is dropped.
pub trait HttpStream: Send + Sync {
    /// Subscribe to an SSE feed at `request.url`.
    ///
    /// Returns a lazy stream that yields [`SseEvent`](crate::api::types::sse::SseEvent)
    /// frames as they arrive from the remote service.
    fn subscribe_sse<'life0, 'async_trait>(
        &'life0 self,
        request: SubscribeSseRequest,
    ) -> HttpEgressFuture<'async_trait, SubscribeSseResponse>
    where
        'life0: 'async_trait,
        Self: Sync + 'async_trait;

    /// Open a WebSocket connection to `request.url`.
    ///
    /// Returns a [`WsChannel`](crate::api::WsChannel) after the
    /// handshake completes.
    fn connect_websocket<'life0, 'async_trait>(
        &'life0 self,
        request: ConnectWebsocketRequest,
    ) -> HttpEgressFuture<'async_trait, ConnectWebsocketResponse>
    where
        'life0: 'async_trait,
        Self: Sync + 'async_trait;
}
