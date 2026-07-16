//! [`Validator`] — self-validation contract for constructed auth types.

/// Types that can assert their own post-construction validity.
pub trait Validator: Send + Sync {
    /// Validate `self`, returning `Ok(())` if valid or an error message otherwise.
    fn validate(&self) -> Result<(), String>;
}
