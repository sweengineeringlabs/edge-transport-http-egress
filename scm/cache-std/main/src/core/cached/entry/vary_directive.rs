//! Classification of a response's `Vary` header for caching decisions.

/// What the response's `Vary` header means for caching.
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum VaryDirective {
    /// Response had no `Vary` header — no per-header keying.
    None,
    /// `Vary: *` — response varies on things we can't observe.
    /// RFC 7234 says: do not cache.
    Star,
    /// Response varies on specific request headers. Names are
    /// normalized to lowercase, sorted, and de-duplicated.
    Names(Vec<String>),
}
