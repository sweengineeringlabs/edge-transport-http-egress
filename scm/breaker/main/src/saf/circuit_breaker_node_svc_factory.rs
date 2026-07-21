//! Composition site for [`CircuitBreakerNode`] — one file per trait keeps
//! wiring focused.
//!
//! `create` is intentionally not paired with an
//! `impl CircuitBreakerNode for CircuitBreakerNodeFactory` here:
//! `CircuitBreakerNode::admit`/`::record` mutate state (`&mut self`), so a
//! delegating impl on this zero-state factory would have to construct a
//! fresh throwaway node on every call — silently discarding any prior
//! `record()` before every `admit()`. That's a real correctness trap if this
//! factory were ever used directly as a `Box<dyn CircuitBreakerNode>` instead
//! of through `create()`.

use crate::api::CircuitBreakerNode;
use crate::core::host::DefaultHostBreaker;

/// Factory for the default [`CircuitBreakerNode`].
pub struct CircuitBreakerNodeFactory;

impl CircuitBreakerNodeFactory {
    /// Construct the default [`CircuitBreakerNode`] — a fresh, Closed-state
    /// per-host breaker.
    pub fn create() -> Box<dyn CircuitBreakerNode> {
        Box::new(DefaultHostBreaker::new())
    }
}
