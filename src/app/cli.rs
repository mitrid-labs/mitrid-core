//! # CLI
//!
//! `cli` is the module providing the trait used to manage and interact with the framework applications
//! from the command line.

use std::collections::HashMap;

use base::Result;
use base::{ConstantSize, VariableSize};
use base::Datable;
use app::command::Request;
use app::{Env, Config, Logger, Manager};

/// Trait implemented by CLI types.
pub trait CLI<M, E, C, L, D, MnP, A, StP, SvP, ClP, CP, Ap, StaP, StaR, StoP, StoR, RP, RR, EP, ER>
    where   M: Manager<E, C, D, MnP, A, StP, SvP, ClP, CP, Ap, StaP, StaR, StoP, StoR, RP, RR, EP, ER>,
            E: Env,
            C: Config<D, MnP, A, StP, SvP, ClP, CP>,
            L: Logger,
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
            ER: Datable
{
    /// Returns the CLI environment.
    fn env(&self) -> E;

    /// Returns the CLI configurations.
    fn config(&self) -> C;

    /// Returns the CLI logger.
    fn logger(&self) -> L;

    /// Creates a `Manager`.
    fn create_manager(&mut self, manager_params: &MnP) -> Result<()>;
    
    /// Returns the apps `Manager`, if present.
    fn manager(&self) -> Option<M>;
    
    /// Logs a `Result`.
    fn log_result<T: Sized>(&mut self, res: &Result<T>);

    /// Parses CLI args.
    fn parse_vars_and_args(&mut self, vars: &HashMap<String, String>, args: &Vec<String>) -> Result<Request<Ap, StaP, StoP, RP, EP>>;

    /// Parses the CLI command.
    fn parse_cmd(&mut self) -> Result<Request<Ap, StaP, StoP, RP, EP>> {
        let res_vars = self.env().vars();
        self.log_result(&res_vars);

        let res_args = self.env().args();
        self.log_result(&res_args);

        let vars = res_vars?;
        let args = res_args?;
        self.parse_vars_and_args(&vars, &args)
    }
    
    /// Runs the CLI.
    fn run(&mut self) {
        let env = self.env();

        let config = self.config();
        let config_check = config.check();
        self.log_result(&config_check);

        let res_req = self.parse_cmd();
        self.log_result(&res_req);

        let req = res_req.unwrap();

        if let Some(mut manager) = self.manager() {
            manager.exec(&env, &config, &req)
        } else {
            let res = self.create_manager(&config.manager_params());
            self.log_result(&res);
            res.unwrap();

            self.manager().unwrap().exec(&env, &config, &req)
        }
    }
}