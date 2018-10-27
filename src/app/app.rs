//! # App
//!
//! `app` is the module providing the trait implemented by Mitrid applications.

use futures::Future as BasicFuture;
use futures::Stream;

use base::Result;
use base::Future;
use base::data::Datable;
use app::{Request, Response};
use app::{RequestChannel, ResponseSender};
use app::Logger;

/// Trait implemented by Mitrid application types.
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
            Self: 'static + Sized + Send + Sync + Logger
{
    /// Returns the `App` `RequestChannel`.
    fn request_channel(&self) -> RequestChannel<Ap, StaP, StoP, RP, EP>;

    /// Returns the `App` `ResponseSender`.
    fn response_sender(&self) -> ResponseSender<Ap, StaR, StoR, RR, ER>;

    /// Executes a command in the `App`.
    fn exec(&mut self, req: &Request<Ap, StaP, StoP, RP, EP>)
        -> Future<Response<Ap, StaR, StoR, RR, ER>>;

    /// Logs a result.
    fn log_result<T: Sized>(&self, res: &Result<T>);

    /// Runs the `App`.
    fn run(&mut self) {
        let mut sender = self.response_sender();

        loop {
            for req_res in self.request_channel().receiver.wait() {
                let req = req_res.unwrap();

                let res = self.exec(&req)
                            .or_else(|e| {
                                let err = Err(format!("{:?}", e));
                                self.log_result(&err);
                                err
                            })
                            .and_then(|res| {
                                sender
                                    .try_send(res)
                                    .map_err(|e| format!("{:?}", e))
                            })
                            .wait();

                self.log_result(&res);
                res.unwrap();
            }
        }   
    }
}