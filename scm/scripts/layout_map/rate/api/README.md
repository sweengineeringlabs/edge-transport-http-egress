# edge-transport-http-egress-rate-api

The rate-limit middleware contract: the `RateConfig` schema (TOML, defaults, validation request),
`RateError`, the request/response DTOs, and the `Processor`, `Validator`, `RateMetrics`,
`RateLayer` and `RateFactory` traits.

Depend on this package to program against a rate limiter. The standard implementation is
`edge-transport-http-egress-rate-std` (`StdRateFactory`).

```bash
cargo test -p edge-transport-http-egress-rate-api
```
