//! A single recorded HTTP exchange.

use serde::{Deserialize, Serialize};

use crate::core::recorded::interaction::RecordedRequest;
use crate::core::recorded::interaction::RecordedResponse;

/// A single recorded HTTP exchange. Bodies are Base64 so binary
/// content round-trips safely through YAML.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RecordedInteraction {
    pub(crate) request: RecordedRequest,
    pub(crate) response: RecordedResponse,
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn test_serde_roundtrip_through_yaml() {
        let ri = RecordedInteraction {
            request: RecordedRequest {
                method: "GET".into(),
                url: "https://example.test/foo".into(),
                body_hash: None,
            },
            response: RecordedResponse {
                status: 200,
                headers: {
                    let mut m = BTreeMap::new();
                    m.insert("content-type".into(), "application/json".into());
                    m
                },
                body_base64: "aGVsbG8=".into(), // "hello"
            },
        };
        let yaml = serde_saphyr::to_string(&ri).unwrap();
        let decoded: RecordedInteraction = serde_saphyr::from_str(&yaml).unwrap();
        assert_eq!(decoded.request.method, "GET");
        assert_eq!(decoded.response.status, 200);
    }
}
