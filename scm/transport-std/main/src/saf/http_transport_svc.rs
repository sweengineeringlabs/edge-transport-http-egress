//! `HttpTransportSvc` — SAF factory type declaration.

/// Zero-sized factory type for the HTTP egress transport SAF.
///
/// All public factory functions for assembling [`HttpEgress`](crate::HttpEgress) and
/// [`HttpStream`](crate::HttpStream) instances are implemented as associated methods on
/// this struct in `saf/`.
pub(crate) struct HttpTransportSvc;
