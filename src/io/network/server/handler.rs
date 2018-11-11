//! # Handler
//!
//! `handler` is the module providing the trait implemented by the server handler.

use base::Result;
use base::size::ConstantSize;
use base::Serializable;
use base::Checkable;
use base::Datable;
use base::{Eval, EvalMut};
use io::store::Store;
use io::network::message::Request;
use io::network::message::Response;

/// Trait implemented by the server handler.
pub trait Handler<St, StS, S, D, P>
    where   St: Store<StS>,
            StS: Datable + Serializable,
            S: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable,
            Self: 'static + Sized + Clone + Send + Sync,
{
    /// Returns the middleware callbacks applied sequentially by the router. Each callback takes as parameters
    /// the results of the preceding one.
    fn middlewares(&mut self, _store: &mut St)
        -> Result<Vec<Box<FnMut(&mut Self, &mut St, &Request<S, D, P>)
                        -> Result<(Request<S, D, P>)>>>> 
    {
            Ok(vec![])
    }

    /// Handles a ping `Request`.
    fn handle_ping(&mut self,
                   store: &mut St,
                   request: &Request<S, D, P>)
        -> Result<Response<S, D, P>>;

    /// Handles a session `Request`.
    fn handle_session(&mut self,
                      store: &mut St,
                      request: &Request<S, D, P>)
        -> Result<Response<S, D, P>>;

    /// Handles a count `Request`.
    fn handle_count(&mut self,
                    store: &mut St,
                    request: &Request<S, D, P>)
        -> Result<Response<S, D, P>>;

    /// Handles a list `Request`.
    fn handle_list(&mut self,
                   store: &mut St,
                   request: &Request<S, D, P>)
        -> Result<Response<S, D, P>>;

    /// Handles a lookup `Request`.
    fn handle_lookup(&mut self,
                     store: &mut St,
                     request: &Request<S, D, P>)
        -> Result<Response<S, D, P>>;

    /// Handles a get `Request`.
    fn handle_get(&mut self,
                  store: &mut St,
                  request: &Request<S, D, P>)
        -> Result<Response<S, D, P>>;

    /// Handles a create `Request`.
    fn handle_create(&mut self,
                     store: &mut St,
                     request: &Request<S, D, P>)
        -> Result<Response<S, D, P>>;

    /// Handles an update `Request`.
    fn handle_update(&mut self,
                     store: &mut St,
                     request: &Request<S, D, P>)
        -> Result<Response<S, D, P>>;

    /// Handles an upsert `Request`.
    fn handle_upsert(&mut self,
                     store: &mut St,
                     request: &Request<S, D, P>)
        -> Result<Response<S, D, P>>;

    /// Handles a delete `Request`.
    fn handle_delete(&mut self,
                     store: &mut St,
                     request: &Request<S, D, P>)
        -> Result<Response<S, D, P>>;

    /// Handles an eval `Request`.
    fn handle_eval<Ev>(&mut self,
                       store: &mut St,
                       request: &Request<S, D, P>,
                       evaluator: &Ev)
        -> Result<Response<S, D, P>>
        where   Ev: Eval<St, Request<S, D, P>, Response<S, D, P>>
    {
        request.check()?;
        
        let response = evaluator.eval(store, request)?;
        response.check()?;

        Ok(response)
    }

    /// Handles an evalmut `Request`.
    fn handle_eval_mut<EvM>(&mut self,
                            store: &mut St,
                            request: &Request<S, D, P>,
                            evaluator: &mut EvM)
        -> Result<Response<S, D, P>>
        where   EvM: EvalMut<St, Request<S, D, P>, Response<S, D, P>>
    {
        request.check()?;
        
        let response = evaluator.eval_mut(store, request)?;
        response.check()?;

        Ok(response)
    }
}