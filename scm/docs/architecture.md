# Architecture — edge-transport-http-egress

Eight independent middleware crates compose into a `reqwest-middleware` chain: `auth → retry → rate → breaker → cache → cassette → tls`. Each is opt-in; policy lives in TOML.

---

## Sequence

> A domain handler sends an outbound HTTP call through the middleware stack; each layer can short-circuit or mutate before the wire call.

```mermaid
sequenceDiagram
    participant Handler
    participant HttpEgress
    participant AuthMiddleware
    participant RetryMiddleware
    participant RateMiddleware
    participant BreakerMiddleware
    participant CacheMiddleware
    participant reqwest

    Handler->>HttpEgress: send(HttpRequest)
    HttpEgress->>AuthMiddleware: add Authorization header
    AuthMiddleware->>RetryMiddleware: forward
    RetryMiddleware->>RateMiddleware: forward (retry budget managed)
    RateMiddleware->>BreakerMiddleware: forward (rate check passed)
    BreakerMiddleware->>CacheMiddleware: forward (circuit closed)
    CacheMiddleware-->>Handler: cached Response (cache hit)
    CacheMiddleware->>reqwest: execute (cache miss)
    reqwest-->>CacheMiddleware: HTTP response
    CacheMiddleware->>CacheMiddleware: store in cache
    CacheMiddleware-->>Handler: Response
```

## Data Flow

> An `HttpRequest` flows left-to-right through the middleware chain; each layer may terminate early or pass through; the final response flows back.

```mermaid
flowchart LR
    A["HttpRequest\n───────────\nmethod, url\nheaders, body"] --> B["edge-transport-http-egress-auth\nBearer / OAuth2 token\ninjected into headers"]
    B --> C["edge-transport-http-egress-retry\nmax_attempts, backoff\nretries on 5xx / timeout"]
    C --> D["edge-transport-http-egress-rate\ntokens-per-second\n429 if exhausted"]
    D --> E["edge-transport-http-egress-breaker\nfailure threshold\nopen → Err immediately"]
    E --> F["edge-transport-http-egress-cache\nLRU in-memory\ncache-hit → skip wire"]
    F --> G["edge-transport-http-egress-tls\nmTLS / custom CA\napplied to rustls"]
    G --> H["HTTP wire\n(reqwest)"]
    H --> I["HttpResponse\n───────────\nstatus, headers, body"]
    I --> J["Result<HttpResponse, EgressError>"]

    D -->|rate exceeded| X1["EgressError::RateLimit"]
    E -->|circuit open| X2["EgressError::CircuitOpen"]
```
