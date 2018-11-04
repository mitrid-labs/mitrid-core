//! # App
//!
//! `app` is the module providing the trait implemented by Mitrid applications.

use base::Result;
use base::data::Datable;
use app::{Request, Response};
use app::{RequestChannel, ResponseSender};
use app::Logger;

/// Trait implemented by Mitrid application types.
pub trait App<A, P, R>
    where   A: Ord + Datable,
            P: Datable,
            R: Datable,
            Self: 'static + Sized + Send + Sync + Logger
{
    /// Returns the `App` address.
    fn address(&self) -> A;

    /// Returns the `App` `RequestChannel`.
    fn request_channel(&self) -> RequestChannel<A, P>;

    /// Returns the `App` `ResponseSender`.
    fn response_sender(&self) -> ResponseSender<A, R>;

    /// Executes a command in the `App`.
    fn exec(&mut self, req: &Request<A, P>)
        -> Result<Response<A, R>>;

    /// Logs a result.
    fn log_result<T: Sized>(&self, res: &Result<T>);

    /// Runs the `App`.
    fn run(&mut self) {
        let sender = self.response_sender();

        loop {
            for req in self.request_channel().receiver {

                let res = self.exec(&req)
                            .or_else(|e| {
                                let err = Err(format!("{:?}", e));
                                self.log_result(&err);
                                err
                            })
                            .and_then(|res| {
                                sender
                                    .send(res)
                                    .map_err(|e| format!("{:?}", e))
                            });

                self.log_result(&res);
                res.unwrap();
            }
        }   
    }
}