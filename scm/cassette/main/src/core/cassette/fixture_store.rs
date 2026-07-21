//! `FixtureStore` — guards [`CassetteLayer`](crate::api::CassetteLayer)'s
//! in-memory fixture map. Declared in core/ (not api/) since its field type
//! (`tokio::sync::Mutex`) is an external crate type — api/'s field
//! referencing it via a fully-qualified inline path is the established SEA
//! pattern for core-internal types appearing in api-visible struct fields.

use std::collections::HashMap;

use crate::core::recorded::interaction::interaction::RecordedInteraction;

pub(crate) struct FixtureStore(pub(crate) tokio::sync::Mutex<HashMap<String, RecordedInteraction>>);

impl std::ops::Deref for FixtureStore {
    type Target = tokio::sync::Mutex<HashMap<String, RecordedInteraction>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: deref
    #[tokio::test]
    async fn test_deref_gives_access_to_the_guarded_map() {
        let store = FixtureStore(tokio::sync::Mutex::new(HashMap::new()));
        let guard = store.lock().await;
        assert!(guard.is_empty());
    }
}
