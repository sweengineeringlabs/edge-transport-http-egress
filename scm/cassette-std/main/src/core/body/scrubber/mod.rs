//! Body scrubbing — removes non-deterministic fields from JSON request bodies
//! before hashing, so SDK-injected fields do not break exact-match on replay.
#[allow(clippy::module_inception)]
pub(crate) mod scrubber;
pub(crate) use scrubber::BodyScrubber;
