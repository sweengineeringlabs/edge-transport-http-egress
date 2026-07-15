//! Integration tests for `ValidatorObject`.
//!
//! Rule 120: `src/api/validator/validator_object.rs` requires a corresponding
//! test file.
//!
//! `ValidatorObject` is a type alias for `dyn Validator`. We test that the
//! alias is accessible and that the trait is object-safe.

use core::marker::PhantomData;

use edge_transport_http_egress_transport::DefaultValidatorAlias;

/// @covers: ValidatorObject (alias accessibility)
/// Naming the SAF-exported `DefaultValidatorAlias` is a compile-time contract:
/// this fails to compile if the alias is removed or renamed. (Replaces a prior
/// `assert!(size_of::<*const _>() > 0)`, which was always true.)
#[test]
fn transport_struct_validator_object_alias_is_accessible_int_test() {
    let _exists = PhantomData::<DefaultValidatorAlias>;
}

/// @covers: ValidatorObject object safety
/// `DefaultValidatorAlias` is `dyn Validator`; it only forms a valid type if the
/// `Validator` trait is object-safe, so binding `PhantomData::<DefaultValidatorAlias>`
/// fails to compile if object safety is lost. (Replaces a prior `assert!(true)`.)
#[test]
fn transport_struct_validator_object_is_object_safe_int_test() {
    let _dyn_compatible: PhantomData<DefaultValidatorAlias> = PhantomData;
}
