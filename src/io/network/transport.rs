//! # Transport
//!
//! `transport` is the module providing the trait implemented by network transports.
use base::Future;
use base::VariableSize;
use base::Datable;
use base::Serializable;

/// Trait implemented by network transports.
pub trait Transport<A, D>
    where   A: Datable + VariableSize,
            D: Datable + Serializable
{
    /// Opens a connection to a network address.
    fn connect<P: Datable>(&mut self, params: &P, address: &A) -> Future<()>;

    /// Closes a connection to a network address.
    fn disconnect<P: Datable>(&mut self, params: &P, address: &A) -> Future<()>;

    /// Listen to connections incoming from a network address.
    fn listen<P: Datable>(&mut self, params: &P, address: &A) -> Future<()>;

    /// Sends data through a network connection.
    fn send<P: Datable>(&mut self, params: &P, data: &D) -> Future<()>;

    /// Receives data from a network connection.
    fn recv<P: Datable>(&mut self, params: &P) -> Future<D>;
}