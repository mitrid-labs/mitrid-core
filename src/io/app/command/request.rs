//! # Request
//!
//! `request` is the module providing the type representing an I/O command request.

use base::Result;
use base::Sizable;
use base::Checkable;
use base::Serializable;
use base::Datable;
use io::config::Config;

/// Type used to represent an I/O command request.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
#[allow(unused_attributes)]
pub enum Request<Ap, C, StaP, StoP, RP, EP>
    where   Ap: Datable,
            C: Config,
            StaP: Datable,
            StoP: Datable,
            RP: Datable,
            EP: Datable,
{
    /// An empty request.
    #[repr(u8)]
    None,
    /// Requests to starts an I/O application.
    Start { app: Ap, config: C, params: StaP },
    /// Requests to stops an I/O application.
    Stop { app: Ap, params: StoP },
    /// Requests to restart an I/O application.
    Restart { app: Ap, config: C, params: RP },
    /// Request to exec an operation on an I/O application.
    Exec { app: Ap, params: EP },
}

impl<Ap, C, StaP, StoP, RP, EP> Request<Ap, C, StaP, StoP, RP, EP>
    where   Ap: Datable,
            C: Config,
            StaP: Datable,
            StoP: Datable,
            RP: Datable,
            EP: Datable,
{
    /// Creates a new none `Request`.
    pub fn new_none() -> Self {
        Request::None
    }

    /// Creates a new start `Request`.
    pub fn new_start(app: &Ap, config: &C, params: &StaP) -> Result<Self> {
        app.check()?;
        config.check()?;
        params.check()?;

        let req = Request::Start {
            app: app.to_owned(),
            config: config.to_owned(),
            params: params.to_owned(),
        };

        Ok(req)
    }

    /// Creates a new stop `Request`.
    pub fn new_stop(app: &Ap, params: &StoP) -> Result<Self> {
        app.check()?;
        params.check()?;

        let req = Request::Stop {
            app: app.to_owned(),
            params: params.to_owned(),
        };

        Ok(req)
    }
    
    /// Creates a new restart `Request`.
    pub fn new_restart(app: &Ap, config: &C, params: &RP) -> Result<Self> {
        app.check()?;
        config.check()?;
        params.check()?;

        let req = Request::Restart {
            app: app.to_owned(),
            config: config.to_owned(),
            params: params.to_owned(),
        };

        Ok(req)
    }
    
    /// Creates a new exec `Request`.
    pub fn new_exec(app: &Ap, params: &EP) -> Result<Self> {
        app.check()?;
        params.check()?;

        let req = Request::Exec {
            app: app.to_owned(),
            params: params.to_owned(),
        };

        Ok(req)
    }
}

impl<Ap, C, StaP, StoP, RP, EP> Default for Request<Ap, C, StaP, StoP, RP, EP>
    where   Ap: Datable,
            C: Config,
            StaP: Datable,
            StoP: Datable,
            RP: Datable,
            EP: Datable,
{
    fn default() -> Self {
        Request::None
    }
}

impl<Ap, C, StaP, StoP, RP, EP> Sizable for Request<Ap, C, StaP, StoP, RP, EP>
    where   Ap: Datable,
            C: Config,
            StaP: Datable,
            StoP: Datable,
            RP: Datable,
            EP: Datable,
{
    fn size(&self) -> u64 {
        match self {
            &Request::None => 1,
            &Request::Start { ref app, ref config, ref params } => {
                app.size() + config.size() + params.size()
            },
            &Request::Stop { ref app, ref params } => {
                app.size() + params.size()
            },
            &Request::Restart { ref app, ref config, ref params } => {
                app.size() + config.size() + params.size()
            },
            &Request::Exec { ref app, ref params } => {
                app.size() + params.size()
            },
        }
    }
}

impl<Ap, C, StaP, StoP, RP, EP> Checkable for Request<Ap, C, StaP, StoP, RP, EP>
    where   Ap: Datable,
            C: Config,
            StaP: Datable,
            StoP: Datable,
            RP: Datable,
            EP: Datable,
{
    fn check(&self) -> Result<()> {
        match self {
            &Request::None => Ok(()),
            &Request::Start { ref app, ref config, ref params } => {
                app.check()?;
                config.check()?;
                params.check()
            },
            &Request::Stop { ref app, ref params } => {
                app.check()?;
                params.check()
            },
            &Request::Restart { ref app, ref config, ref params } => {
                app.check()?;
                config.check()?;
                params.check()
            },
            &Request::Exec { ref app, ref params } => {
                app.check()?;
                params.check()
            },
        }
    }
}

impl<Ap, C, StaP, StoP, RP, EP> Serializable for Request<Ap, C, StaP, StoP, RP, EP>
    where   Ap: Datable + Serializable,
            C: Config + Serializable,
            StaP: Datable + Serializable,
            StoP: Datable + Serializable,
            RP: Datable + Serializable,
            EP: Datable + Serializable,
{}

impl<Ap, C, StaP, StoP, RP, EP> Datable for Request<Ap, C, StaP, StoP, RP, EP>
    where   Ap: Datable,
            C: Config,
            StaP: Datable,
            StoP: Datable,
            RP: Datable,
            EP: Datable,
{}