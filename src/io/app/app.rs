use base::Result;
use base::Future;
use base::data::Datable;
use io::app::config::Config;
use io::app::command::{Request, Response};

/// Trait implemented by I/O application types.
pub trait App<Ap, C, StaP, StaR, StoP, StoR, RP, RR, EP, ER>
    where   Ap: Datable,
            C: Config,
            StaP: Datable,
            StaR: Datable,
            StoP: Datable,
            StoR: Datable,
            RP: Datable,
            RR: Datable,
            EP: Datable,
            ER: Datable,
{
    /// Returns the application identifier
    fn app_id(&self) -> Ap;

    /// Configs the application.
    fn config(&mut self, config: &C) -> Result<()>;

    /// Starts the application.
    fn start(&mut self, config: &C, req: &Request<Ap, C, StaP, StoP, RP, EP>)
        -> Future<Response<Ap, StaR, StoR, RR, ER>>;

    /// Stops the application.
    fn stop(&mut self, req: &Request<Ap, C, StaP, StoP, RP, EP>)
        -> Future<Response<Ap, StaR, StoR, RR, ER>>;

    /// Restarts the application.
    fn restart(&mut self, config: &C, req: &Request<Ap, C, StaP, StoP, RP, EP>)
        -> Future<Response<Ap, StaR, StoR, RR, ER>>;

    /// Execs a custom command.
    fn exec(&mut self, req: &Request<Ap, C, StaP, StoP, RP, EP>)
        -> Future<Response<Ap, StaR, StoR, RR, ER>>;
}