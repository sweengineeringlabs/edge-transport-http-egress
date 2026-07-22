//! Composition site for [`HostBreaker`] — one file per trait keeps wiring
//! focused.
//!
//! `create` is intentionally not paired with an `impl HostBreaker for
//! HostBreakerFactory` here: a delegating impl on this zero-state factory
//! would always report the fresh-instance defaults (`is_closed() == true`)
//! regardless of any real breaker's actual state, which would be misleading
//! if ever used directly as a `Box<dyn HostBreaker>` instead of through
//! `create()`.

use crate::api::HostBreaker;
use crate::core::host::DefaultHostBreaker;

/// Factory for the default [`HostBreaker`].
pub struct HostBreakerFactory;

impl HostBreakerFactory {
    /// Construct the default [`HostBreaker`] — a fresh, Closed-state
    /// per-host breaker.
    pub fn create() -> Box<dyn HostBreaker> {
        Box::new(DefaultHostBreaker::new())
    }
}
