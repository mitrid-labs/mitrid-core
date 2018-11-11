//! # Router
//!
//! `router` is the module providing the trait implemented by the server router.

use base::Result;
use base::size::ConstantSize;
use base::Checkable;
use base::Serializable;
use base::Datable;
use base::{Eval, EvalMut};
use io::store::Store;
use io::network::message::Method;
use io::network::message::Request;
use io::network::message::Response;
use io::network::server::Handler;

/// Trait implemented by the server router.
pub trait Router<St, StS, S, D, MP, H>
    where   St: Store<StS>,
            StS: Datable + Serializable,
            S: Datable,
            D: Ord + Datable + ConstantSize,
            MP: Datable,
            H: Handler<St, StS, S, D, MP>,
            Self: 'static + Sized + Clone + Send + Sync
{
    /// Returns the middleware callbacks applied sequentially by the router. Each callback takes as parameters
    /// the results of the preceding one.
    fn middlewares(&mut self)
        -> Result<Vec<Box<FnMut(&mut Self, &Request<S, D, MP>)
                        -> Result<(Request<S, D, MP>)>>>> 
    {
            Ok(vec![])
    }

    /// Routes an incoming request to the right handler.
    fn route<Ev, EvM>(&mut self,
                     store: &mut St,
                     handler: &mut H,
                     request: &Request<S, D, MP>,
                     evaluator: &Ev,
                     evaluator_mut: &mut EvM)
        -> Result<Response<S, D, MP>>
        where   Ev: Eval<St, Request<S, D, MP>, Response<S, D, MP>>,
                EvM: EvalMut<St, Request<S, D, MP>, Response<S, D, MP>>
    {
        request.check()?;

        let mut request = request.to_owned();

        for mut cb in self.middlewares()? {
            request = cb(self, &mut request)?;
        }

        for mut cb in handler.middlewares(store)? {
            request = cb(handler, store, &mut request)?;
        }

        match request.message.method {
            Method::Ping => {
                let response = handler.handle_ping(store, &mut request)?;

                response.check()?;

                if response.message.method != Method::Ping {
                    return Err(String::from("invalid method"));
                }

                Ok(response)
            },
            Method::Session => {
                let response = handler.handle_session(store, &mut request)?;

                response.check()?;

                if response.message.method != Method::Session {
                    return Err(String::from("invalid method"));
                }

                Ok(response)
            },
            Method::Count => {
                let response = handler.handle_count(store, &mut request)?;

                response.check()?;

                if response.message.method != Method::Count {
                    return Err(String::from("invalid method"));
                }

                Ok(response)
            },
            Method::List => {
                let response = handler.handle_list(store, &mut request)?;

                response.check()?;

                if response.message.method != Method::List {
                    return Err(String::from("invalid method"));
                }

                Ok(response)
            },
            Method::Lookup => {
                let response = handler.handle_lookup(store, &mut request)?;

                response.check()?;

                if response.message.method != Method::Lookup {
                    return Err(String::from("invalid method"));
                }

                Ok(response)
            },
            Method::Get => {
                let response = handler.handle_get(store, &mut request)?;

                response.check()?;

                if response.message.method != Method::Get {
                    return Err(String::from("invalid method"));
                }

                Ok(response)
            },
            Method::Create => {
                let response = handler.handle_create(store, &mut request)?;

                response.check()?;

                if response.message.method != Method::Create {
                    return Err(String::from("invalid method"));
                }

                Ok(response)
            },
            Method::Update => {
                let response = handler.handle_update(store, &mut request)?;

                response.check()?;

                if response.message.method != Method::Update {
                    return Err(String::from("invalid method"));
                }

                Ok(response)
            },
            Method::Upsert => {
                let response = handler.handle_upsert(store, &mut request)?;

                response.check()?;

                if response.message.method != Method::Upsert {
                    return Err(String::from("invalid method"));
                }

                Ok(response)
            },
            Method::Delete => {
                let response = handler.handle_delete(store, &mut request)?;

                response.check()?;

                if response.message.method != Method::Delete {
                    return Err(String::from("invalid method"));
                }

                Ok(response)
            },
            Method::Eval => {
                let response = handler.handle_eval(store, &mut request, evaluator)?;

                response.check()?;

                if response.message.method != Method::Eval {
                    return Err(String::from("invalid method"));
                }

                Ok(response)
            },
            Method::EvalMut => {
                let response = handler.handle_eval_mut(store, &mut request, evaluator_mut)?;

                response.check()?;

                if response.message.method != Method::EvalMut {
                    return Err(String::from("invalid method"));
                }

                Ok(response)
            }
        }
    }
}