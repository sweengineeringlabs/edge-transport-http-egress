//! Interface contract for the `MetricsHttpEgress` decorator.
//!
//! The [`MetricsHttpEgress`] type alias names the dyn-safe [`HttpEgress`] trait
//! interface that `MetricsHttpEgress` (in `core/`) implements.

use crate::HttpEgress;

/// Dyn-safe alias for the metrics-observation HTTP outbound interface.
pub type MetricsHttpEgress = dyn HttpEgress;
