//! `Debug`/`Display`/`Error` impls for [`AssemblyFailure`] — the declaration
//! lives in `api/`.

use std::fmt;

use crate::api::AssemblyFailure;

impl fmt::Debug for AssemblyFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for AssemblyFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl std::error::Error for AssemblyFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.0.as_ref())
    }
}
