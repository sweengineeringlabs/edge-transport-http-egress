//! `impl HttpSecurityContext` — the declaration lives in `api/`.

use crate::api::HttpSecurityContext;

impl HttpSecurityContext {
    /// Unwrap back into the real `edge_security_runtime::SecurityContext`.
    pub fn into_inner(self) -> edge_security_runtime::SecurityContext {
        self.0
    }
}

impl From<edge_security_runtime::SecurityContext> for HttpSecurityContext {
    fn from(ctx: edge_security_runtime::SecurityContext) -> Self {
        Self(ctx)
    }
}
