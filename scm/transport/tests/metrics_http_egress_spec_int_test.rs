//! Integration tests for `MetricsHttpEgressSpec` marker trait.
//!
//! Rule 120: `src/api/metrics/metrics_http_egress_spec.rs` requires a
//! corresponding test file.
//!
//! `MetricsHttpEgressSpec` is a marker trait for metrics HTTP egress
//! implementations. The concrete implementation is `MetricsHttpEgress`
//! (exported as `MetricsEgress`).

use core::marker::PhantomData;

use edge_transport_http_egress_transport::MetricsEgress;

/// @covers: MetricsHttpEgressSpec (via MetricsEgress)
/// Naming the public `MetricsEgress` alias is a compile-time contract: this test
/// fails to compile if the alias is removed or renamed. (Replaces a prior
/// `assert!(size_of::<*const _>() > 0)` — pointer size is a constant, so the
/// assertion was always true and could never catch a regression.)
#[test]
fn transport_trait_metrics_http_egress_spec_alias_is_accessible_int_test() {
    let _exists = PhantomData::<MetricsEgress>;
}

/// @covers: MetricsHttpEgressSpec usability
/// `MetricsEgress` must remain usable as a reference target; the coercion to a
/// `fn(&MetricsEgress)` pointer below fails to compile if the type stops being a
/// valid sized referent. (Replaces a prior `assert!(true)`.)
#[test]
fn transport_trait_metrics_http_egress_spec_is_usable_behind_ref_int_test() {
    fn accepts_ref(_: &MetricsEgress) {}
    let _coerced: fn(&MetricsEgress) = accepts_ref;
}
