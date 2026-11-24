//! Transport-owned measurement events, independent of any metrics backend.

use std::sync::Arc;

/// The aggregation semantics of a transport measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMetricKind {
    /// Add the value to a counter.
    Counter,
    /// Observe a distribution value.
    Histogram,
}

/// One synchronous, borrowed transport measurement.
///
/// The transport retains its existing metric names, values and method labels.
/// An observer that retains an event beyond the call must copy its borrowed data.
#[derive(Debug, Clone, Copy)]
pub struct HttpMetricObservation<'a> {
    /// Stable transport metric name.
    pub name: &'static str,
    /// Counter or histogram semantics.
    pub kind: HttpMetricKind,
    /// Measurement value; latency is expressed in microseconds.
    pub value: f64,
    /// Labels for this measurement, currently the HTTP method.
    pub labels: &'a [(&'a str, &'a str)],
}

/// Shared synchronous observation capability supplied by the composition root.
///
/// Telemetry failures are handled by the observer and do not replace the HTTP
/// result. As with the previous metrics-provider callback, panics propagate.
pub type HttpMetricsObserver = Arc<dyn for<'a> Fn(HttpMetricObservation<'a>) + Send + Sync>;
