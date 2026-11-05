//! Request / response DTOs for the api/ trait contracts.

mod decorate_request;
mod decorate_response;
mod processor_request;
mod processor_response;
mod validation_request;

pub use decorate_request::DecorateRequest;
pub use decorate_response::DecorateResponse;
pub use processor_request::ProcessorRequest;
pub use processor_response::ProcessorResponse;
pub use validation_request::ValidationRequest;
