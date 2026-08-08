//! Public contract: trait, request DTOs, and error type.

pub mod errors;
pub mod traits;
pub mod types;

pub use errors::ResilientError;
pub use traits::ResilientLayers;
pub use types::{ApplyDefaultsRequest, ApplyFromConfigRequest};
