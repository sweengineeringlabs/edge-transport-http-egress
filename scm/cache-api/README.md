# edge-transport-http-egress-cache-api

The HTTP-cache middleware contract: the `CacheConfig` schema (TOML, defaults, validation request),
`CacheError`, the request/response DTOs, and the `Processor`, `Validator`, `HttpCache`,
`CacheLayer` and `CacheFactory` traits.

Depend on this package to program against the cache. The standard implementation is
`edge-transport-http-egress-cache-std` (`StdCacheFactory`).

```bash
cargo test -p edge-transport-http-egress-cache-api
```
