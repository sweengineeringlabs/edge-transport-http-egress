//! `LoadbalancerSvcProcessor` — zero-size service struct with factory methods.

/// Zero-size marker with factory methods for the loadbalancer middleware and
/// the crate's [`Processor`](crate::api::Processor) identity.
///
/// All construction goes through `LoadbalancerSvcProcessor::build_layer`;
/// consumers never name core types directly.
pub struct LoadbalancerSvcProcessor;
