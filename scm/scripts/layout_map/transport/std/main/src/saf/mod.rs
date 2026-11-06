//! Service-access facade: the public construction path of the standard transport.
mod http_transport_svc;
mod std_transport_factory;
mod transport_svc;

pub use std_transport_factory::StdTransportFactory;
