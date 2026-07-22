//! Cached response entry + associated parsing helpers for
//! RFC 7234 `Vary`, `ETag`, and RFC 5861 `stale-while-revalidate`.

mod cache_entry_helper;
mod cached_entry;
mod cached_entry_builder;
mod vary_directive;

pub(crate) use cache_entry_helper::CacheEntryHelper;
pub(crate) use cached_entry::CachedEntry;
pub(crate) use cached_entry_builder::CachedEntryBuilder;
pub(crate) use vary_directive::VaryDirective;
