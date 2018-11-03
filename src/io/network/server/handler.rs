//! # Handler
//!
//! `handler` is the module providing the trait implemented by the server handler.

use base::Result;
use base::size::{ConstantSize, VariableSize};
use base::Serializable;
use base::Datable;
use io::store::Store;
use io::network::message::Request;
use io::network::message::Response;

/// Trait implemented by the server handler.
pub trait Handler<St, StS, StK, StV, StP, StPC, StRC, S, Ad, NP, D, MP>
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
            Ad: Ord + Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            MP: Datable,
            Self: 'static + Clone + Send + Sync
{
    /// Returns the middleware callbacks applied sequentially by the router. Each callback takes as parameters
    /// the results of the preceding one.
    fn middlewares<P: Datable>(&self, _store: &mut St, params: &P)
        -> Result<Vec<Box<Fn(&mut St, &P, &Request<S, Ad, NP, D, MP>)
                        -> Result<(P, Request<S, Ad, NP, D, MP>)>>>> 
    {
            params.check()?;

            Ok(vec![])
    }

    /// Handles a ping `Request`.
    fn handle_ping<P: Datable>(&self,
                               store: &mut St,
                               params: &P,
                               request: &Request<S, Ad, NP, D, MP>)
        -> Result<Response<S, Ad, NP, D, MP>>;

    /// Handles a session `Request`.
    fn handle_session<P: Datable>(&self,
                                  store: &mut St,
                                  params: &P,
                                  request: &Request<S, Ad, NP, D, MP>)
        -> Result<Response<S, Ad, NP, D, MP>>;

    /// Handles a count `Request`.
    fn handle_count<P: Datable>(&self,
                                store: &mut St,
                                params: &P,
                                request: &Request<S, Ad, NP, D, MP>)
        -> Result<Response<S, Ad, NP, D, MP>>;

    /// Handles a list `Request`.
    fn handle_list<P: Datable>(&self,
                               store: &mut St,
                               params: &P,
                               request: &Request<S, Ad, NP, D, MP>)
        -> Result<Response<S, Ad, NP, D, MP>>;

    /// Handles a lookup `Request`.
    fn handle_lookup<P: Datable>(&self,
                                 store: &mut St,
                                 params: &P,
                                 request: &Request<S, Ad, NP, D, MP>)
        -> Result<Response<S, Ad, NP, D, MP>>;

    /// Handles a get `Request`.
    fn handle_get<P: Datable>(&self,
                              store: &mut St,
                              params: &P,
                              request: &Request<S, Ad, NP, D, MP>)
        -> Result<Response<S, Ad, NP, D, MP>>;

    /// Handles a create `Request`.
    fn handle_create<P: Datable>(&self,
                                 store: &mut St,
                                 params: &P,
                                 request: &Request<S, Ad, NP, D, MP>)
        -> Result<Response<S, Ad, NP, D, MP>>;

    /// Handles an update `Request`.
    fn handle_update<P: Datable>(&self,
                                 store: &mut St,
                                 params: &P,
                                 request: &Request<S, Ad, NP, D, MP>)
        -> Result<Response<S, Ad, NP, D, MP>>;

    /// Handles an upsert `Request`.
    fn handle_upsert<P: Datable>(&self,
                                 store: &mut St,
                                 params: &P,
                                 request: &Request<S, Ad, NP, D, MP>)
        -> Result<Response<S, Ad, NP, D, MP>>;

    /// Handles a delete `Request`.
    fn handle_delete<P: Datable>(&self,
                                 store: &mut St,
                                 params: &P,
                                 request: &Request<S, Ad, NP, D, MP>)
        -> Result<Response<S, Ad, NP, D, MP>>;

    /// Handles a custom `Request`.
    fn handle_custom<P: Datable>(&self,
                                 store: &mut St,
                                 params: &P,
                                 request: &Request<S, Ad, NP, D, MP>)
        -> Result<Response<S, Ad, NP, D, MP>>;
}