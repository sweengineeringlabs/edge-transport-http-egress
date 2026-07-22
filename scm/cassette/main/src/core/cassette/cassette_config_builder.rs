//! Impl block for [`CassetteConfigBuilder`] — api/ declares the fields,
//! the fluent construction logic lives here.

use crate::api::{CassetteConfig, CassetteConfigBuilder, CassetteError};

#[expect(
    clippy::derivable_impls,
    reason = "written by hand (not #[derive(Default)]) so the impl block lives in core/, \
              not api/ — api/ is a pure declaration layer"
)]
impl Default for CassetteConfigBuilder {
    fn default() -> Self {
        Self {
            mode: None,
            cassette_dir: None,
            match_on: None,
            scrub_headers: None,
            scrub_body_paths: None,
        }
    }
}

impl CassetteConfigBuilder {
    /// Create a new builder with all fields unset (defaults apply on `build`).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the operating mode: `"replay"` | `"record"` | `"auto"` | `"disabled"`.
    pub fn with_mode(mut self, mode: impl Into<String>) -> Self {
        self.mode = Some(mode.into());
        self
    }

    /// Set the cassette directory.
    pub fn with_cassette_dir(mut self, dir: impl Into<String>) -> Self {
        self.cassette_dir = Some(dir.into());
        self
    }

    /// Set the request components included in the match key.
    pub fn with_match_on(mut self, keys: Vec<String>) -> Self {
        self.match_on = Some(keys);
        self
    }

    /// Set the headers to strip before writing the cassette.
    pub fn with_scrub_headers(mut self, headers: Vec<String>) -> Self {
        self.scrub_headers = Some(headers);
        self
    }

    /// Set the JSON body paths to scrub before hashing.
    pub fn with_scrub_body_paths(mut self, paths: Vec<String>) -> Self {
        self.scrub_body_paths = Some(paths);
        self
    }

    /// Consume the builder and produce a [`CassetteConfig`].
    ///
    /// Returns an error if `mode` is set but is not one of the recognised values.
    pub fn build_config(self) -> Result<CassetteConfig, CassetteError> {
        let defaults = CassetteConfig::default();
        let mode = self.mode.unwrap_or(defaults.mode);
        match mode.as_str() {
            "replay" | "record" | "auto" | "disabled" => {}
            other => {
                return Err(CassetteError::ParseFailed(format!(
                    "unknown cassette mode '{other}'; expected replay|record|auto|disabled"
                )));
            }
        }
        Ok(CassetteConfig {
            mode,
            cassette_dir: self.cassette_dir.unwrap_or(defaults.cassette_dir),
            match_on: self.match_on.unwrap_or(defaults.match_on),
            scrub_headers: self.scrub_headers.unwrap_or(defaults.scrub_headers),
            scrub_body_paths: self.scrub_body_paths.unwrap_or(defaults.scrub_body_paths),
        })
    }
}
