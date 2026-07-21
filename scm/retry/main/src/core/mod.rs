//! Core layer — implementations of the api/ contracts.
//!
//! api/ declares the traits and value objects; every impl block lives here.

mod application_config_builder;
mod default_validator;
mod error_classifier;
mod http_retry_svc;
mod layer;
mod policy;
mod policy_builder;

pub(crate) use default_validator::DefaultValidator;
