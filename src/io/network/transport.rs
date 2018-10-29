//! # Transport
//!
//! `transport` is the module providing the trait implemented by network transports.

use base::Future;
use base::Stream;
use base::VariableSize;
use base::Datable;

/// Trait implemented by transports used by network clients.
pub trait ClientTransport<A>
    where   A: Datable + VariableSize,
            Self: 'static + Clone + Send + Sync
{
    /// Opens one or more connections to one or more network addresses.
    fn connect<P: Datable>(params: &P, addresses: &Vec<A>) -> Future<Self>;

    /// Closes the connections.
    fn disconnect<P: Datable>(&mut self, params: &P, addresses: &Vec<A>) -> Future<()>;

    /// Sends data through the network connections.
    fn send<P: Datable>(&mut self, params: &P, data: &[u8]) -> Future<()>;

    /// Receives data from the network connections, returning an handle to
    /// the transport that can be used to reply to the caller.
    fn recv<P: Datable>(&mut self, params: &P) -> Stream<Vec<u8>>;
}

/// Trait implemented by transports for network servers.
pub trait ServerTransport<A, CT>
    where   A: Datable + VariableSize,
            CT: ClientTransport<A>,
            Self: 'static + Clone + Send + Sync
{
    /// Listens to connections incoming from one or more network addresses.
    fn listen<P: Datable>(sparams: &P, addresses: &Vec<A>) -> Future<Self>;

    /// Accepts connections incoming from one or more network addresses.
    fn accept<P: Datable>(&mut self, params: &P) -> Future<(CT, Vec<A>)>;

    /// Closes the connections.
    fn close<P: Datable>(&mut self, params: &P) -> Future<()>;
}