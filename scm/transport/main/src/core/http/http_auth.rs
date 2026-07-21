//! `impl HttpAuth` — the declaration lives in `api/`.

use crate::api::HttpAuth;

impl HttpAuth {
    /// Construct a `Bearer` variant.
    pub fn bearer(token: impl Into<String>) -> Self {
        HttpAuth::Bearer {
            token: token.into(),
        }
    }

    /// Construct a `Basic` variant.
    pub fn basic(username: impl Into<String>, password: impl Into<String>) -> Self {
        HttpAuth::Basic {
            username: username.into(),
            password: password.into(),
        }
    }

    /// Construct an `ApiKey` variant.
    pub fn api_key(header: impl Into<String>, key: impl Into<String>) -> Self {
        HttpAuth::ApiKey {
            header: header.into(),
            key: key.into(),
        }
    }
}
