//! API layer — public schema + trait contracts + public types.
mod cassette;
mod dto;
mod error;
mod processor;
mod traits;
mod types;

// Re-export public traits and errors at the top level
pub use error::CassetteError;
pub use traits::{HttpCassette, Processor, Validator};

// Re-export public DTOs at the top level
pub use dto::{
    CassetteModeRequest, CassetteModeResponse, ConfigValidationRequest, DescribeRequest,
    DescribeResponse,
};

// Re-export public types at the top level
pub use types::{
    CassetteConfig, CassetteConfigBuilder, CassetteLayer, CassetteLayerBuilder, HttpCassetteSvc,
};
