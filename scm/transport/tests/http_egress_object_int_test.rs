//! Integration tests for `HttpEgressObject`.
//!
//! Rule 120: `src/api/default/http/egress/http_egress_object.rs` requires a
//! corresponding test file.
//!
//! `HttpEgressObject` is a type alias for `dyn HttpEgress`. We test that the
//! trait is object-safe and that the alias is accessible from the public surface.

use core::marker::PhantomData;

use edge_transport_http_egress_transport::{DefaultEgress, HttpEgress};

/// @covers: HttpEgressObject (object safety via HttpEgress)
/// `dyn HttpEgress` only forms a valid type if `HttpEgress` is object-safe, so
/// `PhantomData::<dyn HttpEgress>` is a compile-time contract that fails to
/// compile the moment a non-dispatchable method is added to the trait.
/// (Replaces a prior `assert!(true)` that could not fail.)
#[test]
fn transport_struct_http_egress_object_is_object_safe_int_test() {
    let _dyn_compatible = PhantomData::<dyn HttpEgress>;
}

/// @covers: HttpEgressObject type alias accessibility
/// `DefaultEgress` is the SAF-exported alias for `HttpEgressObject` (`dyn
/// HttpEgress`); naming it is a compile-time contract that fails if the alias is
/// removed or renamed. (Replaces a prior `assert!(size_of::<*const _>() > 0)`,
/// which was always true.)
#[test]
fn transport_struct_http_egress_object_alias_is_accessible_int_test() {
    let _exists = PhantomData::<DefaultEgress>;
}
