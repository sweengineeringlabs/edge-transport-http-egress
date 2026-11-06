# Layout split verification (issue #30)

Comparison of the public surface and the tests before the split (tag `v0.9.0`) and after it (`v0.10.0`,
workspace 0.7.0). It was produced by reading the `pub use` lists of each package's `lib.rs` and counting
test functions with `git grep`; it is a name-level comparison, not a signature-level one.

## How to read the removed names

- `Processor`, `Validator` and the other contract traits, DTOs, errors and config types keep their names and
  are exported by the `-api` crate.
- The names listed as removed are concrete implementation types or static factories. Each is replaced by the
  factory trait in the `-api` package and the single `Std*Factory` in the `-std` package (see the `-api`
  changelogs for the call-by-call migration). The layer types (`MiddlewareHttpCache`, `RetryLayer`,
  `CassetteLayer`, `BreakerLayerBreakerMetrics`, ...) become contract traits or private types, and a factory
  returns them behind an `Arc` or a `Box`.
- `CassetteLayerBuilder` has no replacement type: `CassetteFactory::build_cassette_layer(config, name)` covers
  its three methods, and the state it could be left in (no cassette name) can no longer be expressed.
- `resilient`: the contract names it re-exported are exported by `resilient-api`; `DefaultResilientLayerComposer`
  is renamed `StdResilientLayerComposer`.

## Per package

### breaker -> breaker-api, breaker-std
- public names at v0.9.0: 22; still exported: 17
- removed from the public surface (5): BreakerLayerBreakerMetrics, BreakerMetricsFactory, HttpBreakerSvcProcessor, ProcessorFactory, ValidatorFactory
- new public names (3): BreakerFactory, BreakerLayer, StdBreakerFactory
- test functions (v0.9.0 -> now): 132 -> 141

### rate -> rate-api, rate-std
- public names at v0.9.0: 15; still exported: 10
- removed from the public surface (5): HttpRateSvcProcessor, ProcessorFactory, RateLayerRateMetrics, RateMetricsFactory, ValidatorFactory
- new public names (3): RateFactory, RateLayer, StdRateFactory
- test functions (v0.9.0 -> now): 124 -> 131

### cache -> cache-api, cache-std
- public names at v0.9.0: 15; still exported: 10
- removed from the public surface (5): HttpCacheSvcFactory, HttpCacheSvcProcessor, MiddlewareHttpCache, ProcessorSvcFactory, ValidatorSvcFactory
- new public names (3): CacheFactory, CacheLayer, StdCacheFactory
- test functions (v0.9.0 -> now): 193 -> 199

### retry -> retry-api, retry-std
- public names at v0.9.0: 15; still exported: 12
- removed from the public surface (3): HttpRetrySvc, ProcessorFactory, ValidatorFactory
- new public names (2): RetryFactory, StdRetryFactory
- test functions (v0.9.0 -> now): 143 -> 158

### cassette -> cassette-api, cassette-std
- public names at v0.9.0: 17; still exported: 12
- removed from the public surface (5): CassetteLayerBuilder, HttpCassetteFactory, HttpCassetteSvc, ProcessorFactory, ValidatorFactory
- new public names (2): CassetteFactory, StdCassetteFactory
- test functions (v0.9.0 -> now): 172 -> 171

### transport -> transport-api, transport-std
- public names at v0.9.0: 47; still exported: 45
- removed from the public surface (2): HttpTransportSvc, ValidatorSvcFactory
- new public names (2): StdTransportFactory, TransportFactory
- test functions (v0.9.0 -> now): 262 -> 276

### resilient-port -> resilient-api
- public names at v0.9.0: 5; still exported: 5
- removed from the public surface (0): none
- new public names (0): none
- test functions (v0.9.0 -> now): 9 -> 11

### resilient -> resilient-std
- public names at v0.9.0: 6; still exported: 0
- removed from the public surface (6): CassetteConfig, DefaultResilientLayerComposer, ResilientConfigRequest, ResilientDefaultsRequest, ResilientError, ResilientLayerComposer
- new public names (1): StdResilientLayerComposer
- test functions (v0.9.0 -> now): 3 -> 9

## Intentionally unchanged

- Config application names (configbuilder uses them as a config search directory) and runtime error-message
  prefixes keep the pre-split package names.
- `HttpEgress::send` still turns some response statuses into errors (#46); the split does not change it.

## Limits of this check

- Builder and wither options: every public option name appears in at least one test or example, but this was
  checked by name, not by proving per option that it reaches the transport or layer.
- The cassette test count goes from 172 to 171 because two tests of the removed `CassetteLayerBuilder` states
  were replaced by factory tests.
