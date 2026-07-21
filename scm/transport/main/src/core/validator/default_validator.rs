//! Default pass-through [`Validator`] implementation.
//!
//! [`DefaultValidator`] wraps any value that implements [`Validator`] from the
//! API layer and delegates to it, providing the `impl Validator for` that
//! satisfies SEA Rule 49.

use crate::api::{ValidateRequest, ValidationError, Validator};

/// A pass-through [`Validator`] implementation used by the SAF layer.
///
/// Delegates validation to the inner value. Core infrastructure components
/// use this wrapper so they satisfy the SEA Rule 49 requirement that every
/// trait declared in `api/` has at least one `impl Validator for` in `core/`.
pub(crate) struct DefaultValidator<T: Validator> {
    inner: T,
}

impl<T: Validator> DefaultValidator<T> {
    pub(crate) fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: Validator> Validator for DefaultValidator<T> {
    fn validate(&self, request: ValidateRequest) -> Result<(), ValidationError> {
        self.inner.validate(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DefaultAlwaysOk;
    impl Validator for DefaultAlwaysOk {
        fn validate(&self, _request: ValidateRequest) -> Result<(), ValidationError> {
            Ok(())
        }
    }

    struct DefaultAlwaysFail;
    impl Validator for DefaultAlwaysFail {
        fn validate(&self, _request: ValidateRequest) -> Result<(), ValidationError> {
            Err(ValidationError::Invalid("invalid".into()))
        }
    }

    /// @covers: new
    #[test]
    fn test_new_wraps_inner_validator() {
        // The wrapper must forward the inner validator's verdict, not fabricate
        // its own — so wrapping a passing inner yields Ok and wrapping a failing
        // inner yields that inner's exact error.
        let ok = DefaultValidator::new(DefaultAlwaysOk);
        assert!(
            ok.validate(ValidateRequest).is_ok(),
            "wrapping a passing validator must pass"
        );

        let fail = DefaultValidator::new(DefaultAlwaysFail);
        assert_eq!(
            fail.validate(ValidateRequest).unwrap_err(),
            ValidationError::Invalid("invalid".into()),
            "wrapping a failing validator must surface its error"
        );
    }

    /// @covers: validate
    #[test]
    fn test_validate_delegates_to_inner_ok() {
        // Delegation is proven by the two inners producing different outcomes.
        let ok = DefaultValidator::new(DefaultAlwaysOk);
        assert!(
            ok.validate(ValidateRequest).is_ok(),
            "Ok inner must delegate to Ok"
        );

        let fail = DefaultValidator::new(DefaultAlwaysFail);
        assert!(
            fail.validate(ValidateRequest).is_err(),
            "Err inner must delegate to Err (not swallowed by the wrapper)"
        );
    }

    /// @covers: validate
    #[test]
    fn test_validate_delegates_to_inner_err() {
        let v = DefaultValidator::new(DefaultAlwaysFail);
        assert!(v.validate(ValidateRequest).is_err());
        assert_eq!(
            v.validate(ValidateRequest).unwrap_err(),
            ValidationError::Invalid("invalid".into())
        );
    }
}
