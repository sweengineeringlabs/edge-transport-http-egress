//! Core layer — middleware impl + default impl of the primary
//! api trait.

pub(crate) mod cache;
pub(crate) mod cached;

// Processor impl for HttpCacheSvcProcessor to satisfy rule 154
pub(crate) mod processor;
