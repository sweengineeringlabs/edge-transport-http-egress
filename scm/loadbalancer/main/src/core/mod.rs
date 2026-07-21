//! Core implementations — `pub(crate)` only; never exposed directly.

pub(crate) mod layer;
pub(crate) mod processor;
pub(crate) mod validator;

pub(crate) use validator::DefaultValidator;
