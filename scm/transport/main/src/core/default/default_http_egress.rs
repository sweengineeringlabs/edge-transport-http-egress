use std::collections::HashMap;

use async_trait::async_trait;
use bytes::Bytes;
use futures::StreamExt as _;
use reqwest_middleware::ClientWithMiddleware;

#[cfg(feature = "websocket")]
use crate::api::WsMessage;
use crate::api::{
    ConfigRequest, ConfigResponse, ConnectWebsocketRequest, ConnectWebsocketResponse,
    HealthCheckRequest, HttpAuth, HttpBody, HttpByteStream, HttpConfig, HttpEgress,
    HttpEgressError, HttpEgressResult, HttpRequest, HttpResponse, HttpStream, HttpStreamResponse,
    SseEvent, SseStream, SubscribeSseRequest, SubscribeSseResponse, WsChannel,
};

pub(crate) struct DefaultHttpEgress {
    client: ClientWithMiddleware,
    config: HttpConfig,
}

impl DefaultHttpEgress {
    pub(crate) fn new(client: ClientWithMiddleware, config: HttpConfig) -> Self {
        Self { client, config }
    }

    /// Build a reqwest-middleware request builder from an [`HttpRequest`].
    ///
    /// Shared between [`send`](Self::send) and [`send_stream`](Self::send_stream)
    /// so request construction isn't duplicated.
    fn build_request_builder(
        &self,
        request: HttpRequest,
    ) -> HttpEgressResult<reqwest_middleware::RequestBuilder> {
        let url = self.resolve_url(&request.url);
        let method = reqwest::Method::from_bytes(request.method.to_string().as_bytes())
            .map_err(|e| HttpEgressError::InvalidRequest(e.to_string()))?;

        let mut builder = self.client.request(method, &url);

        for (k, v) in &request.headers {
            builder = builder.header(k, v);
        }

        if !request.query.is_empty() {
            let pairs: Vec<(&str, &str)> = request
                .query
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect();
            builder = builder.query(&pairs);
        }

        if let Some(body) = request.body {
            builder = match body {
                HttpBody::Json(v) => builder.json(&v.into_inner()),
                HttpBody::Raw(b) => builder.body(b),
                HttpBody::Form(f) => {
                    let pairs: Vec<(String, String)> = f.into_iter().collect();
                    builder.form(&pairs)
                }
                HttpBody::Multipart(parts) => {
                    let mut form = reqwest::multipart::Form::new();
                    for part in parts {
                        let mut mp = reqwest::multipart::Part::bytes(part.data);
                        if let Some(filename) = part.filename {
                            mp = mp.file_name(filename);
                        }
                        if let Some(ct) = part.content_type {
                            mp = mp
                                .mime_str(&ct)
                                .map_err(|e| HttpEgressError::InvalidRequest(e.to_string()))?;
                        }
                        form = form.part(part.name, mp);
                    }
                    builder.multipart(form)
                }
            };
        }

        if let Some(timeout) = request.timeout {
            builder = builder.timeout(timeout);
        }

        if let Some(auth) = request.auth {
            builder = match auth {
                HttpAuth::None => builder,
                HttpAuth::Bearer { token } => builder.bearer_auth(token),
                HttpAuth::Basic { username, password } => {
                    builder.basic_auth(username, Some(password))
                }
                HttpAuth::ApiKey { header, key } => builder.header(header, key),
            };
        }

        Ok(builder)
    }

    fn resolve_url(&self, url: &str) -> String {
        match &self.config.base_url {
            Some(base) if !url.starts_with("http://") && !url.starts_with("https://") => {
                format!(
                    "{}/{}",
                    base.trim_end_matches('/'),
                    url.trim_start_matches('/')
                )
            }
            _ => url.to_string(),
        }
    }
}

