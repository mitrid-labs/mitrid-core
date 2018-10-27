//! # CLI
//!
//! `cli` is the module providing the trait used to manage and interact with the framework applications
//! from the command line.

use std::collections::HashMap;

use base::Result;
use base::{ConstantSize, VariableSize};
use base::Checkable;
use base::Datable;
use app::command::Request;
use app::{Env, Config, Logger, Manager};

/// Trait implemented by CLI types.
pub trait CLI<M, E, D, MnP, A, StP, SvP, ClP, CP, Ap, StaP, StaR, StoP, StoR, RP, RR, EP, ER>
    where   M: Manager<E, D, MnP, A, StP, SvP, ClP, CP, Ap, StaP, StaR, StoP, StoR, RP, RR, EP, ER>,
            E: Env,
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
            Self: Sized + Env + Logger
{
    /// Returns the CLI environment.
    fn env(&self) -> E;

    /// Returns the CLI configurations.
    fn config(&self) -> Config<D, MnP, A, StP, SvP, ClP, CP>;
    
    /// Returns the CLI manager.
    fn manager(&self) -> M;
    
    /// Logs a `Result`.
    fn log_result<T: Sized>(&mut self, res: &Result<T>);

    /// Parses CLI args.
    fn parse_args(&mut self, args: &HashMap<String, String>) -> Result<Request<Ap, StaP, StoP, RP, EP>>;

    /// Parses the CLI command.
    fn parse_cmd(&mut self) -> Result<Request<Ap, StaP, StoP, RP, EP>> {
        let res_args = self.env().args();
        self.log_result(&res_args);

        let args = res_args?;
        self.parse_args(&args)
    }
    
    /// Runs the CLI.
    fn run(&mut self) {
        let env = self.env();
        let env_check = env.check();
        self.log_result(&env_check);

        let config = self.config();
        let config_check = config.check();
        self.log_result(&config_check);

        let res_req = self.parse_cmd();
        self.log_result(&res_req);

        let req = res_req.unwrap();
        self.manager().exec(&env, &config, &req)
    }
}