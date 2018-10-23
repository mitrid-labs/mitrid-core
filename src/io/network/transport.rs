//! # Transport
//!
//! `transport` is the module providing the trait implemented by network transports.

use base::Future;
use base::VariableSize;
use base::Datable;

/// Trait implemented by network transports.
pub trait Transport<A>
    where   A: Datable + VariableSize
{
    /// Opens one or more connections to one or more network addresses.
    fn connect<P: Datable>(&mut self, params: &P, addresses: &Vec<A>) -> Future<()>;

    /// Closes the connections.
    fn disconnect<P: Datable>(&mut self, params: &P, addresses: &Vec<A>) -> Future<()>;

    /// Listen to connections incoming from one or more network addresses.
    fn listen<P: Datable>(&mut self, params: &P, addresses: &Vec<A>) -> Future<()>;

    /// Sends data through the network connections.
    fn send<P: Datable>(&mut self, params: &P, data: &[u8]) -> Future<Vec<Vec<u8>>>;

    /// Receives data from the network connections.
    fn recv<P: Datable>(&mut self, params: &P) -> Future<Vec<Vec<u8>>>;
}