//! # Router
//!
//! `router` is the module providing the trait implemented by the server router.

use base::Result;
use base::size::{ConstantSize, VariableSize};
use base::numerical::Numerical;
use base::Checkable;
use base::Serializable;
use base::Datable;
use io::store::Store;
use io::network::message::Method;
use io::network::message::Request;
use io::network::message::Response;
use io::network::server::Handler;

/// Trait implemented by the server router.
pub trait Router<St, StS, StK, StV, StP, StPC, StRC, S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   St: Store<StS, StP, StPC, StRC>,
            StS: Datable + Serializable,
            StK: Ord + Datable + Serializable,
            StV: Datable + Serializable,
            StP: Datable,
            StPC: Datable + Serializable,
            StRC: Datable + Serializable,
            StS: Datable + Serializable,
            StK: Ord + Datable + Serializable,
            StV: Datable + Serializable,
            S: Datable,
            RS: Datable,
            Ad: Ord + Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            Pr: Datable,
            Am: Numerical,
            IP: Datable,
            OP: Datable,
            TP: Datable,
            BP: Datable,
            BGP: Datable,
            C: Datable,
            Self: 'static + Clone + Send + Sync
{
    /// Returns the middleware callbacks applied sequentially by the router. Each callback takes as parameters
    /// the results of the preceding one.
    fn middlewares<P: Datable>(&self, params: &P)
        -> Result<Vec<Box<Fn(&P, &Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>)
                        -> Result<(P, Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>)>>>> 
    {
            params.check()?;

            Ok(vec![])
    }

    /// Routes an incoming request to the right handler.
    fn route<H, P>(&self,
                   store: &mut St,
                   handler: &H,
                   params: &P,
                   request: &Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>)
        -> Result<Response<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>>
        where   H: Handler<St, StS, StK, StV, StP, StPC, StRC, S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>,
                P: Datable
    {
        params.check()?;
        request.check()?;

        let mut middle_res = (params.to_owned(), request.to_owned());

        for cb in self.middlewares(&middle_res.0)? {
            middle_res = cb(&middle_res.0, &middle_res.1)?;
        }

        for cb in handler.middlewares(store, &middle_res.0)? {
            middle_res = cb(store, &middle_res.0, &middle_res.1)?;
        }

        match request.method() {
            Method::Ping => {
                let response = handler.handle_ping(store, &middle_res.0, &middle_res.1)?;

                response.check()?;

                if response.method() != Method::Ping {
                    return Err(String::from("invalid method"));
                }

                Ok(response)
            },
            Method::Session => {
                let response = handler.handle_session(store, &middle_res.0, &middle_res.1)?;

                response.check()?;

                if response.method() != Method::Session {
                    return Err(String::from("invalid method"));
                }

                Ok(response)
            },
            Method::Count => {
                let response = handler.handle_count(store, &middle_res.0, &middle_res.1)?;

                response.check()?;

                if response.method() != Method::Count {
                    return Err(String::from("invalid method"));
                }

                Ok(response)
            },
            Method::List => {
                let response = handler.handle_list(store, &middle_res.0, &middle_res.1)?;

                response.check()?;

                if response.method() != Method::List {
                    return Err(String::from("invalid method"));
                }

                Ok(response)
            },
            Method::Lookup => {
                let response = handler.handle_lookup(store, &middle_res.0, &middle_res.1)?;

                response.check()?;

                if response.method() != Method::Lookup {
                    return Err(String::from("invalid method"));
                }

                Ok(response)
            },
            Method::Get => {
                let response = handler.handle_get(store, &middle_res.0, &middle_res.1)?;

                response.check()?;

                if response.method() != Method::Get {
                    return Err(String::from("invalid method"));
                }

                Ok(response)
            },
            Method::Create => {
                let response = handler.handle_create(store, &middle_res.0, &middle_res.1)?;

                response.check()?;

                if response.method() != Method::Create {
                    return Err(String::from("invalid method"));
                }

                Ok(response)
            },
            Method::Update => {
                let response = handler.handle_update(store, &middle_res.0, &middle_res.1)?;

                response.check()?;

                if response.method() != Method::Update {
                    return Err(String::from("invalid method"));
                }

                Ok(response)
            },
            Method::Upsert => {
                let response = handler.handle_upsert(store, &middle_res.0, &middle_res.1)?;

                response.check()?;

                if response.method() != Method::Upsert {
                    return Err(String::from("invalid method"));
                }

                Ok(response)
            },
            Method::Delete => {
                let response = handler.handle_delete(store, &middle_res.0, &middle_res.1)?;

                response.check()?;

                if response.method() != Method::Delete {
                    return Err(String::from("invalid method"));
                }

                Ok(response)
            },
            Method::Custom => {
                let response = handler.handle_custom(store, &middle_res.0, &middle_res.1)?;

                response.check()?;

                if response.method() != Method::Custom {
                    return Err(String::from("invalid method"));
                }

                Ok(response)
            },
        }
    }
}