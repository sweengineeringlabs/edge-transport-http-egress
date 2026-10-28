# HTTP Egress Package Layout Migration Map

Tracking: [edge-transport-http-egress#30](https://github.com/sweengineeringlabs/edge-transport-http-egress/issues/30)

This map records the current workspace surface and proposed package ownership before files move. It is based on the checked-out workspace manifests and source tree; downstream source searches were read-only. No package split is implemented by this document.

## Current Workspace Inventory

The workspace currently has eight members. The older #20 issue refers to seven and names `loadbalancer`; the current workspace has `resilient` and `resilient-port` instead.

| Current package | Public API traits | Current implementation and factories | Tests / examples | Workspace consumers |
|---|---|---|---|---|
| `edge-transport-http-egress-breaker` | `Processor`, `BreakerMetrics`, `Validator` | `core/breaker`, `core/default/http_breaker.rs`; SAF processor, validator, and breaker-metrics factories | 32 / 1 | `resilient` |
| `edge-transport-http-egress-cache` | `HttpCache`, `Processor`, `Validator` | `core/cache`, `core/cached`, `core/processor`; SAF HTTP-cache, processor, and validator factories | 34 / 1 | `resilient` |
| `edge-transport-http-egress-cassette` | `HttpCassette`, `Processor`, `Validator` | `core/cassette`, recording/replay, fixture store, body scrubber; SAF cassette, processor, and validator factories | 34 / 1 | `resilient`, `resilient-port` |
| `edge-transport-http-egress-rate` | `RateMetrics`, `Processor`, `Validator` | `core/token`, `core/rate`, `core/default/http_rate.rs`; SAF rate-metrics, processor, and validator factories | 29 / 1 | `resilient` |
| `edge-transport-http-egress-retry` | `Processor`, `Validator` | `core/policy`, `core/layer`, `core/http_retry_svc.rs`; SAF processor and validator factories | 29 / 1 | `resilient` |
| `edge-transport-http-egress-transport` | `HttpEgress`, `HttpStream`, `Validator` | `core/default`, `core/http`, `core/sse`, `core/ws`; SAF HTTP egress, stream, transport, and validator factories | 56 / 1 | None inside this workspace |
| `edge-transport-http-egress-resilient` | No `api/` contract directory; exposes composition types from `core/` | `core/preflight.rs`, `core/default_resilient_layer_composer.rs` | 1 / 0 | No workspace package consumes it |
| `edge-transport-http-egress-resilient-port` | `ResilientLayerComposer` port in `main/src/api/traits/resilient_layer_composer.rs` | Contract/error/type files in `main/src/api/`; no implementation package in this workspace | 0 / 0 | `resilient` |

The `resilient` dependency direction is the composition root: it depends on breaker, cache, cassette, rate, retry, and resilient-port. `resilient-port` also depends on cassette. The other middleware packages do not depend on each other in the workspace.

## Crate-Root Public Re-Exports

The current `main/src/lib.rs` re-exports the following API names. These names and their consumer-facing construction paths must be mapped to the new API packages; concrete implementation types are candidates to become private behind factories.

| Package | API re-exports |
|---|---|
| breaker | `Admission`, `AdmitRequest`, `AdmitResponse`, `BreakerConfig`, `BreakerError`, `BreakerLayerBreakerMetrics`, `BreakerMetrics`, `BreakerTransition`, `ConfigValidationRequest`, `DescribeRequest`, `DescribeResponse`, `FailureThresholdRequest`, `FailureThresholdResponse`, `HttpBreakerSvcProcessor`, `Outcome`, `Processor`, `RecordOutcomeRequest`, `RecordOutcomeResponse`, `Validator`; factories: `BreakerMetricsFactory`, `ProcessorFactory`, `ValidatorFactory` |
| cache | `CacheConfig`, `CacheError`, `ConfigValidationRequest`, `DescribeRequest`, `DescribeResponse`, `FallbackTtlRequest`, `FallbackTtlResponse`, `HttpCache`, `HttpCacheSvcProcessor`, `MiddlewareHttpCache`, `Processor`, `Validator`; factories: `HttpCacheSvcFactory`, `ProcessorSvcFactory`, `ValidatorSvcFactory` |
| cassette | `CassetteConfig`, `CassetteConfigBuilder`, `CassetteError`, `CassetteLayer`, `CassetteLayerBuilder`, `CassetteModeRequest`, `CassetteModeResponse`, `ConfigValidationRequest`, `DescribeRequest`, `DescribeResponse`, `HttpCassette`, `HttpCassetteSvc`, `Processor`, `Validator`; factories: `HttpCassetteFactory`, `ProcessorFactory`, `ValidatorFactory` |
| rate | `ConfigValidationRequest`, `DescribeRequest`, `DescribeResponse`, `HttpRateSvcProcessor`, `Processor`, `RateConfig`, `RateError`, `RateLayerRateMetrics`, `RateLimitRequest`, `RateLimitResponse`, `RateMetrics`, `Validator`; factories: `ProcessorFactory`, `RateMetricsFactory`, `ValidatorFactory` |
| retry | `ApplicationConfigBuilder`, `DecorateRequest`, `DecorateResponse`, `HttpRetrySvc`, `Processor`, `ProcessorRequest`, `ProcessorResponse`, `RetryConfig`, `RetryConfigBuilder`, `RetryError`, `RetryLayer`, `ValidationRequest`, `Validator`; factories: `ProcessorFactory`, `ValidatorFactory` |
| transport | `AlwaysValidConfig`, `ApplicationConfigBuilder`, `AssemblyFailure`, `ConfigRequest`, `ConfigResponse`, `ConnectWebsocketRequest`, `ConnectWebsocketResponse`, `FormPart`, `GetRequest`, `HealthCheckRequest`, `HttpAuth`, `HttpBody`, `HttpByteStream`, `HttpConfig`, `HttpConfigBuilder`, `HttpConfigValidator`, `HttpEgress`, `HttpEgressBuildError`, `HttpEgressError`, `HttpEgressObject`, `HttpEgressResult`, `HttpMethod`, `HttpRequest`, `HttpRequestBuilder`, `HttpResponse`, `HttpSecurityContext`, `HttpStream`, `HttpStreamResponse`, `HttpTransportSvc`, `JsonValue`, `MetricsHttpEgress`, `SseEvent`, `SseStream`, `SubscribeSseRequest`, `SubscribeSseResponse`, `ValidateRequest`, `ValidationError`, `Validator`, `ValidatorObject`, `WsChannel`, `WsMessage`, `WsReceiver`, `WsSender`; factory: `ValidatorSvcFactory`; also re-exports `edge_security_runtime::SecurityContext` |
| resilient | `DefaultResilientLayerComposer`; also re-exports `ResilientDefaultsRequest`, `ResilientConfigRequest`, `ResilientError`, and `ResilientLayerComposer` from resilient-port |
| resilient-port | `ResilientDefaultsRequest`, `ResilientConfigRequest`, `ResilientError`, `ResilientLayerComposer`; also re-exports `CassetteConfig` from cassette |

## Proposed Ownership

| Current package | Contract target | Standard implementation target | Provider target |
|---|---|---|---|
| breaker | `edge-transport-http-egress-breaker-api` (`edge_transport_http_egress_breaker_api`) | `edge-transport-http-egress-breaker-std` | None identified; do not create an empty SPI package |
| cache | `edge-transport-http-egress-cache-api` (`edge_transport_http_egress_cache_api`) | `edge-transport-http-egress-cache-std` | None identified; do not create an empty SPI package |
| cassette | `edge-transport-http-egress-cassette-api` (`edge_transport_http_egress_cassette_api`) | `edge-transport-http-egress-cassette-std` | None identified; classify storage backends during dependency review |
| rate | `edge-transport-http-egress-rate-api` (`edge_transport_http_egress_rate_api`) | `edge-transport-http-egress-rate-std` | None identified; do not create an empty SPI package |
| retry | `edge-transport-http-egress-retry-api` (`edge_transport_http_egress_retry_api`) | `edge-transport-http-egress-retry-std` | None identified; classify the retry policy crates during dependency review |
| transport | `edge-transport-http-egress-transport-api` (`edge_transport_http_egress_transport_api`) | `edge-transport-http-egress-transport-std` | None identified; reqwest and optional websocket support are current standard implementations |
| resilient | No separate contract currently exists | `edge-transport-http-egress-resilient-std` if its composition implementation remains | None identified |
| resilient-port | `edge-transport-http-egress-resilient-api` (`edge_transport_http_egress_resilient_api`) | None | None |

API packages own traits, request/response DTOs, errors, config contracts, and factory traits. Standard packages own reqwest/middleware, moka, filesystem, and Tokio implementations behind crate-private implementation types. Provider-specific packages are added only if the inventory identifies a genuinely separate provider implementation. The API packages must not depend on standard or provider implementations.

## Public Builder and Wither Surface

These methods are present in the current source and must retain their behavior through public API/factory contracts. The list is based on `pub fn` declarations in `main/src`, not inferred from private fields.

| Package | Public type / factory | Existing options and construction methods |
|---|---|---|
| breaker | `HttpBreaker` | `create_config_builder`, `build_breaker_layer(config)` |
| cache | `HttpCacheSvcProcessor` | `create_config_builder`, `build_cache_layer(config)` |
| rate | `HttpRateSvc` | `create_config_builder`, `build_rate_layer(config)` |
| cassette | `CassetteConfigBuilder` | `new`, `with_mode`, `with_cassette_dir`, `with_match_on`, `with_scrub_headers`, `with_scrub_body_paths`, `build_config` |
| cassette | `CassetteLayerBuilder` | `new`, `with_config`, `with_cassette_name`, `build_layer` |
| cassette | `HttpCassetteSvc` | `create_config_builder`, `build_cassette_layer` |
| retry | `RetryConfigBuilder` | `new`, `max_retries`, `initial_interval_ms`, `max_interval_ms`, `multiplier`, `jitter_factor`, `retryable_statuses`, `retryable_methods`, `build` |
| retry | `ApplicationConfigBuilder` / `HttpRetrySvc` | `with_config_dir`, `build_loader`, `create_config_builder` |
| transport | `HttpConfigBuilder` | `new`, `with_base_url`, `with_timeout_secs`, `with_connect_timeout_secs`, `with_header`, `with_user_agent`, `build` |
| transport | `HttpRequestBuilder` | `new(method, url)`, `with_header`, `with_query`, `with_timeout`, `with_auth`, `build` |
| transport | `HttpConfig` | `with_base_url`, `with_header`, `with_timeout` |
| transport | `HttpRequest` | `with_header`, `with_query`, `with_json`, `with_body`, `with_form`, `with_timeout`, `with_auth` |
| transport | `ApplicationConfigBuilder` / `HttpTransportSvc` | `with_name`, `with_version`, `build_loader`, `create_config_builder` |

`HttpConfigBuilder` had private fields for retry, redirect, and response-size settings without corresponding public setters; those dead fields have been removed. The remaining builder methods and direct `HttpConfig` field/serde paths must retain their behavior through the package move. Configbuilder loading paths still require source-to-test tracing before T1 is complete.

## Configuration Schema Ownership

The following public configuration schemas and fields are declared under the current `api/` trees. The schema, serde behavior, and consumer-facing constructors belong in the matching `-api` package. Parsing, defaults, validation, configbuilder integration, and conversion into middleware/client settings belong in `-std`; preserve the exact current semantics and serde attributes when moving them.

| Current schema | Public fields and current serde behavior | Target ownership |
|---|---|---|
| `breaker::BreakerConfig` | `failure_threshold: u32`, `half_open_after_seconds: u64`, `reset_after_successes: u32`, `failure_statuses: Vec<u16>`; `deny_unknown_fields` | Schema in breaker-api; Default/from_config/configbuilder and validator in breaker-std |
| `cache::CacheConfig` | `default_ttl_seconds: u64`, `max_entries: u64`, `respect_cache_control: bool`, `cache_private: bool`; `deny_unknown_fields` | Schema in cache-api; Default/from_config/configbuilder and validator in cache-std |
| `cassette::CassetteConfig` | `mode: String`, `cassette_dir: String`, `match_on: Vec<String>`, `scrub_headers: Vec<String>`, `scrub_body_paths: Vec<String>`; `deny_unknown_fields` | Schema in cassette-api; defaults/from_config/configbuilder and validator in cassette-std; keep cassette data contracts in cassette-api only if they are provider-independent |
| `rate::RateConfig` | `tokens_per_second: u32`, `burst_capacity: u32`, `per_host: bool`; `deny_unknown_fields` | Schema in rate-api; Default/from_config/configbuilder and validator in rate-std |
| `retry::RetryConfig` | `max_retries: u32`, `initial_interval_ms: u64`, `max_interval_ms: u64`, `multiplier: f64`, `jitter_factor: f64`, `retryable_statuses: Vec<u16>`, `retryable_methods: Vec<String>`; `deny_unknown_fields`, `jitter_factor` has serde default | Schema in retry-api; defaults/from_config/validation and retry-policy conversion in retry-std |
| `transport::HttpConfig` | `base_url: Option<String>`, `timeout_secs: u64`, `connect_timeout_secs: u64`, `default_headers: HashMap<String,String>`, `follow_redirects: bool`, `max_redirects: u32`, `user_agent: Option<String>`, `max_response_bytes: Option<usize>`; struct-level serde default and deny-unknown-fields, default for headers and response cap | Schema in transport-api; defaults/config loading/validation and reqwest builder application in transport-std |

This is a field inventory, not yet a behavior proof. Before implementation, tests must establish the current default and each field's observable effect, including `HttpConfigBuilder` fields with no public setter and the separate TOML/configbuilder loader path. The matching tests must move with each contract or implementation behavior they verify.

### Current Config-to-Behavior Trace

Source tracing confirms every surviving schema field is read by a default implementation, validator, parser, or observable response. These implementation paths and existing test targets must move together. The list is a starting verification map; it does not assert the tests pass on the migration output.

| Schema | Field behavior in current source | Existing focused test targets |
|---|---|---|
| breaker | `failure_threshold` selects the trip count; `failure_statuses` classifies responses; `half_open_after_seconds` maps to the shared policy cooldown; `reset_after_successes` maps to probe successes. | `breaker_config_int_test.rs`, `breaker_layer_core_int_test.rs`, `breaker_state_int_test.rs`, `default_http_breaker_int_test.rs` |
| cache | `default_ttl_seconds` is the fallback; `max_entries` sets Moka capacity; `respect_cache_control` controls Cache-Control handling; `cache_private` gates private responses. | `cache_config_int_test.rs`, `cache_layer_core_int_test.rs`, `ttl_decision_int_test.rs`, `middleware_http_cache_int_test.rs` |
| cassette | `mode` selects record/replay/auto/disabled; `cassette_dir` selects storage; `match_on` builds the request key; `scrub_headers` and `scrub_body_paths` redact persisted interactions. | `cassette_config_int_test.rs`, `cassette_layer_core_int_test.rs`, `body_scrubber_int_test.rs`, `fixture_store_int_test.rs` |
| rate | `tokens_per_second` controls refill; `burst_capacity` sets initial/capped tokens; `per_host` selects host-keyed versus shared buckets. | `rate_config_int_test.rs`, `rate_layer_core_int_test.rs`, `token_bucket_int_test.rs` |
| retry | `max_retries` bounds attempts; intervals/multiplier/jitter drive shared backoff; status/method lists gate retries. | `retry_config_int_test.rs`, `retry_config_builder_scenario_int_test.rs`, `retry_layer_core_int_test.rs`, `default_http_retry_int_test.rs` |
| transport | `base_url` resolves relative URLs/health checks; timeouts, redirect behavior, user-agent and default headers configure reqwest; response cap is checked before and after buffering. The ineffective `max_retries` field is removed as documented above. | `http_config_int_test.rs`, `http_config_builder_int_test.rs`, `http_config_validator_int_test.rs`, `http_egress_factory_int_test.rs`, `http_egress_int_test.rs` |

The repo uses both explicit `from_config` methods and configbuilder section loading, so the migration must preserve section names, defaults, and validation at those entry points as well as direct serde parsing. Source references show each current business field has a behavior; observable coverage for every supported option and combination still needs a post-move audit.

### Confirmed Transport Field Behavior and Cleanup

- `timeout_secs` and `connect_timeout_secs` configure reqwest and are validated as nonzero.
- `user_agent`, `follow_redirects`, `max_redirects`, and `default_headers` are applied by the single `configure_http_builder` mapping.
- `max_response_bytes` is enforced against both the content-length hint and the buffered response body.
- `base_url` is used by request URL resolution and health checks.
- `max_retries` was present in `HttpConfig` and copied by `HttpConfigBuilder::build`, but no transport code read it. Retry composition moved to the resilient composition root. The field and dead private builder state were removed; `HttpConfig` now denies unknown fields so the removed no-op is rejected instead of silently accepted. A regression test covers the removed setting.
- `HttpConfigBuilder` had private retry, redirect, and response-size fields without public setters. They only read their `None` defaults, so the dead builder state was removed; remaining settings are assigned explicit `HttpConfig` defaults by `build`.
- The same timeout validation was independently implemented in both a private reference validator and `ValidatableHttpConfig`. The private duplicate was removed on the working branch; `ValidatableHttpConfig` now owns the single validation implementation and the SAF validation path delegates to it. Regression tests for both zero-timeout cases remain in the source.

Other public configuration and construction operations found under the current `core/` or `saf/` trees also need explicit API/factory ownership; they are not covered by the builder table above:

| Package | Current operations |
|---|---|
| breaker | `BreakerConfig::from_config`; `HttpBreaker::create_config_builder`, `HttpBreaker::build_breaker_layer`; SAF `ProcessorFactory::create`, `ValidatorFactory::create`, `BreakerMetricsFactory::from_layer` |
| cache | `CacheConfig::from_config`; `HttpCacheSvcProcessor::create_config_builder`, `build_cache_layer`; SAF `ProcessorSvcFactory::create`, `ValidatorSvcFactory::create`, `HttpCacheSvcFactory::from_layer` |
| cassette | `CassetteConfig::from_config`, `disabled`, `swe_default`; `HttpCassetteSvc::create_config_builder`, `build_cassette_layer`; SAF processor, validator, and HTTP-cassette factories |
| rate | `RateConfig::from_config`; `HttpRateSvc::create_config_builder`, `build_rate_layer`; SAF processor, validator, and rate-metrics factories |
| retry | `RetryConfig::from_config`, `validate`; `ApplicationConfigBuilder::{name, version, with_config_dir, build_loader}`; `HttpRetrySvc` factories; `RetryConfigBuilder` operations above |
| resilient | `DefaultResilientLayerComposer::preflight`; currently re-exported directly from `core/` |
| transport | `HttpConfig::{default_max_response_bytes, with_base_url, with_header, with_timeout}`; `HttpRequest` method constructors (`get`, `post`, `put`, `delete`, `patch`, `head`, `options`), `header`, and withers above; `HttpResponse` constructors/accessors; `ApplicationConfigBuilder::{name, version, with_name, with_version, build_loader}`; SAF construction (`plain_http_egress`, `plain_http_stream`, `egress_from_middleware_client`, `observe_http_egress`, `configure_http_builder`, config/validation operations) |

These operations currently include concrete types re-exported from `lib.rs`: `MiddlewareHttpCache`, `CassetteLayer`, `RateLayerRateMetrics`, `BreakerLayerBreakerMetrics`, `DefaultResilientLayerComposer`, and transport stream/request/response/security-context types. Their consumer-facing contracts must remain usable while concrete implementation types move behind API factory traits.

## Current Layout Findings

- Contract declarations are mixed with implementations: e.g. `cassette/main/src/api/default`, `api/body`, `api/cassette`, and transport API modules contain concrete types and implementation-facing modules.
- Several public builder declarations live under `api/` while their inherent implementations live under `core/`. The API package split must give consumers one canonical builder surface and eliminate duplicate or orphan builder types without losing any option.
- Root `lib.rs` re-exports both API DTO/config types and SAF factory types. Current SAF factory methods are inherent methods; #30 requires deciding the factory trait contract and making implementation types crate-private without losing any construction path.
- Transport defines both API builder types and core builder implementations for HTTP config/request construction. The migration must preserve the API builder methods while removing the redundant construction path only after call sites and tests are mapped.
- `resilient` currently exposes `DefaultResilientLayerComposer` from `core/`; public construction ownership must be assigned to a contract factory before implementation types become crate-private.
- Current standard implementations use reqwest and reqwest-middleware; breaker/retry also depend on the `edge-transport-breaker*` and `edge-transport-retry*` domain crates. Whether those are standard or provider-specific dependencies remains a T1 classification item.

### Concrete Implementation Dependencies Leaking into Current API Modules

These declarations cannot move into a contract-only crate unchanged. Their current fields or constructors expose implementation dependencies, so the migration must separate the contract from the concrete implementation and replace public construction with the mapped factory contract:

| Current API declaration | Implementation dependency currently embedded | Migration consequence |
|---|---|---|
| `breaker::BreakerLayerBreakerMetrics` | `moka::future::Cache`, `tokio::sync::Mutex`, breaker-policy node | Move concrete cache/state to breaker-std; keep `BreakerMetrics` as the consumer-facing contract and expose construction through its factory contract. |
| `cache::MiddlewareHttpCache` | Moka cache and `reqwest::Client` fields | Make middleware implementation crate-private in cache-std; preserve supported cache inspection/configuration through contract traits/factory. |
| `rate::RateLayerRateMetrics` | Moka cache, Tokio mutex, and core token-bucket type | Move concrete metrics state to rate-std; keep only the metrics contract and factory in rate-api. |
| `retry::RetryLayer` and `DecorateResponse` | Middleware type is consumed by `reqwest_middleware::ClientBuilder`; the core module implements the provider middleware trait | Define how the API factory returns a layer without exposing an implementation type; validate compatibility with `ClientBuilder::with` before choosing the contract. |
| `ApplicationConfigBuilder` in breaker/cache/cassette/rate/retry/transport | Wraps `configbuilder::ConfigBuilderImpl` in API source (usually `pub(crate)` field) | Move config loading/building implementation to -std or define the smallest public configuration contract needed by consumers; do not add configbuilder to every -api dependency without evidence it is part of the contract. |
| `resilient-port::ResilientLayerComposer` / request DTOs | `reqwest_middleware::ClientBuilder`, `configbuilder::SectionLoaderImpl`, and cassette config type | The port is explicitly provider-coupled today. Decide whether this composition port is allowed to be reqwest/configbuilder-specific or replace it with a provider-neutral contract; either way remove direct cassette implementation dependency from the API contract. |

Transport's `HttpEgress` trait itself uses local request/response types, but `HttpTransportSvc` exposes reqwest-specific construction methods (`configure_http_builder`, `egress_from_middleware_client`). Keep those in transport-std or explicitly classify reqwest as part of the transport contract; do not copy these methods into transport-api by default. `HttpByteStream`, `SseStream`, and websocket sender/receiver contracts also currently expose futures/Tokio implementation types and need an explicit package/dependency owner.

## Consumer Search

- Inside this workspace, only `resilient` composes the middleware crates; `resilient-port` is consumed by `resilient` and depends on cassette.
- The local `edge` checkout refers to `swe-edge-egress-http-transport` from the separate `edge-egress-http` repository, not these current workspace package names. That checkout is dirty and was read only.
- No Cargo-manifest references to these current package names were found in the local `edge-application` or `edge-security` checkouts. The old `edge-plugin-a2a` checkout was absent locally; the current upstream repository is `sweengineeringlabs/edge-llm-a2a` on `main`. Its 14 Cargo manifests were checked through GitHub's Contents API and none declares these workspace packages. The only Cargo-manifest text match is a comment in `scm/adapter/a2a/Cargo.toml` stating its outbound client uses plain reqwest. GitHub code search for current and former transport package names returned no references.
- The current upstream `sweengineeringlabs/edge-bootstrap` `dev` branch has one manifest consumer: `scm/examples/egress-proxy/Cargo.toml` aliases the transport package as `swe-edge-egress-http` and pins tag `v0.8.1`. Its source constructs `HttpConfig` with `with_base_url`; it does not use the removed `max_retries` field. This is an example consumer and its import paths must be checked during migration.

## Remaining T1 Review Items

- Map every public re-export and every consumer call site to the target contract/factory package.
- Inspect all config fields, loader sections, and constructor behavior; compare each field's current behavior before/after.
- Classify every dependency as API, standard implementation, or provider-specific using its actual role and feature use.
- Verify any consumers not covered by the `edge-application`, `edge-security`, or `edge-llm-a2a` checks, especially after package naming changes.
- Record the final file/factory/test/example move map before applying the migration script.
