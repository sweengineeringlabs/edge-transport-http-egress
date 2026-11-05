//! Public contract: trait, request DTOs, and error type.

mod error;
mod traits;
mod types;

pub use error::ResilientError;
pub use traits::ResilientLayerComposer;
pub use types::{ResilientConfigRequest, ResilientDefaultsRequest};
