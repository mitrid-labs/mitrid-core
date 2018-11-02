//! # Manager
//!
//! `manager` is the module providing the trait used to manage Mitrid applications.

use base::Result;
use base::{ConstantSize, VariableSize};
use base::Checkable;
use base::Datable;
use app::command::{Request, Response};
use app::{RequestSender, ResponseReceiver};
use app::{Env, Config, Logger};

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
    /// Inits the `Manager`.
    fn init() -> Result<()>;

    /// Creates an `App`.
    fn create_app(&mut self, env: &E, config: &Config<D, MnP, A, StP, SvP, ClP, CP>, app: &Ap) -> Result<()>;

    /// Lookups for an `App`.
    fn lookup_app(&self, app: &Ap) -> Result<bool>;

    /// Gets an `App` `RequestSender`.
    fn app_request_sender(&self, app: &Ap) -> Result<RequestSender<Ap, StaP, StoP, RP, EP>>;

    /// Gets an `App` `ResponseReceiver`.
    fn app_response_receiver(&self, app: &Ap) -> Result<ResponseReceiver<Ap, StaR, StoR, RR, ER>>;

    /// Logs a `Result`.
    fn log_result<T: Sized>(&self, res: &Result<T>);

    /// Logs a command response.
    fn log_response(&self, req: &Response<Ap, StaR, StoR, RR, ER>);

    /// Executes a command request.
    fn exec(&mut self, env: &E, config: &Config<D, MnP, A, StP, SvP, ClP, CP>, req: &Request<Ap, StaP, StoP, RP, EP>) {
        Self::init().unwrap();
        
        let config_check = config.check();
        self.log_result(&config_check);

        let app_opt = match req {
            &Request::None => None,
            &Request::Start { ref app, .. } => Some(app),
            &Request::Stop { ref app, .. } => Some(app),
            &Request::Restart { ref app, .. } => Some(app),
            &Request::Exec { ref app, .. } => Some(app),
        };

        if app_opt.is_none() {
            let res = Response::None;
            self.log_response(&res);
        }

        let app = app_opt.unwrap();

        let lookup_res = self.lookup_app(app);
        self.log_result(&lookup_res);

        if lookup_res.unwrap() {
            let create_app_res = self.create_app(env, config, app);
            self.log_result(&create_app_res);
            
            create_app_res.unwrap();

            return self.exec(env, config, req);
        }

        let sender_res = self.app_request_sender(app);
        self.log_result(&sender_res);

        let sender = sender_res.unwrap();

        let res = sender
                    .send(req.to_owned())
                    .map_err(|e| format!("{:?}", e));

        self.log_result(&res);

        res.unwrap();

        let responses_res = self.app_response_receiver(app);
        self.log_result(&responses_res);

        let responses = responses_res.unwrap();

        for res in responses {
            self.log_response(&res);
        }
    }
}