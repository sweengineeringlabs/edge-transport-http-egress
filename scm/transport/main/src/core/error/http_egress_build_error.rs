//! `From<ForeignError>` conversion for [`HttpEgressBuildError`] — the `?`-operator conversion used by
//! the bare-primitive constructors in `saf/transport_svc.rs`.

use crate::api::{AssemblyFailure, HttpEgressBuildError};

impl From<reqwest::Error> for HttpEgressBuildError {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(AssemblyFailure(Box::new(e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The `Reqwest` variant's `Display` must carry the `reqwest:` prefix so a
    /// caller can tell *which* assembly step failed from the message alone.
    #[test]
    fn test_reqwest_variant_display_starts_with_reqwest_prefix() {
        let err = HttpEgressBuildError::Reqwest(AssemblyFailure(Box::new(std::fmt::Error)));
        let msg = err.to_string();
        assert!(
            msg.starts_with("reqwest:"),
            "Display must start with 'reqwest:', got: {msg:?}"
        );
    }

    /// `From<reqwest::Error>` must map into the `Reqwest` variant — a malformed
    /// URL makes `RequestBuilder::build()` yield a real `reqwest::Error`, so this
    /// exercises the actual conversion the `?` operator uses, not a hand-built stub.
    #[test]
    fn test_from_reqwest_error_maps_to_reqwest_variant() {
        let reqwest_err = reqwest::Client::new()
            .get("http://a b")
            .build()
            .expect_err("a URL containing a space must fail to build");
        let mapped: HttpEgressBuildError = reqwest_err.into();
        assert!(
            matches!(mapped, HttpEgressBuildError::Reqwest(_)),
            "reqwest::Error must convert into HttpEgressBuildError::Reqwest, got: {mapped:?}"
        );
    }
}