#[async_trait]
impl HttpEgress for DefaultHttpEgress {
    async fn send(&self, request: HttpRequest) -> HttpEgressResult<HttpResponse> {
        let max_response_bytes = self.config.max_response_bytes;
        let builder = self.build_request_builder(request)?;
        let response = builder.send().await.map_err(|e| {
            if let reqwest_middleware::Error::Reqwest(ref re) = e {
                if re.is_timeout() {
                    return HttpEgressError::Timeout(e.to_string());
                }
            }
            HttpEgressError::ConnectionFailed(e.to_string())
        })?;

        // Early rejection on content-length hint (avoids buffering huge bodies).
        if let Some(max) = max_response_bytes {
            if let Some(len) = response.content_length() {
                if len as usize > max {
                    return Err(HttpEgressError::Internal(format!(
                        "response too large: content-length {len} bytes exceeds limit of {max} bytes"
                    )));
                }
            }
        }

        let status = response.status().as_u16();
        let headers: HashMap<String, String> = response
            .headers()
            .iter()
            .filter_map(|(k, v)| v.to_str().ok().map(|v| (k.to_string(), v.to_string())))
            .collect();
        let body_bytes = response
            .bytes()
            .await
            .map_err(|e| HttpEgressError::Internal(e.to_string()))?;

        if let Some(max) = max_response_bytes {
            if body_bytes.len() > max {
                return Err(HttpEgressError::Internal(format!(
                    "response too large: {} bytes exceeds limit of {} bytes",
                    body_bytes.len(),
                    max
                )));
            }
        }

        match status {
            401 => {
                return Err(HttpEgressError::Unauthorized(
                    "remote returned 401 Unauthorized".to_string(),
                ))
            }
            403 => {
                return Err(HttpEgressError::Forbidden(
                    "remote returned 403 Forbidden".to_string(),
                ))
            }
            404 => {
                return Err(HttpEgressError::NotFound(
                    "remote returned 404 Not Found".to_string(),
                ))
            }
            429 => {
                return Err(HttpEgressError::RateLimited(
                    "remote returned 429 Too Many Requests".to_string(),
                ))
            }
            502 => {
                return Err(HttpEgressError::BadGateway(
                    "remote returned 502 Bad Gateway".to_string(),
                ))
            }
            503 => {
                return Err(HttpEgressError::ServiceUnavailable(
                    "remote returned 503 Service Unavailable".to_string(),
                ))
            }
            _ => {}
        }

        Ok(HttpResponse {
            status,
            headers,
            body: body_bytes.to_vec(),
        })
    }

    async fn send_stream(&self, request: HttpRequest) -> HttpEgressResult<HttpStreamResponse> {
        let builder = self.build_request_builder(request)?;
        let response = builder.send().await.map_err(|e| {
            if let reqwest_middleware::Error::Reqwest(ref re) = e {
                if re.is_timeout() {
                    return HttpEgressError::Timeout(e.to_string());
                }
            }
            HttpEgressError::ConnectionFailed(e.to_string())
        })?;

        let status = response.status().as_u16();
        let headers: HashMap<String, String> = response
            .headers()
            .iter()
            .filter_map(|(k, v)| v.to_str().ok().map(|v| (k.to_string(), v.to_string())))
            .collect();

        let body = HttpByteStream::new(response.bytes_stream().map(|r| {
            r.map(|b: Bytes| b.to_vec())
                .map_err(|e| HttpEgressError::Internal(e.to_string()))
        }));

        Ok(HttpStreamResponse {
            status,
            headers,
            body,
        })
    }

    async fn health_check(&self, _request: HealthCheckRequest) -> HttpEgressResult<()> {
        let url = match &self.config.base_url {
            Some(u) => u.clone(),
            None => return Ok(()),
        };
        let resp = self.client.get(&url).send().await.map_err(|e| {
            if let reqwest_middleware::Error::Reqwest(ref re) = e {
                if re.is_timeout() {
                    return HttpEgressError::Timeout(e.to_string());
                }
            }
            HttpEgressError::ConnectionFailed(e.to_string())
        })?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(HttpEgressError::Internal(format!(
                "health check failed: HTTP {}",
                resp.status().as_u16()
            )))
        }
    }

    fn config(&self, _request: ConfigRequest) -> HttpEgressResult<ConfigResponse> {
        Ok(ConfigResponse {
            config: self.config.clone(),
        })
    }
}

