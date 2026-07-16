//! Error types for the tls crate.

pub mod tls_config_error;
pub mod tls_error;
pub use tls_config_error::TlsConfigError;
pub use tls_error::TlsError;
