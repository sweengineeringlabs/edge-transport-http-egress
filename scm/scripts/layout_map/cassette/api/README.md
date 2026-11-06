# edge-transport-http-egress-cassette-api

The record/replay middleware contract: the `CassetteConfig` schema (TOML, defaults, builder),
`CassetteError`, the request/response DTOs, and the `Processor`, `Validator`, `HttpCassette`,
`CassetteLayer` and `CassetteFactory` traits.

Depend on this package to program against the cassette. The standard implementation is
`edge-transport-http-egress-cassette-std` (`StdCassetteFactory`).

```bash
cargo test -p edge-transport-http-egress-cassette-api
```
