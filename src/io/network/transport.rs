//! # Transport
//!
//! `transport` is the module providing the trait implemented by network transports.

use base::Result;
use base::VariableSize;
use base::Datable;

/// Trait implemented by transports used by network clients.
pub trait ClientTransport<A>
    where   A: Datable + VariableSize,
            Self: 'static + Sized + Send + Sync + Clone
{
    /// Opens one or more connections to one or more network addresses.
    fn connect<P: Datable>(params: &P, addresses: &Vec<A>) -> Result<Self>;

    /// Closes the connections.
    fn disconnect<P: Datable>(&mut self, params: &P) -> Result<()>;

    /// Sends data through the network connections.
    fn send<P: Datable>(&mut self, params: &P, data: &[u8]) -> Result<()>;

    /// Receives data from the network connections, returning an handle to
    /// the transport that can be used to reply to the caller.
    fn recv<P: Datable>(&mut self, params: &P) -> Result<Vec<Vec<u8>>>;
}

/// Trait implemented by transports for network servers.
pub trait ServerTransport<A, CT>
    where   A: Datable + VariableSize,
            CT: ClientTransport<A>,
            Self: 'static + Sized + Send + Sync
{
    /// Listens to connections incoming from one or more network addresses.
    fn listen<P: Datable>(sparams: &P, addresses: &Vec<A>) -> Result<Self>;

    /// Accepts connections incoming from one or more network addresses.
    fn accept<P: Datable>(&mut self, params: &P) -> Result<(CT, Vec<A>)>;

    /// Closes the connections.
    fn close<P: Datable>(&mut self, params: &P) -> Result<()>;
}