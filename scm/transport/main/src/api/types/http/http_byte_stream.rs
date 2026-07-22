//! Streaming HTTP response body type (egress).

use std::pin::Pin;

use futures::Stream;

use crate::api::error::HttpEgressError;

/// A lazy stream of raw byte chunks consumed from a streaming HTTP response
/// body. Implements [`Stream`] — drive it with [`futures::StreamExt::next`].
pub struct HttpByteStream(
    pub(crate) Pin<Box<dyn Stream<Item = Result<Vec<u8>, HttpEgressError>> + Send>>,
);
