# edge-transport-http-egress-retry-api

The retry middleware contract: the `RetryConfig` schema and its builders (TOML, defaults,
validation), `RetryError`, the request/response DTOs, and the `Processor`, `Validator`,
`RetryLayer` and `RetryFactory` traits.

Depend on this package to program against retries. The standard implementation is
`edge-transport-http-egress-retry-std` (`StdRetryFactory`).

```bash
cargo test -p edge-transport-http-egress-retry-api
```
