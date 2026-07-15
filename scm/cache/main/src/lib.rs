//! edge_transport_http_egress_cache — RFC-7234 HTTP cache middleware (wraps http-cache-reqwest with moka).
//!

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use crate::api::types::cached_entry_builder::CachedEntryBuilder;
pub use crate::api::types::CacheLayer;
pub use crate::api::{CacheConfig, CacheError, HttpCache, HttpCacheSvc, Processor, Validator};
