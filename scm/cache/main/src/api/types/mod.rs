//! Value objects for the cache API.

pub(crate) mod application_config_builder;

pub mod cache;
pub use cache::cache_config::CacheConfig;
pub use cache::middleware_http_cache::MiddlewareHttpCache;
pub mod http_cache_svc_processor;

// Re-export HttpCacheSvcProcessor at the types/ level for use by core/ and saf/.
pub use http_cache_svc_processor::HttpCacheSvcProcessor;
