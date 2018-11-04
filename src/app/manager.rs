//! # Manager
//!
//! `manager` is the module providing the trait used to manage Mitrid applications.

use base::Result;
use base::{ConstantSize, VariableSize};
use base::Datable;
use app::command::{Request, Response};
use app::{RequestSender, ResponseReceiver};
use app::{Env, Config, Logger};

/// Trait implemented by Mitrid application managers.
pub trait Manager<E, C, D, MnP, A, StP, SvP, ClP, CP, P, R>
    where   E: Env,
            C: Config<D, MnP, A, StP, SvP, ClP, CP>,
            D: Datable + ConstantSize,
            MnP: Datable,
            A: Ord + Datable + VariableSize,
            StP: Datable,
            SvP: Datable,
            ClP: Datable,
            CP: Datable,
            P: Datable,
            R: Datable,
            Self: Sized + Logger
{
    /// Inits the `Manager`.
    fn init() -> Result<()>;

    /// Creates an `App`.
    fn create_app(&mut self, env: &E, config: &C, app: &A) -> Result<()>;

    /// Lookups for an `App`.
    fn lookup_app(&self, app: &A) -> Result<bool>;

    /// Gets an `App` `RequestSender`.
    fn app_request_sender(&self, app: &A) -> Result<RequestSender<A, P>>;

    /// Gets an `App` `ResponseReceiver`.
    fn app_response_receiver(&self, app: &A) -> Result<ResponseReceiver<A, R>>;

    /// Logs a `Result`.
    fn log_result<T: Sized>(&self, res: &Result<T>);

    /// Logs a command response.
    fn log_response(&self, req: &Response<A, R>);

    /// Executes a command request.
    fn exec(&mut self, env: &E, config: &C, req: &Request<A, P>) {
        Self::init().unwrap();
        
        let config_check = config.check();
        self.log_result(&config_check);

        let address = req.address.clone();

        let lookup_res = self.lookup_app(&address);
        self.log_result(&lookup_res);

        if lookup_res.unwrap() {
            let create_app_res = self.create_app(env, config, &address);
            self.log_result(&create_app_res);
            
            create_app_res.unwrap();

            return self.exec(env, config, req);
        }

        let sender_res = self.app_request_sender(&address);
        self.log_result(&sender_res);

        let sender = sender_res.unwrap();

        let res = sender
                    .send(req.to_owned())
                    .map_err(|e| format!("{:?}", e));

        self.log_result(&res);

        res.unwrap();

        let responses_res = self.app_response_receiver(&address);
        self.log_result(&responses_res);

        let responses = responses_res.unwrap();

        for res in responses {
            self.log_response(&res);
        }
    }
}