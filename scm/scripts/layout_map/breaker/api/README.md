# edge-transport-http-egress-breaker-api

The circuit-breaker middleware contract: the `BreakerConfig` schema (TOML, defaults, validation
request), `BreakerError`, the request/response DTOs, and the `Processor`, `Validator`,
`BreakerMetrics`, `BreakerLayer` and `BreakerFactory` traits.

Depend on this package to program against a breaker. The standard implementation is
`edge-transport-http-egress-breaker-std` (`StdBreakerFactory`).

```bash
cargo test -p edge-transport-http-egress-breaker-api
```
