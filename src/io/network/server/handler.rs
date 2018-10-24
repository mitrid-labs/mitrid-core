//! # Handler
//!
//! `handler` is the module providing the trait implemented by the server handler.

use base::Result;
use base::size::{ConstantSize, VariableSize};
use base::numerical::Numerical;
use base::Serializable;
use base::Datable;
use io::store::Store;
use io::network::message::Request;
use io::network::message::Response;

/// Trait implemented by the server handler.
pub trait Handler<St, StS, StK, StV, S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   St: Store<StS, StK, StV>,
            StS: Datable + Serializable,
            StK: Datable + Serializable,
            StV: Datable + Serializable,
            S: Datable,
            RS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Datable + ConstantSize,
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
            Self: Clone + Send + Sync
{
    /// Returns the middleware callbacks applied sequentially by the router. Each callback takes as parameters
    /// the results of the preceding one.
    fn middlewares<P: Datable>(&self, _store: &mut St, params: &P)
        -> Result<Vec<Box<Fn(&mut St, &P, &Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>)
                        -> Result<(P, Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>)>>>> 
    {
            params.check()?;

            Ok(vec![])
    }

    /// Handles a ping `Request`.
    fn handle_ping<P: Datable>(&self,
                               store: &mut St,
                               params: &P,
                               request: &Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>)
        -> Result<Response<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>>;

    /// Handles a session `Request`.
    fn handle_session<P: Datable>(&self,
                                  store: &mut St,
                                  params: &P,
                                  request: &Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>)
        -> Result<Response<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>>;

    /// Handles a count `Request`.
    fn handle_count<P: Datable>(&self,
                                store: &mut St,
                                params: &P,
                                request: &Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>)
        -> Result<Response<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>>;

    /// Handles a list `Request`.
    fn handle_list<P: Datable>(&self,
                               store: &mut St,
                               params: &P,
                               request: &Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>)
        -> Result<Response<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>>;

    /// Handles a lookup `Request`.
    fn handle_lookup<P: Datable>(&self,
                                 store: &mut St,
                                 params: &P,
                                 request: &Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>)
        -> Result<Response<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>>;

    /// Handles a get `Request`.
    fn handle_get<P: Datable>(&self,
                              store: &mut St,
                              params: &P,
                              request: &Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>)
        -> Result<Response<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>>;

    /// Handles a create `Request`.
    fn handle_create<P: Datable>(&self,
                                 store: &mut St,
                                 params: &P,
                                 request: &Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>)
        -> Result<Response<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>>;

    /// Handles an update `Request`.
    fn handle_update<P: Datable>(&self,
                                 store: &mut St,
                                 params: &P,
                                 request: &Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>)
        -> Result<Response<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>>;

    /// Handles an upsert `Request`.
    fn handle_upsert<P: Datable>(&self,
                                 store: &mut St,
                                 params: &P,
                                 request: &Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>)
        -> Result<Response<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>>;

    /// Handles a delete `Request`.
    fn handle_delete<P: Datable>(&self,
                                 store: &mut St,
                                 params: &P,
                                 request: &Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>)
        -> Result<Response<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>>;

    /// Handles a custom `Request`.
    fn handle_custom<P: Datable>(&self,
                                 store: &mut St,
                                 params: &P,
                                 request: &Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>)
        -> Result<Response<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>>;
}