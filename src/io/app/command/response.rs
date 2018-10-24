//! # Response
//!
//! `response` is the module providing the type representing an I/O command response.

use base::Result;
use base::Sizable;
use base::Checkable;
use base::Serializable;
use base::Datable;

/// Type used to represent an I/O command response.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
#[allow(unused_attributes)]
pub enum Response<Ap, StaR, StoR, RR, ER>
    where   Ap: Datable,
            StaR: Datable,
            StoR: Datable,
            RR: Datable,
            ER: Datable,
{
    /// An empty response.
    #[repr(u8)]
    None,
    /// Replies to the resuest of starting an I/O application.
    Start { app: Ap, result: Option<StaR>, error: Option<String> },
    /// Replies to the resuest of stopping an I/O application.
    Stop { app: Ap, result: Option<StoR>, error: Option<String> },
    /// Replies to the resuest of restarting an I/O application.
    Restart { app: Ap, result: Option<RR>, error: Option<String> },
    /// Replies to the resuest of executing an operation on an I/O application.
    Exec { app: Ap, result: Option<ER>, error: Option<String> },
}

impl<Ap, StaR, StoR, RR, ER> Response<Ap, StaR, StoR, RR, ER>
    where   Ap: Datable,
            StaR: Datable,
            StoR: Datable,
            RR: Datable,
            ER: Datable,
{
    /// Creates a new none `Response`.
    pub fn new_none() -> Self {
        Response::None
    }

    /// Creates a new start `Response`.
    pub fn new_start(app: &Ap, result: &Option<StaR>, error: &Option<String>) -> Result<Self> {
        app.check()?;
        result.check()?;
        error.check()?;

        if error.is_some() && result.is_some() {
            return Err(String::from("invalid result"));
        }

        let res = Response::Start {
            app: app.to_owned(),
            result: result.to_owned(),
            error: error.to_owned(),
        };

        Ok(res)
    }

    /// Creates a new stop `Response`.
    pub fn new_stop(app: &Ap, result: &Option<StoR>, error: &Option<String>) -> Result<Self> {
        app.check()?;
        result.check()?;
        error.check()?;

        if error.is_some() && result.is_some() {
            return Err(String::from("invalid result"));
        }

        let res = Response::Stop {
            app: app.to_owned(),
            result: result.to_owned(),
            error: error.to_owned(),
        };

        Ok(res)
    }
    
    /// Creates a new restart `Response`.
    pub fn new_restart(app: &Ap, result: &Option<RR>, error: &Option<String>) -> Result<Self> {
        app.check()?;
        result.check()?;
        error.check()?;

        if error.is_some() && result.is_some() {
            return Err(String::from("invalid result"));
        }

        let res = Response::Restart {
            app: app.to_owned(),
            result: result.to_owned(),
            error: error.to_owned(),
        };

        Ok(res)
    }
    
    /// Creates a new exec `Response`.
    pub fn new_exec(app: &Ap, result: &Option<ER>, error: &Option<String>) -> Result<Self> {
        app.check()?;
        result.check()?;
        error.check()?;

        if error.is_some() && result.is_some() {
            return Err(String::from("invalid result"));
        }

        let res = Response::Exec {
            app: app.to_owned(),
            result: result.to_owned(),
            error: error.to_owned(),
        };

        Ok(res)
    }
}

impl<Ap, StaR, StoR, RR, ER> Default for Response<Ap, StaR, StoR, RR, ER>
    where   Ap: Datable,
            StaR: Datable,
            StoR: Datable,
            RR: Datable,
            ER: Datable,
{
    fn default() -> Self {
        Response::None
    }
}

impl<Ap, StaR, StoR, RR, ER> Sizable for Response<Ap, StaR, StoR, RR, ER>
    where   Ap: Datable,
            StaR: Datable,
            StoR: Datable,
            RR: Datable,
            ER: Datable,
{
    fn size(&self) -> u64 {
        match self {
            &Response::None => 1,
            &Response::Start { ref app, ref result, ref error } => {
                app.size() + result.size() + error.size()
            },
            &Response::Stop { ref app, ref result, ref error } => {
                app.size() + result.size() + error.size()
            },
            &Response::Restart { ref app, ref result, ref error } => {
                app.size() + result.size() + error.size()
            },
            &Response::Exec { ref app, ref result, ref error } => {
                app.size() + result.size() + error.size()
            },
        }
    }
}

impl<Ap, StaR, StoR, RR, ER> Checkable for Response<Ap, StaR, StoR, RR, ER>
    where   Ap: Datable,
            StaR: Datable,
            StoR: Datable,
            RR: Datable,
            ER: Datable,
{
    fn check(&self) -> Result<()> {
        match self {
            &Response::None => Ok(()),
            &Response::Start { ref app, ref result, ref error } => {
                app.check()?;
                result.check()?;
                error.check()?;

                if error.is_some() && result.is_some() {
                    return Err(String::from("invalid result"));
                }

                Ok(())
            },
            &Response::Stop { ref app, ref result, ref error } => {
                app.check()?;
                result.check()?;
                error.check()?;

                if error.is_some() && result.is_some() {
                    return Err(String::from("invalid result"));
                }

                Ok(())
            },
            &Response::Restart { ref app, ref result, ref error } => {
                app.check()?;
                result.check()?;
                error.check()?;

                if error.is_some() && result.is_some() {
                    return Err(String::from("invalid result"));
                }

                Ok(())
            },
            &Response::Exec { ref app, ref result, ref error } => {
                app.check()?;
                result.check()?;
                error.check()?;

                if error.is_some() && result.is_some() {
                    return Err(String::from("invalid result"));
                }

                Ok(())
            },
        }
    }
}

impl<Ap, StaR, StoR, RR, ER> Serializable for Response<Ap, StaR, StoR, RR, ER>
    where   Ap: Datable + Serializable,
            StaR: Datable + Serializable,
            StoR: Datable + Serializable,
            RR: Datable + Serializable,
            ER: Datable + Serializable,
{}

impl<Ap, StaR, StoR, RR, ER> Datable for Response<Ap, StaR, StoR, RR, ER>
    where   Ap: Datable,
            StaR: Datable,
            StoR: Datable,
            RR: Datable,
            ER: Datable,
{}