impl DefaultHttpEgress {
    /// Parse a raw byte stream from an SSE response into a stream of [`SseEvent`]s.
    ///
    /// Spawns a background task that reads bytes, buffers and parses the
    /// `text/event-stream` format, and sends events through an mpsc channel.
    fn parse_sse_bytes(
        bytes_stream: impl futures::Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Send + 'static,
    ) -> impl futures::Stream<Item = Result<SseEvent, HttpEgressError>> + Send {
        use futures::StreamExt as _;

        let (tx, rx) = futures::channel::mpsc::unbounded::<Result<SseEvent, HttpEgressError>>();

        tokio::spawn(async move {
            futures::pin_mut!(bytes_stream);
            let mut buf = String::new();

            while let Some(chunk) = bytes_stream.next().await {
                match chunk {
                    Ok(bytes) => {
                        buf.push_str(&String::from_utf8_lossy(&bytes));
                        while let Some(pos) = buf.find("\n\n") {
                            let block = buf[..pos].to_string();
                            buf = buf[pos + 2..].to_string();
                            if let Some(ev) = DefaultHttpEgress::parse_sse_block(&block) {
                                if tx.unbounded_send(Ok(ev)).is_err() {
                                    return;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        let _ = tx.unbounded_send(Err(HttpEgressError::Internal(e.to_string())));
                        return;
                    }
                }
            }
            // Flush any trailing partial event.
            if !buf.trim().is_empty() {
                if let Some(ev) = DefaultHttpEgress::parse_sse_block(&buf) {
                    let _ = tx.unbounded_send(Ok(ev));
                }
            }
        });

        rx
    }

    /// Parse a single SSE block (delimited by `\n\n`) into an [`SseEvent`].
    fn parse_sse_block(block: &str) -> Option<SseEvent> {
        let mut data = String::new();
        let mut event: Option<String> = None;
        let mut id: Option<String> = None;

        for line in block.lines() {
            if line.starts_with(':') {
                continue; // comment
            }
            let (field, value) = if let Some(pos) = line.find(':') {
                let f = &line[..pos];
                let v = line[pos + 1..].trim_start_matches(' ');
                (f, v)
            } else {
                (line, "")
            };

            match field {
                "data" => {
                    if !data.is_empty() {
                        data.push('\n');
                    }
                    data.push_str(value);
                }
                "event" => event = Some(value.to_string()),
                "id" => id = Some(value.to_string()),
                _ => {}
            }
        }

        if data.is_empty() {
            None
        } else {
            Some(SseEvent { event, data, id })
        }
    }

    /// Connect to a WebSocket server and return a [`WsChannel`].
    ///
    /// Requires the `websocket` feature.
    async fn connect_ws(url: String) -> HttpEgressResult<WsChannel> {
        #[cfg(feature = "websocket")]
        {
            use futures::SinkExt as _;
            use futures::StreamExt as _;
            use tokio::sync::mpsc;
            use tokio_tungstenite::tungstenite::Message as TungMsg;

            let (ws_stream, _) = tokio_tungstenite::connect_async(&url)
                .await
                .map_err(|e| HttpEgressError::ConnectionFailed(e.to_string()))?;

            let (mut ws_sink, ws_read) = ws_stream.split();

            let (out_tx, mut out_rx) = mpsc::unbounded_channel::<WsMessage>();

            // Bridge outgoing mpsc → WebSocket sink.
            tokio::spawn(async move {
                while let Some(msg) = out_rx.recv().await {
                    let tung_msg = if msg.binary {
                        TungMsg::Binary(msg.data.to_vec().into())
                    } else {
                        TungMsg::Text(String::from_utf8_lossy(&msg.data).into_owned().into())
                    };
                    if ws_sink.send(tung_msg).await.is_err() {
                        break;
                    }
                }
            });

            let incoming = crate::api::WsReceiver::new(ws_read.filter_map(|item| async move {
                match item {
                    Ok(TungMsg::Text(t)) => Some(Ok(WsMessage::text(t.as_str()))),
                    Ok(TungMsg::Binary(b)) => Some(Ok(WsMessage::binary(b.to_vec()))),
                    Ok(TungMsg::Close(_)) => None,
                    Ok(_) => None,
                    Err(e) => Some(Err(HttpEgressError::ConnectionFailed(e.to_string()))),
                }
            }));

            Ok(WsChannel {
                sender: crate::api::WsSender::new(out_tx),
                receiver: incoming,
            })
        }

        #[cfg(not(feature = "websocket"))]
        {
            let _ = url;
            Err(HttpEgressError::Internal(
                "WebSocket support requires the 'websocket' feature flag".into(),
            ))
        }
    }
}

#[async_trait]
impl HttpStream for DefaultHttpEgress {
    async fn subscribe_sse(
        &self,
        request: SubscribeSseRequest,
    ) -> HttpEgressResult<SubscribeSseResponse> {
        let client = self.client.clone();
        let response = client
            .get(&request.url)
            .header("Accept", "text/event-stream")
            .header("Cache-Control", "no-cache")
            .send()
            .await
            .map_err(|e| HttpEgressError::ConnectionFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(HttpEgressError::Internal(format!(
                "SSE feed returned HTTP {}",
                response.status().as_u16()
            )));
        }

        let bytes_stream = response.bytes_stream();
        let sse_stream = DefaultHttpEgress::parse_sse_bytes(bytes_stream);
        Ok(SubscribeSseResponse {
            stream: SseStream::new(sse_stream),
        })
    }

    async fn connect_websocket(
        &self,
        request: ConnectWebsocketRequest,
    ) -> HttpEgressResult<ConnectWebsocketResponse> {
        let channel = DefaultHttpEgress::connect_ws(request.url).await?;
        Ok(ConnectWebsocketResponse { channel })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest_middleware::ClientBuilder;

    fn client() -> ClientWithMiddleware {
        ClientBuilder::new(reqwest::Client::new()).build()
    }

    fn make_outbound() -> DefaultHttpEgress {
        DefaultHttpEgress::new(
            client(),
            HttpConfig {
                base_url: Some("http://localhost".into()),
                ..HttpConfig::default()
            },
        )
    }

    #[test]
    fn test_new_creates_outbound_with_base_url() {
        let out = make_outbound();
        assert_eq!(out.config.base_url.as_deref(), Some("http://localhost"));
    }

    #[test]
    fn test_parse_sse_block_parses_data_only_event() {
        let ev = DefaultHttpEgress::parse_sse_block("data: hello");
        assert!(ev.is_some());
        assert_eq!(ev.unwrap().data, "hello");
    }

    #[test]
    fn test_parse_sse_block_parses_event_type_and_id() {
        let block = "event: update\ndata: {}\nid: 42";
        let ev = DefaultHttpEgress::parse_sse_block(block).unwrap();
        assert_eq!(ev.event.as_deref(), Some("update"));
        assert_eq!(ev.data, "{}");
        assert_eq!(ev.id.as_deref(), Some("42"));
    }

    #[test]
    fn test_parse_sse_block_ignores_comment_lines() {
        let ev = DefaultHttpEgress::parse_sse_block(": comment\ndata: real");
        assert_eq!(ev.unwrap().data, "real");
    }

    #[test]
    fn test_parse_sse_block_returns_none_for_empty_block() {
        assert!(DefaultHttpEgress::parse_sse_block("").is_none());
        assert!(DefaultHttpEgress::parse_sse_block("   ").is_none());
    }

    #[test]
    fn test_parse_sse_block_concatenates_multiple_data_lines() {
        let ev = DefaultHttpEgress::parse_sse_block("data: line1\ndata: line2").unwrap();
        assert_eq!(ev.data, "line1\nline2");
    }

    #[tokio::test]
    async fn test_parse_sse_bytes_emits_events_from_double_newline_separated_blocks() {
        use futures::stream;
        let chunks: Vec<Result<bytes::Bytes, reqwest::Error>> =
            vec![Ok(bytes::Bytes::from("data: hello\n\ndata: world\n\n"))];
        let byte_stream = stream::iter(chunks);
        let events: Vec<_> = DefaultHttpEgress::parse_sse_bytes(byte_stream)
            .collect()
            .await;
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].as_ref().unwrap().data, "hello");
        assert_eq!(events[1].as_ref().unwrap().data, "world");
    }

    #[tokio::test]
    async fn test_subscribe_sse_returns_err_when_no_server() {
        let out = make_outbound();
        let result = out
            .subscribe_sse(SubscribeSseRequest {
                url: "http://127.0.0.1:1".to_string(),
            })
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_connect_websocket_returns_err_when_no_feature() {
        let out = make_outbound();
        let result = out
            .connect_websocket(ConnectWebsocketRequest {
                url: "ws://127.0.0.1:1".to_string(),
            })
            .await;
        // Without websocket feature: returns an Internal error.
        // With websocket feature: returns a ConnectionFailed error.
        assert!(result.is_err());
    }

    #[test]
    fn test_http_stream_outbound_is_implemented_by_default_http_egress() {
        fn _assert(_: &dyn HttpStream) {}
        let out = make_outbound();
        _assert(&out);
    }
}
