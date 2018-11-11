//! # Transport
//!
//! `transport` is the module providing the trait implemented by network transports.

use base::Result;
use base::VariableSize;
use base::Checkable;
use base::Datable;
use base::{Eval, EvalMut};

/// Trait implemented by transports used by network clients.
pub trait ClientTransport<A>
    where   A: Datable + VariableSize,
            Self: 'static + Sized + Send + Sync + Clone + Checkable
{
    /// Opens one or more connections to one or more network addresses.
    fn connect(address: &A) -> Result<Self>;

    /// Closes the connections.
    fn disconnect(&mut self) -> Result<()>;

    /// Sends data through the network connections.
    fn send(&mut self, data: &[u8]) -> Result<()>;

    /// Receives data from the network connections, returning an handle to
    /// the transport that can be used to reply to the caller.
    fn recv(&mut self) -> Result<Vec<Vec<u8>>>;

    /// Eval operation in the client transport.
    fn eval<Ev: Eval<Self, P, R>, P: Datable, R: Datable>(&self, params: &P, evaluator: &Ev) -> Result<R> {
        self.check()?;

        params.check()?;

        evaluator.eval(self, params)
    }

    /// Mutable eval operation in the client transport.
    fn eval_mut<EvM: EvalMut<Self, P, R>, P: Datable, R: Datable>(&mut self, params: &P, evaluator: &mut EvM) -> Result<R> {
        self.check()?;

        params.check()?;

        let result = evaluator.eval_mut(self, params)?;
        self.check()?;

        Ok(result)
    }
}

/// Trait implemented by transports for network servers.
pub trait ServerTransport<A, CT>
    where   A: Datable + VariableSize,
            CT: ClientTransport<A>,
            Self: 'static + Sized + Send + Sync + Checkable
{
    /// Listens to connections incoming from one or more network addresses.
    fn listen(address: &A) -> Result<Self>;

    /// Accepts connections incoming from one or more network addresses.
    fn accept(&mut self) -> Result<(CT, A)>;

    /// Closes the connections.
    fn close(&mut self) -> Result<()>;

    /// Eval operation in the server transport.
    fn eval<Ev: Eval<Self, P, R>, P: Datable, R: Datable>(&self, params: &P, evaluator: &Ev) -> Result<R> {
        self.check()?;

        params.check()?;

        evaluator.eval(self, params)
    }

    /// Mutable eval operation in the server transport.
    fn eval_mut<EvM: EvalMut<Self, P, R>, P: Datable, R: Datable>(&mut self, params: &P, evaluator: &mut EvM) -> Result<R> {
        self.check()?;

        params.check()?;

        let result = evaluator.eval_mut(self, params)?;
        self.check()?;

        Ok(result)
    }
}