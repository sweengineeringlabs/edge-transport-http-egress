//! API layer — public schema + trait contracts + public types.

mod dto;
mod error;
mod traits;
mod types;

pub use dto::{
    DecorateRequest, DecorateResponse, ProcessorRequest, ProcessorResponse, ValidationRequest,
};
pub use error::RetryError;
pub use traits::{Processor, Validator};
pub use types::{
    ApplicationConfigBuilder, HttpRetrySvc, RetryConfig, RetryConfigBuilder, RetryLayer,
};
