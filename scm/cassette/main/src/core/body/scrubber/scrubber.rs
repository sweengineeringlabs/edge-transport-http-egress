//! Apply `config.scrub_body_paths` to a request body before
//! hashing, so SDK-injected non-deterministic fields
//! (request_id, trace_id, timestamps) don't break exact-match
//! on replay.
//!
//! Path syntax: dot-separated segments, e.g. `"request_id"`,
//! `"metadata.trace_id"`, or `"results.0.id"`. Each segment is
//! either an object key or, when the current value is a JSON
//! array and the segment parses as `usize`, an array index. The
//! terminal segment's field is removed (`Map::remove`) or, for
//! arrays, the element at that index (`Vec::remove`). Non-existent
//! paths and out-of-bounds indices are no-ops.
//!
//! ## Limitations
//!
//! - Only JSON bodies are scrubbed. Non-JSON bodies (binary,
//!   form-encoded, plain text) hash as-is.
//! - Array segments must be numeric strings — a non-numeric
//!   segment into an array is a no-op (we bail rather than
//!   pretending it matched).

pub(crate) struct BodyScrubber;

impl BodyScrubber {
    /// Apply each path to the raw body and return the resulting
    /// bytes. If the body isn't valid JSON, returns the raw bytes
    /// unchanged — scrubbing is best-effort and doesn't gate hashing.
    pub(crate) fn scrub_body(raw: &[u8], paths: &[String]) -> Vec<u8> {
        if paths.is_empty() {
            return raw.to_vec();
        }
        let mut value: serde_json::Value = match serde_json::from_slice(raw) {
            Ok(v) => v,
            Err(_) => return raw.to_vec(), // not JSON → no scrub
        };
        for path in paths {
            Self::remove_path(&mut value, path);
        }
        // Re-serialize. `to_vec` preserves object key order as
        // serde_json emits it (sorted by insertion if the value is
        // from parsing; stable enough for hash reproducibility
        // across runs with the same scrub paths).
        serde_json::to_vec(&value).unwrap_or_else(|_| raw.to_vec())
    }

    /// Descend into `value` following `path` (dot-separated) and
    /// remove the terminal field or element. No-op if the path
    /// doesn't exist, an intermediate segment doesn't match the
    /// current value's shape (object key missing, array index OOB,
    /// non-numeric segment into an array, scalar encountered
    /// mid-path), or the terminal is missing.
    pub(crate) fn remove_path(value: &mut serde_json::Value, path: &str) {
        let mut segments: Vec<&str> = path.split('.').collect();
        let Some(terminal) = segments.pop() else {
            return;
        };
        let mut current = value;
        for seg in segments {
            let Some(next) = Self::descend(current, seg) else {
                return;
            };
            current = next;
        }
        match current {
            serde_json::Value::Object(map) => {
                map.remove(terminal);
            }
            serde_json::Value::Array(array) => match terminal.parse::<usize>() {
                // Terminal array removal: numeric in-bounds removes
                // the element (array shrinks). Anything else is a
                // no-op.
                Ok(index) if index < array.len() => {
                    array.remove(index);
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn descend<'a>(
        value: &'a mut serde_json::Value,
        segment: &str,
    ) -> Option<&'a mut serde_json::Value> {
        match value {
            serde_json::Value::Object(map) => map.get_mut(segment),
            serde_json::Value::Array(array) => segment
                .parse::<usize>()
                .ok()
                .and_then(|index| array.get_mut(index)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: remove_path
    #[test]
    fn test_remove_path_removes_top_level_key() {
        let mut v = serde_json::json!({"a": 1, "b": 2});
        BodyScrubber::remove_path(&mut v, "a");
        assert!(v.get("a").is_none(), "key 'a' must be removed");
        assert_eq!(v.get("b").and_then(|x| x.as_i64()), Some(2));
    }

    /// @covers: remove_path
    #[test]
    fn test_remove_path_noop_on_missing_key() {
        let mut v = serde_json::json!({"a": 1});
        BodyScrubber::remove_path(&mut v, "nonexistent");
        // Value must be unchanged.
        assert_eq!(v.get("a").and_then(|x| x.as_i64()), Some(1));
    }

    /// @covers: remove_path
    #[test]
    fn test_remove_path_removes_nested_key() {
        let mut v = serde_json::json!({"outer": {"inner": "secret", "keep": "yes"}});
        BodyScrubber::remove_path(&mut v, "outer.inner");
        let outer = v.get("outer").unwrap();
        assert!(outer.get("inner").is_none(), "nested key must be removed");
        assert_eq!(outer.get("keep").and_then(|x| x.as_str()), Some("yes"));
    }

    /// @covers: remove_path
    #[test]
    fn test_remove_path_descends_array_and_removes_nested_key() {
        let mut value = serde_json::json!({"items": [{"secret": "x", "keep": "y"}]});
        BodyScrubber::remove_path(&mut value, "items.0.secret");

        let item = &value["items"][0];
        assert!(item.get("secret").is_none());
        assert_eq!(item.get("keep").and_then(|entry| entry.as_str()), Some("y"));
    }

    /// @covers: remove_path
    #[test]
    fn test_remove_path_removes_array_element() {
        let mut value = serde_json::json!({"items": ["first", "remove", "last"]});
        BodyScrubber::remove_path(&mut value, "items.1");

        assert_eq!(value["items"], serde_json::json!(["first", "last"]));
    }

    /// @covers: remove_path
    #[test]
    fn test_remove_path_noop_on_scalar_mid_path() {
        // "a" is a scalar, not an object; descending into it must bail.
        let mut v = serde_json::json!({"a": 42});
        BodyScrubber::remove_path(&mut v, "a.b");
        // Original value unchanged.
        assert_eq!(v.get("a").and_then(|x| x.as_i64()), Some(42));
    }

    /// @covers: scrub_body
    #[test]
    fn test_scrub_body_removes_specified_path() {
        let body = br#"{"id":"abc","keep":"yes"}"#;
        let paths = vec!["id".to_string()];
        let result = BodyScrubber::scrub_body(body, &paths);
        let v: serde_json::Value = serde_json::from_slice(&result).unwrap();
        assert!(v.get("id").is_none());
        assert_eq!(v.get("keep").and_then(|x| x.as_str()), Some("yes"));
    }

    /// @covers: scrub_body
    #[test]
    fn test_empty_paths_returns_raw_unchanged() {
        let body = br#"{"a": 1}"#;
        assert_eq!(BodyScrubber::scrub_body(body, &[]), body.to_vec());
    }

    /// @covers: scrub_body
    #[test]
    fn test_non_json_body_returns_raw_unchanged() {
        let body = b"not actual json";
        let paths = vec!["some.path".to_string()];
        assert_eq!(BodyScrubber::scrub_body(body, &paths), body.to_vec());
    }
}
