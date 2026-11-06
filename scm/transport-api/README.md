# edge-transport-http-egress-transport-api

The HTTP egress transport contract: the `HttpEgress`, `HttpStream` and `Validator` traits, the
request/response DTOs and value types (`HttpRequest`, `HttpResponse`, `HttpConfig`, builders,
streams, `WsMessage`, `SseEvent`), the errors, and the `TransportFactory` that builds an outbound
transport.

Depend on this package to program against the transport. The standard reqwest-backed implementation
is `edge-transport-http-egress-transport-std` (`StdTransportFactory`).

```bash
cargo test -p edge-transport-http-egress-transport-api
```
