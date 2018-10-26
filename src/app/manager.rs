//! # Manager
//!
//! `manager` is the module providing the trait used to manage Mitrid applications.

use futures::Future as BasicFuture;

use std::collections::HashMap;

use base::Result;
use base::Future;
use base::{ConstantSize, VariableSize};
use base::Datable;
use app::command::{Request, Response};
use app::{RequestSender, ResponseSender};
use app::{Env, Logger, Config};

/// Trait implemented by Mitrid application managers.
pub trait Manager<E, D, MnP, A, StP, SvP, ClP, CP, Ap, StaP, StaR, StoP, StoR, RP, RR, EP, ER>
    where   E: Env,
            D: Datable + ConstantSize,
            MnP: Datable,
            A: Datable + VariableSize,
            StP: Datable,
            SvP: Datable,
            ClP: Datable,
            CP: Datable,
            Ap: Datable,
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
    /// Starts the `Manager`.
    fn start(&mut self, params: &MnP, env: &E, config: &Config<D, MnP, A, StP, SvP, ClP, CP>) -> Result<Self>;

    /// Returns the current environemnt.
    fn env(&self) -> Result<E>;

    /// Returns the `Manager` `ResponseSender`.
    fn response_sender(&self) -> ResponseSender<Ap, StaR, StoR, RR, ER>;

    /// Returns the `Manager` `RequestSender`s.
    fn request_senders(&self) -> Vec<(Ap, RequestSender<Ap, StaP, StoP, RP, EP>)>;

    /// Adds to the `Manager` a `RequestSender`.
    fn add_request_sender(&mut self, app: &Ap, sender: &RequestSender<Ap, StaP, StoP, RP, EP>) -> Result<()>;

    /// Runs an `App`.
    fn run_app<P: Datable>(&mut self, params: &P) -> Result<HashMap<Ap, RequestSender<Ap, StaP, StoP, RP, EP>>>;

    /// Executes a command in the `App`.
    fn exec_app(&mut self, req: &Request<Ap, StaP, StoP, RP, EP>) -> Future<Response<Ap, StaR, StoR, RR, ER>>;

    /// Logs a command response.
    fn log_response(&self, req: &Response<Ap, StaR, StoR, RR, ER>);

    /// Logs a `Result`.
    fn log_result<T: Sized>(&self, res: &Result<T>);

    /// Executes a command.
    fn exec_cmd(&mut self, req: &Request<Ap, StaP, StoP, RP, EP>) {
        let res = self.exec_app(req).wait();

        self.log_result(&res);
        self.log_response(&res.unwrap())
    }
}