//! Error classification for the retry middleware.

/// Classifies reqwest-middleware errors as transient (retry-worthy) or not.
pub(crate) struct RetryErrorClassifier;

impl RetryErrorClassifier {
    /// Classify a `reqwest_middleware::Error` as transient (retry-worthy) or
    /// non-transient. Transient failures: connection reset, DNS temporary
    /// failures, read timeout, etc. Non-transient: certificate errors,
    /// malformed URL, middleware-level (config/credential) errors.
    pub(crate) fn is_transient(err: &reqwest_middleware::Error) -> bool {
        match err {
            reqwest_middleware::Error::Reqwest(inner) => {
                // I/O-shaped reqwest errors are worth retrying; certificate /
                // redirect / builder errors are not.
                inner.is_timeout() || inner.is_connect() || inner.is_request()
            }
            // Middleware-level errors reflect configuration or credential
            // problems — retrying won't help.
            reqwest_middleware::Error::Middleware(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: is_transient
    #[test]
    fn test_is_transient_middleware_error_is_not_transient() {
        let err = reqwest_middleware::Error::middleware(std::io::Error::other("config error"));
        assert!(
            !RetryErrorClassifier::is_transient(&err),
            "Middleware-level errors must NOT be retried"
        );
    }
}
