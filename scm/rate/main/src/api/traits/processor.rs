//! `Processor` — primary processing contract for the rate middleware.

/// Primary processing contract. Every rate-limiter middleware unit
/// produced by this crate implements this trait.
pub trait Processor: Send + Sync {
    /// Identify this processor in log / trace output.
    ///
    /// Returns the crate's canonical name (e.g. `"edge_transport_http_egress_rate"`).
    fn describe(&self) -> &'static str;
}
