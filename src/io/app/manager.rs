//! # Manager
//!
//! `manager` is the module providing the trait used to manage I/O applications.

use base::Result;
use base::Future;
use base::Datable;
use io::app::{RequestSender, ResponseSender};
use io::app::Logger;

/// Trait implemented by I/O applications managers.
pub trait Manager<Ap, StaP, StaR, StoP, StoR, RP, RR, EP, ER>
    where   Ap: Datable,
            StaP: Datable,
            StaR: Datable,
            StoP: Datable,
            StoR: Datable,
            RP: Datable,
            RR: Datable,
            EP: Datable,
            ER: Datable,
            Self: Sized + Logger
{
    /// Creates the `Manager`.
    fn create<P: Datable>(&mut self, params: &P) -> Result<Self>;

    /// Returns the `Manager` `ResponseSender`.
    fn response_sender(&self) -> ResponseSender<Ap, StaR, StoR, RR, ER>;

    /// Returns the `Manager` `RequestSender`s.
    fn request_senders(&self) -> Vec<(Ap, RequestSender<Ap, StaP, StoP, RP, EP>)>;

    /// Adds to the `Manager` a `RequestSender`.
    fn add_request_sender(&mut self, app: &Ap, sender: &RequestSender<Ap, StaP, StoP, RP, EP>) -> Result<()>;

    /// Creates an `App`.
    fn create_app<P: Datable>(&mut self, params: &P) -> Result<(Ap, RequestSender<Ap, StaP, StoP, RP, EP>)>;

    /// Starts an `App`.
    fn start_app<P: Datable>(&mut self, params: &P, app: &Ap, start_params: &StaP) -> Future<StaR>;

    /// Stops the `App`.
    fn stop_app<P: Datable>(&mut self, params: &P, app: &Ap, stop_params: &StoP) -> Future<StoR>;

    /// Restarts the `App`.
    fn restart_app<P: Datable>(&mut self, params: &P, app: &Ap, restart_params: &RP) -> Future<RR>;

    /// Execs a custom command in the `App`.
    fn exec_app<P: Datable>(&mut self, params: &P, app: &Ap, exec_params: &EP) -> Future<ER>;
}