//! # App
//!
//! `app` is the module providing the trait implemented by I/O applications.

use base::Result;
use base::Future;
use base::data::Datable;
use io::app::{Request, Response};
use io::app::{RequestSender, ResponseSender};

/// Trait implemented by I/O application types.
pub trait App<Ap, StaP, StaR, StoP, StoR, RP, RR, EP, ER>
    where   Ap: Datable,
            StaP: Datable,
            StaR: Datable,
            StoP: Datable,
            StoR: Datable,
            RP: Datable,
            RR: Datable,
            EP: Datable,
            ER: Datable,
            Self: 'static + Sized + Send + Sync
{
    /// Returns the `App` identifier.
    fn app_id(&self) -> Ap;

    /// Creates the `App`.
    fn create<P: Datable>(&mut self, params: &P) -> Result<Self>;

    /// Returns the `App` `RequestSender`.
    fn request_sender(&self) -> RequestSender<Ap, StaP, StoP, RP, EP>;

    /// Sets the `App` `ResponseSender`.
    fn response_sender(&mut self, sender: &ResponseSender<Ap, StaR, StoR, RR, ER>) -> Result<()>;

    /// Starts the `App`.
    fn start<P: Datable>(&mut self, params: &P, req: &Request<Ap, StaP, StoP, RP, EP>)
        -> Future<Response<Ap, StaR, StoR, RR, ER>>;

    /// Stops the `App`.
    fn stop<P: Datable>(&mut self, params: &P, req: &Request<Ap, StaP, StoP, RP, EP>)
        -> Future<Response<Ap, StaR, StoR, RR, ER>>;

    /// Restarts the `App`.
    fn restart<P: Datable>(&mut self, params: &P, req: &Request<Ap, StaP, StoP, RP, EP>)
        -> Future<Response<Ap, StaR, StoR, RR, ER>>;

    /// Execs a custom command in the `App`.
    fn exec<P: Datable>(&mut self, params: &P, req: &Request<Ap, StaP, StoP, RP, EP>)
        -> Future<Response<Ap, StaR, StoR, RR, ER>>;
}