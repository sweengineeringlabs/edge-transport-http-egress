//! `impl HttpSecurityContext` — the declaration lives in `api/`.

use crate::api::HttpSecurityContext;

impl HttpSecurityContext {
    /// Unwrap back into the real `edge_application::SecurityContext`.
    pub fn into_inner(self) -> edge_application::SecurityContext {
        self.0
    }
}

impl From<edge_application::SecurityContext> for HttpSecurityContext {
    fn from(ctx: edge_application::SecurityContext) -> Self {
        Self(ctx)
    }
}
